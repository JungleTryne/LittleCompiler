use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{Node, ProgramLine};

pub const MEMORY_START: u32 = 0x30;

pub struct Compiler {
    data_storage: DataStorage
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            data_storage: DataStorage::default(),
        }
    }
}

impl Compiler {
    pub fn compile(&mut self, ast: Vec<Node>) -> Vec<u8> {
        unimplemented!()
    }

    fn compile_instruction(&mut self, program_line: ProgramLine) -> Vec<u8> {
        unimplemented!()
    }
}
