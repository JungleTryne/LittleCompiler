extern crate core;

extern crate pest;
extern crate pest_derive;

use crate::frontend::parser::AssemblyParser;
use anyhow::Context;
use std::fs;

use clap;
use clap::Parser;

mod backend;
mod frontend;

#[derive(clap::Parser)]
struct Cli {
    image_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let source = fs::read_to_string(&args.image_path).context("Cannot read file")?;
    let parsing_result = AssemblyParser::get_ast(&source).context("Parsing error")?;

    todo!("implement compiler");
    Ok(())
}
