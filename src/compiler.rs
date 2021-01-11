#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    LOCAL,
    ARG,
    THIS,
    THAT,
    POINTER,
    TEMP,
    STATIC,
    CONSTANT,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    ADD,
    SUB,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NEG,
    NOT,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Push(Segment, i32),
    Pop(Segment, i32),
    Operate(Operator),
}

#[allow(dead_code)]
pub struct Parser {
    position: u32,
    // sourceLine: &str;
    // currentLine: &str;
}

impl Parser {
    // pub fn hasMoreCommands(&self) -> bool {
    //     false
    // }

    // pub fn getCurrentLine(&self) -> &str {
    //     self.currentLine
    // }
    #[allow(dead_code)]
    pub fn foo() -> bool {
        true
    }
}

pub fn compile(source: &str) -> &[Command] {
    &[
        Command::Push(Segment::CONSTANT, 5),
        Command::Push(Segment::CONSTANT, 4),
        Command::Operate(Operator::ADD),
    ]
}
