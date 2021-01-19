use std::fmt;
use std::collections::HashMap;

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

impl Command {
    pub fn get_label(&self) -> Option<&String> {
        match self {
            Command::Label(m) => Some(&m),
            Command::Function(m, _) => Some(&m),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CompilationError {
    message: String,
    pub line_number: i32,
}

impl CompilationError {
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Instruction<'a> {
    line_number: i32,
    command_type: Option<&'a str>,
    arg1: Option<&'a str>,
    arg2: Option<&'a str>,
}

// impl<'a> fmt::Display for Instruction<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.command_type.and_then(|ct| {
//             write!(f, "{}", ct);
//             Some(ct)
//         });
//         self.arg1.and_then(|a1| {
//             write!(f, " {}", a1);
//             Some(a1)
//         });
//         self.arg2.and_then(|a2| {
//             write!(f, " {}", a2);
//             Some(a2)
//         });
//         Ok(())
//     }
// }

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
            // TODO: ensure tabs are trimmed off too.
            let mut parts = remove_comment(self.source[pos].trim().trim_end()).split(' ');
            self.current_instruction = Some(Instruction {
                line_number: self.position + 1,
                command_type: parts.next(),
                arg1: parts.next().map(|s| s.trim_end()),
                arg2: parts.next().map(|s| s.trim_end()),
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
            let s = remove_comment(self.source[pos].trim().trim_end());
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
        message: "Expected a memory segment for push".to_string(),
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
            message: "Unexpected segment for push".to_string(),
        })
    }?;

    let arg2 = parse_positive_int(instruction.arg2).map_err(
        |_e| CompilationError {
            line_number,
            // TODO: find a better way to match partial string for error.
        //    message: format!("Expected a positive integer for argument:\n{}\n\n{}\n", instruction, _e),
            message: "Expected a positive integer for argument".to_string(),
        })?;

    match instruction.command_type {
        Some("push") => Ok(Command::Push(segment, arg2)),
        Some("pop") => Ok(Command::Pop(segment, arg2)),
        _ => Err(CompilationError {
            line_number,
            message: "Expected a push or a pop".to_string(),
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
            message: "Unexpected arithmetic operation".to_string(),
        })
    }?;
    Ok(Command::Arithmetic(operation))
}

fn parse_label(cur_function: &str, instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the label".to_string(),
    })?;

    let mut label = cur_function.to_string();
    if label.len() > 0 {
        label.push_str("$");
    }
    label.push_str(arg1);
    println!("adding label {}", label);
    Ok(Command::Label(label))
}

fn parse_goto(cur_function: &str, instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the goto".to_string(),
    })?;

    let mut label = cur_function.to_string();
    if label.len() > 0 {
        label.push_str("$");
    }
    label.push_str(arg1);
    Ok(Command::Goto(String::from(label)))
}

fn parse_if_goto(cur_function: &str, instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let arg1 = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the if-goto".to_string(),
    })?;

    let mut label = cur_function.to_string();
    if label.len() > 0 {
        label.push_str("$");
    }
    label.push_str(arg1);
    Ok(Command::IfGoto(String::from(label)))
}

fn parse_function(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let name = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the function".to_string(),
    })?;

    let n_locals = parse_positive_int(instruction.arg2).map_err(
        |_| CompilationError {
            line_number,
            message: "Expected a positive integer for number of locals for the function".to_string(),
        })?;

    Ok(Command::Function(name.to_string(), n_locals))
}

fn parse_call(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    let name = instruction.arg1.ok_or(CompilationError {
        line_number,
        message: "Expected a name for the function".to_string(),
    })?;

    let n_args = parse_positive_int(instruction.arg2).map_err(
        |_| CompilationError {
            line_number,
            message: "Expected a positive integer for number of arguments being passed to the function".to_string(),
        })?;

    Ok(Command::Call(name.to_string(), n_args))
}

fn parse_return(instruction: &Instruction) -> Result<Command, CompilationError> {
    let line_number = instruction.line_number;
    match (instruction.arg1, instruction.arg2) {
        (None, None) => (),
        _ => return Err(CompilationError {
            line_number,
            message: "Expected no arguments for return".to_string(),
        })
    }

    Ok(Command::Return)
}

