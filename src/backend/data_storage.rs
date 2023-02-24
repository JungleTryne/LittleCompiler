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
    size: usize,
}

impl From<Vec<DataLine>> for DataStorage {
    fn from(data_lines: Vec<DataLine>) -> Self {
        let mut size = 0 as usize;

        let data_cells: HashMap<_, _> = data_lines
            .into_iter()
            .map(|value| (value.var_name, DataStorage::compile_data_value(value.value)))
            .scan(MEMORY_START, |address, pair| {
                let new_pair = Some((
                    pair.0,
                    DataCellBuf {
                        address: *address,
                        value: pair.1,
                    },
                ));

                *address += pair.1.len() as u32;
                size = *address as usize;
                new_pair
            })
            .collect();

        DataStorage { data_cells, size }
    }
}

impl DataStorage {
    fn compile_data_value(value: DataValue) -> Vec<u8> {
        match value {
            DataValue::Str(str) => str.into_bytes(),
        }
    }

    pub fn compile_data_storage(self) -> Vec<u8> {
        let mut memory = vec![0; self.size];

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
}
