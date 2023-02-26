use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{
    DataLine, Instruction, InstructionArgument, InstructionLine, Node, ProgramLine, Register,
};
use anyhow::Context;
use byteorder::{ByteOrder, LittleEndian};
use num::ToPrimitive;
use std::collections::HashMap;

pub const MEMORY_START: u32 = 0x30;
pub const ARCH_BYTES: u32 = 4;

type MarkerStorage = HashMap<String, u32>;

pub fn compile(ast: Vec<Node>) -> Vec<u8> {
    let (data_lines, program_lines) = split_data_program_lines(ast);
    let data_storage = DataStorage::from(data_lines);

    let initial_ip_value = data_storage.size() as u32;
    let (markers_storage, instructions) = scan_markers(initial_ip_value, program_lines);

    let instruction_line_compiler = InstructionLineCompiler::new(&data_storage, &markers_storage);

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
            Some(match line {
                ProgramLine::Mark(mark) => {
                    markers.insert(mark, *ip_value);
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

struct InstructionLineCompiler<'a> {
    data_storage: &'a DataStorage,
    marker_storage: &'a MarkerStorage,
}

impl<'a> InstructionLineCompiler<'a> {
    pub fn new(data_storage: &'a DataStorage, marker_storage: &'a MarkerStorage) -> Self {
        InstructionLineCompiler {
            data_storage,
            marker_storage,
        }
    }

    pub fn compile(&self, current_ip: u32, instruction_line: InstructionLine) -> Vec<u8> {
        let mut code = vec![0; ARCH_BYTES as usize];
        code[0] = get_code(instruction_line.instruction);

        let args: Vec<u8> = instruction_line
            .args
            .into_iter()
            .flat_map(|arg| match arg {
                InstructionArgument::UnsignedNumber(num) => {
                    let mut num_bytes = vec![0; 2];
                    LittleEndian::write_u16(&mut num_bytes, num);
                    num_bytes
                }
                InstructionArgument::SignedNumber(num) => {
                    let mut num_bytes = vec![0; 2];
                    LittleEndian::write_i16(&mut num_bytes, num);
                    num_bytes
                }
                InstructionArgument::Mark(mark_name) => {
                    let mark_addr = *self.marker_storage.get(&mark_name).unwrap();
                    let offset = self.get_offset(current_ip, mark_addr);

                    let mut num_bytes = vec![0; 2];
                    LittleEndian::write_i16(&mut num_bytes, offset);
                    num_bytes
                }
                InstructionArgument::Register(register) => match register {
                    Register::R0 => vec![0x4],
                    Register::R1 => vec![0x8],
                    Register::R2 => vec![0xC],
                    Register::R3 => vec![0x10],
                },
                InstructionArgument::Identifier(identifier) => {
                    let data_addr = self.data_storage.get_var_address(&identifier);
                    let offset = self.get_offset(current_ip, data_addr);

                    let mut num_bytes = vec![0; 2];
                    LittleEndian::write_i16(&mut num_bytes, offset);
                    num_bytes
                }
            })
            .collect();

        assert!(args.len() < ARCH_BYTES as usize, "Too many arguments");
        for (i, byte) in args.iter().enumerate() {
            code[i + 1] = *byte;
        }

        code
    }

    fn get_offset(&self, addr_from: u32, addr_to: u32) -> i16 {
        let (offset, overflow) = addr_to.overflowing_sub(addr_from);
        let offset: i16 = if !overflow {
            offset
                .to_i16()
                .context("Couldn't cast mark offset to i16")
                .unwrap()
        } else {
            let offset_abs = (u32::MAX - offset)
                .to_i16()
                .context("Couldn't cast mark offset to i16")
                .unwrap();
            -offset_abs
        };
        offset
    }
}

fn get_code(instruction: Instruction) -> u8 {
    match instruction {
        Instruction::ADD => 0x01,
        Instruction::SUB => 0x02,
        Instruction::MUL => 0x03,
        Instruction::DIV => 0x04,
        Instruction::JMP => 0x05,
        Instruction::LD => 0x06,
        Instruction::FIN => 0x07,
        Instruction::OUT => 0x08,
        Instruction::EQ => 0x09,
        Instruction::L => 0x0A,
        Instruction::LE => 0x0B,
        Instruction::LDA => 0x0C,
        Instruction::INP => 0x0D,
        Instruction::JCMP => 0x0E,
        Instruction::JNCMP => 0x0F,
        Instruction::OUTR => 0x10,
        Instruction::SKIP => 0x11,
        Instruction::OUTN => 0x12,
        Instruction::MOV => 0x13,
    }
}