// TODO: find out how to set a statically defined set
fn is_arithmetic(command_type: &str) -> bool {
    let ct = command_type;
    ct == "add" || ct == "sub" || ct == "eq" || ct == "gt" || ct == "lt" || ct == "and" || ct == "or" || ct == "neg" || ct == "not"
}

pub fn compile(source: &str) -> Result<(Vec<Command>, HashMap<String, i16>), Vec<CompilationError>> {
    let mut parser = Parser::new(source);
    let mut program: Vec<Command> = Vec::new();
    let mut errors: Vec<CompilationError> = Vec::new();
    let mut addresses: HashMap<String, i16> = HashMap::new();

    let mut pc = 0;
    let mut cur_function: String = String::from("");
    while parser.has_more_commands() {
        parser.advance();
        let ins = parser.get_instruction();
        let line_number = parser.get_line_number();
        let command_or_error = match parser.get_command_type() {
            Some("push") | Some("pop") => parse_push_pop(&ins.unwrap()),
            Some(ct) if is_arithmetic(ct) => parse_arithmetic(&ins.unwrap()),
            Some("label") => parse_label(&cur_function, &ins.unwrap()),
            Some("goto") => parse_goto(&cur_function, &ins.unwrap()),
            Some("if-goto") => parse_if_goto(&cur_function, &ins.unwrap()),
            Some("function") => {
                let func_def = parse_function(&ins.unwrap());
                if let Ok(Command::Function(ref name, _)) = func_def {
                    cur_function = name.to_string();
                }
                func_def
            },
            Some("call") => parse_call(&ins.unwrap()),
            Some("return") => parse_return(&ins.unwrap()),
            _ => Err(CompilationError {
                line_number,
                message: "Unrecognized instruction".to_string(),
            }),
        };
        match command_or_error {
            Ok(command) => {
                if let Command::Function(_, _) = command {
                    addresses.insert(command.get_label().unwrap().clone(), pc);
                }

                if let Command::Label(_) = command {
                    addresses.insert(command.get_label().unwrap().clone(), pc);
                } else {
                    program.push(command);
                    pc += 1;
                }
            },
            Err(e) => errors.push(e),
        }
    }

    if errors.len() == 0 {
        Ok((program, addresses))
    } else {
        Err(errors)
    }
}

