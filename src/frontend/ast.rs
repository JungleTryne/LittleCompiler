use crate::frontend::util_macros::create_parser;
use strum_macros::EnumString;

create_parser! {
    Instruction,
    ADD, "add", 0x1;
    SUB, "sub", 0x2;
    MUL, "mul", 0x3;
    DIV, "div", 0x4;
    JMP, "jmp", 0x5;
    LD, "ld", 0x6;
    FIN, "fin", 0x7;
    OUT, "out", 0x8;
    EQ, "eq", 0x9;
    L, "l", 0xA;
    LE, "le", 0xB;
    LDA, "lda", 0xC;
    INP, "inp", 0xD;
    JCMP, "jcmp", 0xE;
    JNCMP, "jncmp", 0xF;
    OUTR, "outr", 0x10;
    SKIP, "skip", 0x11;
    OUTRN, "outn", 0x12;
    MOV, "mov", 0x13
}

create_parser! {
    Register,
    R0, "r0", 0x4;
    R1, "r1", 0x8;
    R2, "r2", 0xC;
    R3, "r3", 0x10
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
    Number(i16),
    Register(Register),
    Mark(String),
}
