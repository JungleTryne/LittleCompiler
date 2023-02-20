use anyhow::{anyhow, Context};
use pest::iterators::Pairs;
use pest::Parser;

pub enum Node {
    ProgramLine(ProgramLine),
    DataLine(DataLine),
}

pub enum DataValue {
    Number(i16),
    Str(String),
}

pub struct DataLine {
    pub var_name: String,
    pub value: DataValue,
}

pub enum ProgramLine {
    Mark(String),
    InstructionLine(InstructionLine),
}

pub struct InstructionLine {
    instruction: Instruction,
    args: Vec<InstructionArguments>,
}

pub enum InstructionArguments {

}

pub enum Instruction {
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    LD,
    FIN,
    OUT,
    EQ,
    L,
    LE,
    LDA,
    INP,
    JCMP,
    JNCMP,
    OUTR,
}

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct AssemblyParser;

impl AssemblyParser {
    pub fn get_ast(source: &str) -> anyhow::Result<Vec<Node>> {
        AssemblyParser::parse(Rule::program, &source)
            .expect("unsuccessful parse")
            .map(|node| match node.as_rule() {
                Rule::data_line => AssemblyParser::parse_data_line(node.into_inner()),
                Rule::program_line => AssemblyParser::parse_program_line(node.into_inner()),
                _ => Err(anyhow!(format!("Unexpected rule {}", node.as_str()))),
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
            Rule::number => DataValue::Number(
                var_value_iter
                    .as_str()
                    .parse::<i16>()
                    .context("Couldn't parse data value i16")?,
            ),

            Rule::string_value => DataValue::Str(var_value_iter.as_str().to_owned()),
            _ => {
                return Err(anyhow!(format!(
                    "Unexpected rule {}",
                    var_value_iter.as_str()
                )))
            }
        };

        Ok(Node::DataLine(DataLine {
            var_name,
            value: var_value,
        }))
    }

    fn parse_program_line(mut program_line: Pairs<Rule>) -> anyhow::Result<Node> {
        unimplemented!()
    }
}
