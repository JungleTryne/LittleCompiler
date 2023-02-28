use crate::frontend::util_macros::create_parser;
use strum_macros::EnumString;

create_parser! {
    Instruction,
    ADD, "add";
    SUB, "sub";
    MUL, "mul";
    DIV, "div";
    JMP, "jmp";
    LD, "ld";
    FIN, "fin";
    OUT, "out";
    EQ, "eq";
    L, "l";
    LE, "le";
    LDA, "lda";
    INP, "inp";
    JCMP, "jcmp";
    JNCMP, "jncmp";
    OUTR, "outr";
    SKIP, "skip";
    OUTN, "outn";
    MOV, "mov";
    INPN, "inpn"
}

create_parser! {
    Register,
    R0, "r0";
    R1, "r1";
    R2, "r2";
    R3, "r3"
}

pub enum Node {
    ProgramLine(ProgramLine),
    DataLine(DataLine),
}

pub enum DataValue {
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
    pub instruction: Instruction,
    pub args: Vec<InstructionArgument>,
}

pub enum InstructionArgument {
    Identifier(String),
    SignedNumber(i16),
    UnsignedNumber(u16),
    Register(Register),
    Mark(String),
}
