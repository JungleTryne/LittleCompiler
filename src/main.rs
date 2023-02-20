extern crate core;

extern crate pest;
extern crate pest_derive;

use crate::parser::parser::AssemblyParser;
use anyhow::Context;
use std::fs;

mod parser;

fn main() -> anyhow::Result<()> {
    let source = fs::read_to_string("examples/fibonacci.las").context("Cannot read file")?;
    let parsing_result = AssemblyParser::get_ast(&source).context("Parsing error")?;
    todo!("implement compiler");

    Ok(())
}