fn parse_positive_int(arg: Option<&str>) -> Result<i32, String> {
    let s = arg.ok_or("Expected a non-positive integer, but found nothing")?;
    let i = s.parse::<i32>().map_err(|e| format!("Cannot parse integer: {}", e))?;
    println!("args are {}", i);
    if i < 0 {
        Err(format!("Expected a non-negative integer, but got {}", i))
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
                message: "Unexpected arithmetic operation".to_string()
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
        assert_eq!(parse_push_pop(&ins), Ok(Command::Push(Segment::POINTER, 0)));
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
        let message = "Unexpected segment for push".to_string();
        assert_eq!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 3, message }));
    }

    #[test]
    fn test_parse_push_pop_with_no_arg2() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("push"),
            arg1: Some("static"),
            arg2: None,
        };
        let message = "Expected a positive integer for argument".to_string();
        assert_eq!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 8, message }));
    }

    #[test]
    fn test_parse_push_pop_with_no_arg1() {
        let ins = Instruction {
            line_number: 8,
            command_type: Some("doesntmatter"),
            arg1: None,
            arg2: None,
        };
        let  message = "Expected a memory segment for push".to_string();
        assert_eq!(parse_push_pop(&ins),
            Err(CompilationError{ line_number: 8, message }));
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

        assert_eq!(parse_label("Main.main", &ins), Ok(Command::Label("Main.main$WHILE-0".to_string())));
    }

    #[test]
    fn test_parse_invalid_label() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("label"),
            arg1: None,
            arg2: None,
        };

        let message = "Expected a name for the label".to_string();
        assert_eq!(parse_label("Main.main", &ins), Err(CompilationError {
            message,
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

        assert_eq!(parse_goto("Main.test", &ins), Ok(Command::Goto("Main.test$WHILE-1".to_string())));
    }

    #[test]
    fn test_parse_invalid_goto() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("label"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_goto("Main.test", &ins), Err(CompilationError {
            message: "Expected a name for the goto".to_string(),
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

        assert_eq!(parse_if_goto("Main.add", &ins), Ok(Command::IfGoto("Main.add$WHILE-1".to_string())));
    }

    #[test]
    fn test_parse_invalid_if_goto() {
        let ins = Instruction {
            line_number: 3,
            command_type: Some("if-goto"),
            arg1: None,
            arg2: None,
        };

        assert_eq!(parse_if_goto("Main.test", &ins), Err(CompilationError {
            message: "Expected a name for the if-goto".to_string(),
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
            message: "Expected a name for the function".to_string(),
            line_number: 3
        }));

        let ins = Instruction {
            line_number: 3,
            command_type: Some("function"),
            arg1: Some("Doesnt.Matter"),
            arg2: Some("-1"),
        };

        assert_eq!(parse_function(&ins), Err(CompilationError {
            message: "Expected a positive integer for number of locals for the function".to_string(),
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
            message: "Expected a name for the function".to_string(),
            line_number: 8
        }));

        let ins = Instruction {
            line_number: 4,
            command_type: Some("function"),
            arg1: Some("Doesnt.Matter"),
            arg2: Some("-1"),
        };

        assert_eq!(parse_call(&ins), Err(CompilationError {
            message: "Expected a positive integer for number of arguments being passed to the function".to_string(),
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
            message: "Expected no arguments for return".to_string(),
            line_number: 8
        }));

        let ins = Instruction {
            line_number: 4,
            command_type: Some("return"),
            arg1: None,
            arg2: Some("-1"),
        };

        assert_eq!(parse_return(&ins), Err(CompilationError {
            message: "Expected no arguments for return".to_string(),
            line_number: 4
        }));
    }

    #[test]
    fn test_compile_simple_add_program() {
        let source = "push constant 5
                      push constant 4
                      // comment on separate line
                      add // comment on same line
                      ";
        let result = compile(&source[..]);
        match result {
            Ok((prog, _)) =>  {
                assert_eq!(vec!(
                    Command::Push(Segment::CONSTANT, 5),
                    Command::Push(Segment::CONSTANT, 4),
                    Command::Arithmetic(Operator::ADD),
                ), prog);
            }
            Err(_) => {
                panic!("Expected OK response");
            }
        }
    }

    #[test]
    fn test_compile_produces_correct_addresses() {
        let source = "
                      push constant 1 // 0
                      label next_add
                      push constant 1 // 1
                      add // 2
                      goto next_add // 3
                      label label1
                      label label2
                      add // 4
                      function add_two 2 // 5
                      label before_add
                      push argument 0  // 6
                      push argument 1 // 7
                      add
                      return
                      ";
        let result = compile(&source[..]);
        match result {
            Ok((prog, addresses)) => {
                assert_eq!(vec!(
                    Command::Push(Segment::CONSTANT, 1),
                    Command::Push(Segment::CONSTANT, 1),
                    Command::Arithmetic(Operator::ADD),
                    Command::Goto("next_add".to_string()),
                    Command::Arithmetic(Operator::ADD),
                    Command::Function("add_two".to_string(), 2),
                    Command::Push(Segment::ARG, 0),
                    Command::Push(Segment::ARG, 1),
                    Command::Arithmetic(Operator::ADD),
                    Command::Return,
                ), prog);
                assert_eq!(addresses.get("next_add"), Some(&1));
                assert_eq!(addresses.get("label1"), Some(&4));
                assert_eq!(addresses.get("label2"), Some(&4));
                assert_eq!(addresses.get("add_two"), Some(&5));
                assert_eq!(addresses.get("add_two$before_add"), Some(&6));
            },
            Err(errors) => panic!("Expected Ok response: {:?}", errors),
        }
    }


    #[test]
    fn test_compile_simple_incorrect_program() {
        let source = "foo constant 5";
        let prog = compile(&source[..]);
        assert_eq!(Err(vec!(CompilationError {
            line_number: 1,
            message: "Unrecognized instruction".to_string(),
        })), prog);
    }
}
