use crate::backend::compiler::{MarkerStorage, ARCH_BYTES};
use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{Instruction, InstructionArgument, InstructionLine, Register};
use anyhow::Context;
use byteorder::{ByteOrder, LittleEndian};
use num::ToPrimitive;


pub struct InstructionLineCompiler<'a> {
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
                InstructionArgument::Register(register) => {
                    let addr = get_register_address(register);
                    vec![addr]
                }
                InstructionArgument::Identifier(identifier) => {
                    let data_addr = self.data_storage.get_var_address(&identifier);
                    let data_addr = data_addr
                        .to_u16()
                        .context("Couldn't cast data address to u16")
                        .unwrap();

                    let mut num_bytes = vec![0; 2];
                    LittleEndian::write_u16(&mut num_bytes, data_addr);
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
            let (offset_abs, overflow) = addr_from.overflowing_sub(addr_to);
            assert!(!overflow);

            let offset_abs = offset_abs
                .to_i16()
                .context("Couldn't cast mark offset to i16")
                .unwrap();
            -offset_abs
        };
        offset
    }
}

fn get_register_address(register: Register) -> u8 {
    #[allow(clippy::identity_op)]
    let addr = match register {
        Register::R0 => 1 * ARCH_BYTES,
        Register::R1 => 2 * ARCH_BYTES,
        Register::R2 => 3 * ARCH_BYTES,
        Register::R3 => 4 * ARCH_BYTES,
        Register::SP => 7 * ARCH_BYTES,
    };
    addr as u8
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
        Instruction::INPN => 0x14,
        Instruction::PUSH => 0x15,
        Instruction::POP => 0x16,
        Instruction::CALL => 0x17,
        Instruction::RET => 0x18,
    }
}
