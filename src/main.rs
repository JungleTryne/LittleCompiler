extern crate pest;

#[macro_use]
extern crate pest_derive;
extern crate core;

use crate::parser::parser::AssemblyParser;
use std::fs;

mod parser;

fn main() -> anyhow::Result<()> {
    let source = fs::read_to_string("examples/echo.las").expect("Cannot read file");
    let parsing_result = AssemblyParser::get_ast(&source)?;

    Ok(())
}
