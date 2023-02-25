use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{DataLine, Instruction, InstructionLine, Node, ProgramLine};
use std::collections::HashMap;

pub const MEMORY_START: u32 = 0x30;
pub const ARCH_BYTES: u32 = 4;

pub fn compile(ast: Vec<Node>) -> Vec<u8> {
    let (data_lines, program_lines) = split_data_program_lines(ast);
    let data_storage = DataStorage::from(data_lines);

    let initial_ip_value = data_storage.size() as u32;
    let (markers_storage, instructions) = scan_markers(initial_ip_value, program_lines);

    unimplemented!();
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
) -> (HashMap<String, u32>, Vec<InstructionLine>) {
    let mut markers = HashMap::new();
    let instruction_lines = program_lines
        .into_iter()
        .scan(initial_ip_value, |ip_value, line| {
            Some(match line {
                ProgramLine::Mark(mark) => {
                    markers.insert(mark, *ip_value as u32);
                    InstructionLine {
                        instruction: Instruction::SKIP,
                        args: vec![],
                    }
                }
                ProgramLine::InstructionLine(instruction_line) => instruction_line,
            })
        })
        .collect();

    (markers, instruction_lines)
}
