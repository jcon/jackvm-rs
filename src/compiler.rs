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
            println!("parts are {:?}", self.current_instruction);
        } else {
            self.current_instruction = None;
        }
    }

    pub fn has_more_commands(&self) -> bool {
        let next_pos = (self.position + 1) as usize;
        next_pos < self.source.len()
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

fn parse_push(parser: &Parser) -> Result<Command, CompilationError> {
    let arg1 = parser.get_arg1().unwrap();
    let arg2 = parser.get_arg2().unwrap().parse::<i32>().unwrap();
    match arg1 {
        "constant" => Ok(Command::Push(Segment::CONSTANT, arg2)),
        // TODO: fix
        _ => Ok(Command::Push(Segment::ARG, 3))
    }
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
        // println!("next command type is {:?}", parser.get_command_type());
        let command = match parser.get_command_type() {
            Some("push") => parse_push(&parser),
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
