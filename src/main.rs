extern crate core;

extern crate pest;
extern crate pest_derive;

use crate::frontend::parser::AssemblyParser;
use anyhow::Context;
use std::fs;
use std::io::Write;

use crate::backend::compiler::compile;
use clap::Parser;

mod backend;
mod frontend;

#[derive(clap::Parser)]
struct Cli {
    image_path: std::path::PathBuf,
    bin_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let source = fs::read_to_string(args.image_path).context("Cannot read file")?;
    let ast = AssemblyParser::get_ast(&source).context("Parsing error")?;
    let bytes = compile(ast);

    let mut bin_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(&args.bin_path)
        .context("Couldn't create bin file")?;

    bin_file
        .write_all(&bytes)
        .context("Couldn't save the compiled program into file")?;

    Ok(())
}
