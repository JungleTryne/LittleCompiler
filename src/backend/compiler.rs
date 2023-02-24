use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{DataLine, Node, ProgramLine};

pub const MEMORY_START: u32 = 0x30;

pub struct CompilerBuilder {}

impl Default for CompilerBuilder {
    fn default() -> Self {
        CompilerBuilder {}
    }
}

impl CompilerBuilder {
    pub fn build(self, ast: Vec<Node>) -> Compiler {
        let (data_lines, program_lines): (Vec<Option<DataLine>>, Vec<Option<ProgramLine>>) = ast
            .into_iter()
            .map(|value| match value {
                Node::DataLine(data_line) => (Some(data_line), None),
                Node::ProgramLine(program_line) => (None, Some(program_line)),
            })
            .unzip();

        let data_lines: Vec<DataLine> = data_lines.into_iter().flatten().collect();
        let program_lines = program_lines.into_iter().flatten().collect();

        Compiler {
            data_storage: DataStorage::from(data_lines),
            program_lines,
        }
    }
}

pub struct Compiler {
    data_storage: DataStorage,
    program_lines: Vec<ProgramLine>,
}

impl Compiler {
    pub fn compile(self) -> Vec<u8> {
        unimplemented!()
    }

    fn compile_instruction(&mut self, program_line: ProgramLine) -> Vec<u8> {
        unimplemented!()
    }
}
