use crate::backend::compiler::MEMORY_START;
use crate::frontend::ast::{DataLine, DataValue};
use anyhow::Context;
use std::collections::HashMap;

struct DataCellBuf {
    address: u32,
    value: Vec<u8>,
}

pub struct DataStorage {
    data_cells: HashMap<String, DataCellBuf>,
    current_free_address: u32,
}

impl Default for DataStorage {
    fn default() -> Self {
        DataStorage {
            data_cells: HashMap::new(),
            current_free_address: MEMORY_START,
        }
    }
}

impl DataStorage {
    pub fn push_data_line(&mut self, data_line: DataLine) {
        let compiled_value = self.compile_data_value(data_line.value);
        let value_size = compiled_value.len() as u32;
        let address = self.current_free_address;

        self.data_cells
            .insert(
                data_line.var_name,
                DataCellBuf {
                    address,
                    value: compiled_value,
                },
            )
            .unwrap();

        self.current_free_address += value_size;
    }

    pub fn compile_data_storage(self) -> Vec<u8> {
        let mut memory = vec![0; self.current_free_address as usize];

        for (_, data_cell) in self.data_cells.into_iter() {
            memory[data_cell.address as usize..].copy_from_slice(&data_cell.value)
        }

        memory
    }

    pub fn get_var_address(&self, var_name: &str) -> u32 {
        self.data_cells
            .get(var_name)
            .context("There is not such variable")
            .unwrap()
            .address
    }

    fn compile_data_value(&self, data_value: DataValue) -> Vec<u8> {
        match data_value {
            DataValue::Str(str) => str.into_bytes(),
        }
    }
}
