use crate::frontend::ast::{
    DataLine, DataValue, Instruction, InstructionArgument, InstructionLine, Node, ProgramLine,
    Register,
};
use anyhow::{anyhow, Context};
use pest::iterators::{Pair, Pairs};
use pest::{self, Parser};
use std::str::FromStr;

#[derive(pest_derive::Parser)]
#[grammar = "frontend/grammar.pest"]
pub struct AssemblyParser;

impl AssemblyParser {
    pub fn get_ast(source: &str) -> anyhow::Result<Vec<Node>> {
        AssemblyParser::parse(Rule::program, &source)
            .context("unsuccessful parse")?
            .filter_map(|node| match node.as_rule() {
                Rule::data_line => Some(AssemblyParser::parse_data_line(node.into_inner())),
                Rule::program_line => Some(AssemblyParser::parse_program_line(node.into_inner())),
                Rule::mark => Some(AssemblyParser::parse_mark(node)),
                Rule::EOI => None,
                _ => Some(Err(anyhow!(format!(
                    "Unexpected rule in get_ast {}",
                    node.as_str()
                )))),
            })
            .collect()
    }

    fn parse_data_line(mut data_line: Pairs<Rule>) -> anyhow::Result<Node> {
        let var_name = data_line
            .next()
            .context("Couldn't extract data name")?
            .as_str()
            .to_owned();

        let var_value_iter = data_line.next().context("Couldn't extract data value")?;

        let var_value = match var_value_iter.as_rule() {
            Rule::string_value => DataValue::Str(var_value_iter.as_str().to_owned()),
            _ => {
                return Err(anyhow!(format!(
                    "Unexpected rule in parse_data_line {}",
                    var_value_iter.as_str()
                )))
            }
        };

        Ok(Node::DataLine(DataLine {
            var_name,
            value: var_value,
        }))
    }

    fn parse_mark(mark: Pair<Rule>) -> anyhow::Result<Node> {
        Ok(Node::ProgramLine(ProgramLine::Mark(
            mark.as_str().to_owned(),
        )))
    }

    fn parse_program_line(mut program_line: Pairs<Rule>) -> anyhow::Result<Node> {
        let instruction = program_line
            .next()
            .context("Couldn't fetch instruction")?
            .as_str();

        let instruction = Instruction::from_str(instruction)?;

        let instruction_args: anyhow::Result<Vec<InstructionArgument>> = program_line
            .map(|pair| match pair.as_rule() {
                Rule::instruction_arg => {
                    AssemblyParser::parse_instruction_argument(pair.into_inner())
                }
                _ => Err(anyhow!(format!(
                    "Unexpected rule in parse_program_line {}",
                    pair.as_str()
                ))),
            })
            .collect();

        let instruction_args = instruction_args?;

        Ok(Node::ProgramLine(ProgramLine::InstructionLine(
            InstructionLine {
                instruction,
                args: instruction_args,
            },
        )))
    }

    fn parse_instruction_argument(
        mut instruction_arg: Pairs<Rule>,
    ) -> anyhow::Result<InstructionArgument> {
        let argument = instruction_arg
            .next()
            .context("Couldn't fetch instruction argument")?;

        let instruction_argument = match argument.as_rule() {
            Rule::identifier => InstructionArgument::Identifier(argument.as_str().to_owned()),
            Rule::number => InstructionArgument::Number(
                argument
                    .as_str()
                    .parse::<i16>()
                    .context("Couldn't parse instruction argument i16")?,
            ),
            Rule::register => InstructionArgument::Register(
                Register::from_str(argument.as_str())
                    .context("Couldn't parse instruction argument register")?,
            ),
            Rule::mark => InstructionArgument::Mark(argument.as_str().to_owned()),
            _ => Err(anyhow!(format!(
                "Unexpected rule in parse_instruction_argument {}",
                argument.as_str()
            )))?,
        };

        Ok(instruction_argument)
    }
}
