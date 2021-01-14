#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Push(Segment, i32),
    Pop(Segment, i32),
    Arithmetic(Operator),
    Label(String),
    Goto(String),
    IfGoto(String),
    Function(String, i32),
    Call(String, i32),
    Return,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompilationError {
    pub message: &'static str,
    pub line_number: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instruction<'a> {
    line_number: i32,
    command_type: Option<&'a str>,
    arg1: Option<&'a str>,
    arg2: Option<&'a str>,
}

pub struct Parser<'a> {
    position: i32,
    source: Vec<&'a str>,
    current_instruction: Option<Instruction<'a>>,
}

fn remove_comment(line: &str) -> &str {
    let end = line.find("//").unwrap_or(line.len());
    &line[..end]
}

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
        let pos = self.get_next_pos();
        if pos < self.source.len() {
            let mut parts = remove_comment(self.source[pos].trim()).split(' ');
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

        self.position = pos as i32;
    }

    pub fn has_more_commands(&self) -> bool {
        self.get_next_pos() < self.source.len()
    }

    pub fn get_instruction(&self) -> &Option<Instruction<'a>> {
        &self.current_instruction
    }

    pub fn get_line_number(&self) -> i32 {
        self.position + 1
    }

    pub fn get_command_type(&self) -> Option<&str> {
        self.current_instruction?.command_type
    }

    fn get_next_pos(&self) -> usize {
        let mut pos = (self.position + 1) as usize;
        while pos < self.source.len() {
            let s = remove_comment(self.source[pos].trim());
            if s.len() != 0 {
                break
            }
            pos = pos + 1;
        }
        pos
    }
}

fn parse_push_pop(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a memory segment for push",
    })?;
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

    match instruction.command_type {
        Some("push") => Ok(Command::Push(segment, arg2)),
        Some("pop") => Ok(Command::Pop(segment, arg2)),
        _ => Err(CompilationError {
            line_number,
            message: "Expected a push or a pop",
        }),
    }
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

fn parse_label(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the label",
    })?;
    Ok(Command::Label(String::from(arg1)))
}

fn parse_goto(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the goto",
    })?;
    Ok(Command::Goto(String::from(arg1)))
}

fn parse_if_goto(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the if-goto",
    })?;
    Ok(Command::IfGoto(String::from(arg1)))
}

fn parse_function(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let name = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the function",
    })?;

    let n_locals = parse_positive_int(instruction.arg2).map_err(
        |_| CompilationError {
            line_number,
            message: "Expected a positive integer for number of locals for the function",
        })?;

    Ok(Command::Function(name.to_string(), n_locals))
}

fn parse_call(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let name = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the function",
    })?;

    let n_args = parse_positive_int(instruction.arg2).map_err(
        |_| CompilationError {
            line_number,
            message: "Expected a positive integer for number of arguments being passed to the function",
        })?;

    Ok(Command::Call(name.to_string(), n_args))
}

