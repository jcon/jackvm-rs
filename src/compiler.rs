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

    pub fn get_instruction(&self) -> Option<Instruction> {
        self.current_instruction
    }

    pub fn get_line_number(&self) -> i32 {
        self.position
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

fn parse_push(instruction: &Instruction) -> Result<Command, CompilationError> {
    let arg1 = match instruction.arg1 {
        Some(a1) => Ok(a1),
        None => Err(CompilationError {
            line_number: instruction.line_number,
            message: "Expected a memory segment for push",
        })
    }?;
    let arg2 = match instruction.arg2 { // parser.get_arg2().unwrap().parse::<i32>().unwrap
        Some(a2) => a2.parse::<i32>().map_err(|_| CompilationError {
            line_number: instruction.line_number,
            message: "Expected a positive integer for argument",
        }),
        None => Err(CompilationError {
            line_number: instruction.line_number,
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
            line_number: instruction.line_number,
            message: "Unexpected segment for push",
        })
    }?;
    Ok(Command::Push(segment, arg2))
}

fn parse_operator(parser: &Parser) -> Result<Command, CompilationError> {
    match parser.get_command_type() {
        Some("add") => Ok(Command::Operate(Operator::ADD)),
        // TODO: fix
        _ => Ok(Command::Push(Segment::ARG, 2))
    }
}

pub fn compile(source: &str) -> Result<Vec<Command>, CompilationError> {
    let mut parser = Parser::new(source);
    let mut program: Vec<Command> = Vec::new();

    while parser.has_more_commands() {
        parser.advance();
        let ins = parser.get_instruction();
        // println!("next command type is {:?}", parser.get_command_type());
        let command = match parser.get_command_type() {
            Some("push") => parse_push(&ins.unwrap()),
            Some("add") => parse_operator(&parser),
            // todo fix
            Some(x) => {
                println!("got {}", x);
                Ok(Command::Push(Segment::ARG, 1))
            },
            None => {
                println!("got NONE");
                Ok(Command::Push(Segment::ARG, 1))
            },
        }.unwrap();
        program.push(command);
    }

    Ok(program)
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
            Command::Operate(Operator::ADD),
        )), prog);
    }
}
