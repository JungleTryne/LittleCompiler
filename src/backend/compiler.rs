use crate::backend::data_storage::DataStorage;
use crate::backend::instruction_line::InstructionLineCompiler;
use crate::frontend::ast::{DataLine, Instruction, InstructionLine, Node, ProgramLine};
use byteorder::{ByteOrder, LittleEndian};
use std::collections::HashMap;

pub const MEMORY_START: u32 = 0x30;
pub const ARCH_BYTES: u32 = 4;
pub const STACK_MEMORY_SIZE: usize = 2048;
pub const SP_MEMORY_ADDR: usize = 28;

pub type MarkerStorage = HashMap<String, u32>;

pub fn compile(ast: Vec<Node>) -> Vec<u8> {
    let (data_lines, program_lines) = split_data_program_lines(ast);
    let data_storage = DataStorage::from(data_lines);

    let initial_ip_value = data_storage.size() as u32;
    let (markers_storage, instructions) = scan_markers(initial_ip_value, program_lines);

    let instruction_line_compiler = InstructionLineCompiler::new(&data_storage, &markers_storage);
    debug_assert_eq!(initial_ip_value % ARCH_BYTES, 0);

    let codes: Vec<u8> = instructions
        .into_iter()
        .scan(initial_ip_value, |current_ip, instruction_line| {
            let result = Some(instruction_line_compiler.compile(*current_ip, instruction_line));
            *current_ip += ARCH_BYTES;
            result
        })
        .flatten()
        .collect();

    let mut program = vec![data_storage.compile_data_storage(), codes].concat();

    let mut ip_value_bytes = vec![0; 4];
    LittleEndian::write_u32(&mut ip_value_bytes, initial_ip_value);
    for (i, byte) in ip_value_bytes.into_iter().enumerate() {
        program[i] = byte;
    }

    let sp_value = program.len() as u32;
    let mut sp_value_bytes = vec![0; 4];
    LittleEndian::write_u32(&mut sp_value_bytes, sp_value);
    for (i, byte) in sp_value_bytes.into_iter().enumerate() {
        program[i + SP_MEMORY_ADDR] = byte;
    }

    let stack_memory = vec![0; STACK_MEMORY_SIZE];
    program.extend_from_slice(&stack_memory);

    program
}

fn split_data_program_lines(ast: Vec<Node>) -> (Vec<DataLine>, Vec<ProgramLine>) {
    let (data_lines, program_lines): (Vec<_>, Vec<_>) = ast
        .into_iter()
        .map(|value| match value {
            Node::DataLine(data_line) => (Some(data_line), None),
            Node::ProgramLine(program_line) => (None, Some(program_line)),
        })
        .unzip();

    let data_lines = data_lines.into_iter().flatten().collect();
    let program_lines = program_lines.into_iter().flatten().collect();

    (data_lines, program_lines)
}

fn scan_markers(
    initial_ip_value: u32,
    program_lines: Vec<ProgramLine>,
) -> (MarkerStorage, Vec<InstructionLine>) {
    let mut markers = HashMap::new();
    let instruction_lines = program_lines
        .into_iter()
        .scan(initial_ip_value, |ip_value, line| {
            let result = Some(match line {
                ProgramLine::Mark(mark) => {
                    markers.insert(mark, *ip_value);
                    InstructionLine {
                        instruction: Instruction::SKIP,
                        args: vec![],
                    }
                }
                ProgramLine::InstructionLine(instruction_line) => instruction_line,
            });
            *ip_value += ARCH_BYTES;
            result
        })
        .collect();

    (markers, instruction_lines)
}