fn parse_return(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    match (instruction.arg1, instruction.arg2) {
        (None, None) => (),
        _ => return Err(CompilationError {
            line_number,
            message: "Expected no arguments for return",
        })
    }

    Ok(Command::Return)
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
            Some("push") | Some("pop") => parse_push_pop(&ins.unwrap()),
            Some(ct) if is_arithmetic(ct) => parse_arithmetic(&ins.unwrap()),
            Some("label") => parse_label(&ins.unwrap()),
            Some("goto") => parse_goto(&ins.unwrap()),
            Some("if-goto") => parse_if_goto(&ins.unwrap()),
            Some("function") => parse_function(&ins.unwrap()),
            Some("call") => parse_call(&ins.unwrap()),
            Some("return") => parse_return(&ins.unwrap()),
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

fn parse_positive_int(arg: Option<&str>) -> Result<i32, String> {
    let s = arg.ok_or("Expected a positive integer")?;
    let i = s.parse::<i32>().map_err(|_| "Expected a positive integer".to_string())?;
    println!("args are {}", i);
    if i < 0 {
        Err("Expected a positive integer".to_string())
    } else {
        Ok(i)
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
    fn test_parse_push_pop_with_valid_push() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("push"),
            arg1: Some("pointer"),
            arg2: Some("0"),
        };
        assert_matches!(parse_push_pop(&ins), Ok(Command::Push(Segment::POINTER, 0)));
    }

    #[test]
    fn test_parse_push_pop_with_valid_pop() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("pop"),
            arg1: Some("pointer"),
            arg2: Some("1"),
        };
        assert_matches!(parse_push_pop(&ins), Ok(Command::Pop(Segment::POINTER, 1)));
    }

    #[test]
    fn test_parse_push_pop_with_invalid_segment() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("push"),
            arg1: Some("invalid"),
            arg2: Some("10"),
        };
        assert_matches!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 3, message: "Unexpected segment for push" }));
    }

    #[test]
    fn test_parse_push_pop_with_no_arg2() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("push"),
            arg1: Some("static"),
            arg2: None,
        };
        assert_matches!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 8, message: "Expected a second argement for push"}));
    }

    #[test]
    fn test_parse_push_pop_with_no_arg1() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("doesntmatter"),
            arg1: None,
            arg2: None,
        };
        assert_matches!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 8, message: "Expected a memory segment for push" }));
    }

    #[test]
    fn test_parse_valid_label() {
        let label = "WHILE-0";
        let ins = Instruction {
            line_number: 3,
            command_type: Some("label"),
            arg1: Some(label),
            arg2: None,
        };

        assert_eq!(parse_label(&ins), Ok(Command::Label(label.to_string())));
    }

    #[test]
    fn test_parse_invalid_label() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("label"),
            arg1: None,
            arg2: None,
        };

        assert_matches!(parse_label(&ins), Err(CompilationError {
            message: "Expected a name for the label",
            line_number: 3
        }));
    }

    #[test]
    fn test_parse_valid_goto() {
        let label = "WHILE-1";
        let ins = Instruction {
            line_number: 3,
            command_type: Some("goto"),
            arg1: Some(label),
            arg2: None,
        };

        assert_eq!(parse_goto(&ins), Ok(Command::Goto(label.to_string())));
    }

    #[test]
    fn test_parse_invalid_goto() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("label"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_goto(&ins), Err(CompilationError {
            message: "Expected a name for the goto",
            line_number: 3
        }));
    }

    #[test]
    fn test_parse_valid_if_goto() {
        let label = "WHILE-1";
        let ins = Instruction {
            line_number: 3,
            command_type: Some("if-goto"),
            arg1: Some(label),
            arg2: None,
        };

        assert_eq!(parse_if_goto(&ins), Ok(Command::IfGoto(label.to_string())));
    }

    #[test]
    fn test_parse_invalid_if_goto() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("if-goto"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_if_goto(&ins), Err(CompilationError {
            message: "Expected a name for the if-goto",
            line_number: 3
        }));
    }

    #[test]
    fn test_parse_valid_function() {
        let function_name = "Sys.init";
        let ins = Instruction {
            line_number: 3,
            command_type: Some("function"),
            arg1: Some(function_name),
            arg2: Some("2"),
        };

        assert_eq!(parse_function(&ins), Ok(Command::Function(function_name.to_string(), 2)));
    }

    #[test]
    fn test_parse_invalid_function() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("function"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_function(&ins), Err(CompilationError {
            message: "Expected a name for the function",
            line_number: 3
        }));

        let ins = Instruction {
            line_number: 3,
            command_type: Some("function"),
            arg1: Some("Doesnt.Matter"),
            arg2: Some("-1"),
        };

        assert_eq!(parse_function(&ins), Err(CompilationError {
            message: "Expected a positive integer for number of locals for the function",
            line_number: 3
        }));
    }

    #[test]
    fn test_parse_valid_call() {
        let function_name = "Sys.init";
        let ins = Instruction {
            line_number: 3,
            command_type: Some("call"),
            arg1: Some(function_name),
            arg2: Some("0"),
        };

        assert_eq!(parse_call(&ins), Ok(Command::Call(function_name.to_string(), 0)));
    }

    #[test]
    fn test_parse_invalid_call() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("call"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_call(&ins), Err(CompilationError {
            message: "Expected a name for the function",
            line_number: 8
        }));

        let ins = Instruction {
            line_number: 4,
            command_type: Some("function"),
            arg1: Some("Doesnt.Matter"),
            arg2: Some("-1"),
        };

        assert_eq!(parse_call(&ins), Err(CompilationError {
            message: "Expected a positive integer for number of arguments being passed to the function",
            line_number: 4
        }));
    }

    #[test]
    fn test_parse_valid_return() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("return"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_return(&ins), Ok(Command::Return));
    }

    #[test]
    fn test_parse_invalid_return() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("return"),
            arg1: Some("something"),
            arg2: None,
        };

        assert_eq!(parse_return(&ins), Err(CompilationError {
            message: "Expected no arguments for return",
            line_number: 8
        }));

        let ins = Instruction {
            line_number: 4,
            command_type: Some("return"),
            arg1: None,
            arg2: Some("-1"),
        };

        assert_eq!(parse_return(&ins), Err(CompilationError {
            message: "Expected no arguments for return",
            line_number: 4
        }));
    }

    #[test]
    fn test_simple_add_program() {
        let source = "push constant 5
                      push constant 4
                      // comment on separate line
                      add // comment on same line
                      ";
        let prog = compile(&source[..]);
        assert_eq!(Ok(vec!(
            Command::Push(Segment::CONSTANT, 5),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::ADD),
        )), prog);
    }

    #[test]
    fn test_compile_simple_incorrect_program() {
        let source = "foo constant 5";
        let prog = compile(&source[..]);
        assert_eq!(Err(vec!(CompilationError {
            line_number: 1,
            message: "Unrecognized instruction",
        })), prog);
    }
}
