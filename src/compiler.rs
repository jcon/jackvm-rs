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
    Arithmetic(Operator),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompilationError {
    message: &'static str,
    line_number: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction<'a> {
    line_number: i32,
    command_type: Option<&'a str>,
    arg1: Option<&'a str>,
    arg2: Option<&'a str>,
}

#[allow(dead_code)]
pub struct Parser<'a> {
    position: i32,
    source: Vec<&'a str>,
    current_instruction: Option<Instruction<'a>>,
}

// use std::fmt;

// impl fmt::Display for Vec<&str> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

impl<'a> Parser<'a> {
    pub fn new(source: &str) -> Parser {
        let mut lines = Vec::new();
        for line in source.lines() {
            lines.push(line);
        }
        Parser {
            position: -1,
            source: lines,
            current_instruction: None,
        }
    }

    pub fn advance(&mut self) -> () {
        self.position = self.position + 1;
        let pos = self.position as usize;
        if pos < self.source.len() {
            let mut parts = self.source[pos].split(' ');
            self.current_instruction = Some(Instruction {
                line_number: self.position + 1,
                command_type: parts.next(),
                arg1: parts.next(),
                arg2: parts.next(),
            });
            // println!("parts are {:?}", self.current_instruction);
        } else {
            self.current_instruction = None;
        }
    }

    pub fn has_more_commands(&self) -> bool {
        let next_pos = (self.position + 1) as usize;
        next_pos < self.source.len()
    }

    pub fn get_instruction(&self) -> Option<Instruction<'a>> {
        self.current_instruction
    }

    pub fn get_line_number(&self) -> i32 {
        self.position + 1
    }

    pub fn get_command_type(&self) -> Option<&str> {
        self.current_instruction?.command_type
    }

    pub fn get_arg1(&self) -> Option<&str> {
        self.current_instruction?.arg1
    }

    pub fn get_arg2(&self) -> Option<&str> {
        self.current_instruction?.arg2
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Instruction<'a>> {
        if self.has_more_commands() {
            self.advance();
            self.get_instruction()
        } else {
            None
        }
    }
}

fn parse_push(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a memory segment for push",
    })?;
    let arg2 = match instruction.arg2 {
        Some(a2) => a2.parse::<i32>().map_err(|_| CompilationError {
            line_number,
            message: "Expected a positive integer for argument",
        }),
        None => Err(CompilationError {
            line_number,
            message: "Expected a second argement for push",
        })
    }?;
    let segment = match arg1 {
        "constant" => Ok(Segment::CONSTANT),
        "argument" => Ok(Segment::ARG),
        "local" => Ok(Segment::LOCAL),
        "this" => Ok(Segment::THIS),
        "that" => Ok(Segment::THAT),
        "pointer" => Ok(Segment::POINTER),
        "temp" => Ok(Segment::TEMP),
        "static" => Ok(Segment::STATIC),
        _ => Err(CompilationError {
            line_number,
            message: "Unexpected segment for push",
        })
    }?;
    Ok(Command::Push(segment, arg2))
}

fn parse_arithmetic(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let operation = match instruction.command_type {
        Some("add") => Ok(Operator::ADD),
        Some("sub") => Ok(Operator::SUB),
        Some("eq") => Ok(Operator::EQ),
        Some("gt") => Ok(Operator::GT),
        Some("lt") => Ok(Operator::LT),
        Some("and") => Ok(Operator::AND),
        Some("or") => Ok(Operator::OR),
        Some("neg") => Ok(Operator::NEG),
        Some("not") => Ok(Operator::NOT),
        _ => Err(CompilationError {
            line_number,
            message: "Unexpected arithmetic operation",
        })
    }?;
    Ok(Command::Arithmetic(operation))
}

// TODO: find out how to set a statically defined set
fn is_arithmetic(command_type: &str) -> bool {
    let ct = command_type;
    ct == "add" || ct == "sub" || ct == "eq" || ct == "gt" || ct == "lt" || ct == "and" || ct == "or" || ct == "neg" || ct == "not"
}

pub fn compile(source: &str) -> Result<Vec<Command>, Vec<CompilationError>> {
    let mut parser = Parser::new(source);
    let mut program: Vec<Command> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();

    while parser.has_more_commands() {
        parser.advance();
        let ins = parser.get_instruction();
        let line_number = parser.get_line_number();
        let command_or_error = match parser.get_command_type() {
            Some("push") => parse_push(&ins.unwrap()),
            Some(ct) if is_arithmetic(ct) => parse_arithmetic(&ins.unwrap()),
            _ => Err(CompilationError {
                line_number,
                message: "Unrecognized instruction",
            }),
        };
        match command_or_error {
            Ok(command) => program.push(command),
            Err(e) => errors.push(e),
        }
    }

    if errors.len() == 0 {
        Ok(program)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_segment_match() {
        let ct = Command::Push(Segment::LOCAL, 1);
        assert_matches!(ct, Command::Push(Segment::LOCAL, 1));
    }

    #[test]
    fn test_parse_arithmetic_valid_operations() {
        for (text, op) in &[
            ("add", Operator::ADD),
            ("sub", Operator::SUB),
            ("eq", Operator::EQ),
            ("lt", Operator::LT),
            ("gt", Operator::GT),
            ("and", Operator::AND),
            ("or", Operator::OR),
            ("neg", Operator::NEG),
            ("not", Operator::NOT),
        ] {
            let ins = Instruction {
                line_number: 3,
                command_type: Some(text),
                arg1: None,
                arg2: None,
            };
            assert_eq!(parse_arithmetic(&ins), Ok(Command::Arithmetic(*op)));
        }
    }

    #[test]
    fn test_parse_arithmetic_invalid_operations() {
        for text in &["foo", "bar", "baz"] {
            let ins = Instruction {
                line_number: 3,
                command_type: Some(text),
                arg1: None,
                arg2: None,
            };
            assert_eq!(parse_arithmetic(&ins), Err(CompilationError{
                line_number: 3,
                message: "Unexpected arithmetic operation"
            }));
        }
    }

    #[test]
    fn test_parse_push_with_invalid_segment() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("push"),
            arg1: Some("invalid"),
            arg2: Some("10"),
        };
        assert_matches!(parse_push(&ins),
            Err(CompilationError{ line_number: 3, message: "Unexpected segment for push" }));
    }

    #[test]
    fn test_parse_push_with_no_arg2() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("push"),
            arg1: Some("doesntmatter"),
            arg2: None,
        };
        assert_matches!(parse_push(&ins),
            Err(CompilationError{ line_number: 8, message: "Expected a second argement for push"}));
    }

    #[test]
    fn test_parse_push_with_no_arg1() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("doesntmatter"),
            arg1: None,
            arg2: None,
        };
        assert_matches!(parse_push(&ins),
            Err(CompilationError{ line_number: 8, message: "Expected a memory segment for push" }));
    }

    #[test]
    fn test_simple_add() {
        let source = "push constant 5
push constant 4
add";
        let prog = compile(&source[..]);
        assert_eq!(Ok(vec!(
            Command::Push(Segment::CONSTANT, 5),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::ADD),
        )), prog);
    }

    #[test]
    fn test_compile_simple_inccorect_program() {
        let source = "foo constant 5";
        let prog = compile(&source[..]);
        assert_eq!(Err(vec!(CompilationError {
            line_number: 1,
            message: "Unrecognized instruction",
        })), prog);
    }
}
