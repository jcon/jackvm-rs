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
pub struct Parser<'a> {
    position: i32,
    source: Vec<&'a str>,
    current_line: Vec<&'a str>,
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
        // let mut parser = Parser {
        //     position: 0,
        //     source: lines,
        //     current_line: vec!(),
        // };
        // parser.parse_line();
        // parser
        Parser {
            position: -1,
            source: lines,
            current_line: vec!(),
        }
    }

    // pub fn parse_line(&mut self) -> () {
    //     if self.position >= 0 && (self.position as usize) < self.source.len() {
    //         let parts: Vec<&str> = self.source[self.position as usize].split(' ').collect();
    //         println!("parts are {:?}", parts);
    //         self.current_line = parts
    //     }
    // }

    pub fn advance(&mut self) -> () {
        self.position = self.position + 1;
        let pos = self.position as usize;
        if pos < self.source.len() {
            let parts: Vec<&str> = self.source[pos].split(' ').collect();
            println!("parts are {:?}", parts);
            self.current_line = parts
        } else {
            self.current_line = vec!()
        }
    }

    pub fn has_more_commands(&self) -> bool {
        let next_pos = (self.position + 1) as usize;
        next_pos < self.source.len()
    }

    pub fn get_command_type(&self) -> Option<&str> {
        let pos = self.position as usize;
        if pos < self.source.len() {
            Some(self.current_line[0])
        } else {
            None
        }
    }

    pub fn get_arg1(&self) -> Option<&str> {
        if self.has_more_commands() && self.current_line.len() > 1 {
            Some(self.current_line[1])
        } else {
            None
        }
    }

    pub fn get_arg2(&self) -> Option<&str> {
        if self.has_more_commands() && self.current_line.len() > 2 {
            Some(self.current_line[2])
        } else {
            None
        }
    }
}

fn parse_push(parser: &Parser) -> Command {
    let arg1 = parser.get_arg1().unwrap();
    let arg2 = parser.get_arg2().unwrap().parse::<i32>().unwrap();
    match arg1 {
        "constant" => Command::Push(Segment::CONSTANT, arg2),
        // TODO: fix
        _ => Command::Push(Segment::ARG, 3)
    }
}

fn parse_operator(parser: &Parser) -> Command {
    match parser.get_command_type() {
        Some("add") => Command::Operate(Operator::ADD),
        // TODO: fix
        _ => Command::Push(Segment::ARG, 2)
    }
}

pub fn compile(source: &str) -> Vec<Command> {
    // let lines = source.split('\n');
    let mut parser = Parser::new(source);
    let mut program: Vec<Command> = Vec::new();

    // for line in source.lines() {

    // }
    while parser.has_more_commands() {
        parser.advance();
        println!("next command type is {:?}", parser.get_command_type());
        let command = match parser.get_command_type() {
            Some("push") => parse_push(&parser),
            Some("add") => parse_operator(&parser),
            // todo fix
            Some(x) => {
                println!("got {}", x);
                Command::Push(Segment::ARG, 1)
            },
            None => {
                println!("got NONE");
                Command::Push(Segment::ARG, 1)
            },
        };
        program.push(command);
    }

    // program.push(Command::Push(Segment::CONSTANT, 5));
    // program.push(Command::Push(Segment::CONSTANT, 4));
    // program.push(Command::Operate(Operator::ADD));

    program
    // &[
    //     Command::Push(Segment::CONSTANT, 5),
    //     Command::Push(Segment::CONSTANT, 4),
    //     Command::Operate(Operator::ADD),
    // ]
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
        assert_eq!(vec!(
            Command::Push(Segment::CONSTANT, 5),
            Command::Push(Segment::CONSTANT, 4),
            Command::Operate(Operator::ADD),
        ), prog);
    }
}
