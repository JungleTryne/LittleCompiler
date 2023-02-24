use std::collections::HashMap;
use crate::backend::data_storage::DataStorage;
use crate::frontend::ast::{DataLine, InstructionLine, Node, ProgramLine};

pub const MEMORY_START: u32 = 0x30;
pub const ARCH_BYTES: u32 = 4;

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
            markers: HashMap::new(),
        }
    }
}

pub struct Compiler {
    data_storage: DataStorage,
    program_lines: Vec<ProgramLine>,
    markers: HashMap<String, u32>,
}

impl Compiler {
    pub fn compile(mut self) -> Vec<u8> {
        let ip_value = self.data_storage.size();
        assert_eq!(ip_value % 4, 0);

        for line in self.program_lines {
            match line {
                ProgramLine::Mark(mark) => {
                    self.markers.insert(mark, ip_value as u32);
                },
                ProgramLine::InstructionLine(instruction_line) => {

                },
            }
        }

        unimplemented!()
    }

    fn compile_instruction(&mut self, program_line: InstructionLine) -> Vec<u8> {
        unimplemented!()
    }
}
