use std::collections::HashMap;
use std::cmp::max;
use std::num::Wrapping;
use crate::compiler::compile;
use crate::compiler::Command;
use crate::compiler::CompilationError;
use crate::compiler::Segment;
use crate::compiler::Operator;
extern crate web_sys;


struct FunctionCall {
    name: String,
}

impl FunctionCall {
    pub fn get_class_name(&self) -> &str {
        let end = self.name.find(".").unwrap_or(self.name.len());
        &self.name[..end]
    }

    pub fn get_function_name(&self) -> &str {
        let start = self.name.find(".").unwrap_or(self.name.len());
        &self.name[start + 1..]
    }
}

pub struct VirtualMachine {
    pub memory: [i16; KEYBOARD_START + 1],
    pc: usize,
    program: Vec<Command>,
    pub addresses: HashMap<String, i16>,
    static_addresses: HashMap<String, i32>,
    call_stack: Vec<FunctionCall>,
    is_debug: bool,
}

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const SP: usize = 0;
const LCL: usize = 1;
const ARG: usize = 2;
const THIS: usize = 3;
const THAT: usize = 4;
const TEMP_START: usize = 5;
const STATIC_START: usize = 16;
const STACK_START: usize = 256;
const _HEAP_START: usize = 2048;
pub const SCREEN_START: usize = 16384;
pub const KEYBOARD_START: usize = 24575;

const VM_TRUE: i16 = -1;
const VM_FALSE: i16 = 0;

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; KEYBOARD_START + 1],
            pc: 0,
            program: vec!(),
            addresses: HashMap::new(),
            static_addresses: HashMap::new(),
            call_stack: Vec::new(),
            is_debug: false,
        }
    }

    pub fn compile_and_load(&mut self, program: &str) -> Result<(), Vec<CompilationError>> {
        let (bytecode, addresses) = compile(program)?;
        self.addresses = addresses;
        self.load(&bytecode[..]);
        match self.addresses.get("Sys.init") {
            Some(addr) => {
                self.pc = *addr as usize;
                // log!("Loading with PC starting at {}", self.pc);
                println!("Loading with PC starting at {}", self.pc);
            },
            None => {
                // log!("No Sys.init found");
                println!("No Sys.init found");
                self.pc = 0;
            },
        }
        Ok(())
    }

    pub fn load(&mut self, prog: &[Command]) -> () {
        self.program = prog.iter().cloned().collect();
        self.pc = 0;
        self.memory[SP] = STACK_START as i16;
        self.static_addresses = get_static_addresses(&self.program);
        self.call_stack.clear();
    }

    pub fn get_instruction(&self) -> String {
        if self.pc < self.program.len() {
            format!("{:?}", self.program[self.pc])
        } else {
            "HALT".to_string()
        }
    }

    pub fn tick(&mut self) -> () {
        if self.pc >= self.program.len() {
            return ()
        }

        let command = &self.program[self.pc];
        if self.is_debug {
            println!("running command {}: {:?}", self.pc, command);
        }
        // log!("running command {:?}", command);
        match command {
            &Command::Push(segment, arg2) => self.stack_push_segment(segment, arg2 as i16),
            &Command::Pop(segment, arg2) => self.stack_pop_segment(segment, arg2 as i16),
            &Command::Arithmetic(operator) => self.process_arithmetic(operator),
            &Command::Goto(ref label) => {
                self.pc = *self.addresses.get(label).unwrap() as usize;
                return; // don't update the program counter
            },
            &Command::IfGoto(ref label) => {
                // NOTE: we cannot have an immutable reference to label while having
                //       an immutable reference to self.
                let label_copy = label.clone();
                if self.process_if_goto(&label_copy) {
                    return; // don't update the program counter when if-goto was successful
                }
            },
            &Command::Function(ref function_name, n_locals) => {
                self.call_stack.push(FunctionCall { name: function_name.to_string() });
                // if function_name == "Math.sqrt" {
                //     self.is_debug = true;
                // } else {
                //     self.is_debug = false;
                // }
                // if self.is_debug && function_name == "Math.multiply" {
                //     let args_addr = self.peek(2) as usize;
                //     println!("{} * {} ({}, {})", self.peek(args_addr), self.peek(args_addr + 1), args_addr, args_addr + 1);
                // }
                self.process_function(n_locals);
            },
            &Command::Call(ref name, n_args) => {
                let name_copy = name.clone();
                let result_pc = self.process_call(&name_copy, n_args);
                if name_copy == "Sys.error" {
                    panic!("Calling Sys.error from {}", self.call_stack.last().unwrap().name);
                }
                self.pc = result_pc;
                return; // don't increment the program counter automatically.
            },
            &Command::Return => {
                self.call_stack.pop();
                // match self.call_stack.last() {
                //     Some(ref func_call) if func_call.get_function_name() == "sqrt" =>
                //         self.is_debug = true,
                //     _ => (),
                // }
                let result_pc = self.process_return();
                self.pc = result_pc;
                return;
            },
            _ => panic!(format!("unimplemented command: {:?}", command))
        };
        self.pc = self.pc + 1;
    }

    fn process_if_goto(&mut self, address: &str) -> bool {
        if self.stack_pop() != VM_FALSE {
            if let None = self.addresses.get(address) {
                panic!("Can't find address {} in {:?}", address, self.addresses);
            }
            self.pc = *self.addresses.get(address).unwrap() as usize;
            true
        } else {
            false
        }
    }

    fn process_call(&mut self, function_name: &str, n_args: i32) -> usize {
        // log!("calling {} @ {}", function_name, *self.addresses.get(function_name).unwrap_or(&0));
        // let sp = self.memory[0];
        // if n_args > 0 {
        //     log!("arg0: {}", self.memory[(sp - n_args as i16 - 1) as usize]);
        // }
        // if n_args > 1 {
        //     log!("arg1: {}", self.memory[(sp - n_args as i16 - 1 + 1) as usize]);
        // }
        // if n_args > 2 {
        //     log!("arg2: {}", self.memory[(sp - n_args as i16 - 1 + 2) as usize]);
        // }
        self.stack_push((self.pc + 1) as i16);
        self.stack_push(self.memory[LCL]);
        self.stack_push(self.memory[ARG]);
        self.stack_push(self.memory[THIS]);
        self.stack_push(self.memory[THAT]);
        self.memory[ARG] = self.memory[SP] - (n_args as i16) - 5;
        self.memory[LCL] = self.memory[SP];

        let new_pc = *self.addresses.get(function_name).unwrap() as usize;
        new_pc
    }

    fn process_function(&mut self, n_locals: i32) -> () {
        for _ in 0..n_locals {
            self.stack_push(0);
        }
    }

    fn process_return(&mut self) -> usize {
        let frame = self.memory[LCL] as usize;
        let return_addr = self.memory[frame - 5] as usize;
        self.memory[self.memory[ARG] as usize] = self.stack_pop();
        self.memory[SP] = self.memory[LCL] - 1;
        // self.memory[THAT] = self.stack_pop();
        // self.memory[THIS] = self.stack_pop();
        // self.memory[ARG] = self.stack_pop();
        // self.memory[LCL] = self.stack_pop();
        self.memory[SP] = self.memory[ARG] + 1;
        self.memory[THAT] = self.memory[frame - 1];
        self.memory[THIS] = self.memory[frame - 2];
        self.memory[ARG] = self.memory[frame - 3];
        self.memory[LCL] = self.memory[frame - 4];
        // println!("return address: {}\n", return_addr);
        return_addr
    }

    pub fn peek(&self, address: usize) -> i16 {
        self.memory[address]
    }

    pub fn poke(&mut self, address: usize, val: i16) -> () {
        self.memory[address] = val;
    }

    fn stack_push(&mut self, val: i16) -> () {
        self.memory[self.memory[SP] as usize] = val;
        self.memory[SP] = self.memory[SP] + 1;
    }

    fn stack_push_segment(&mut self, segment: Segment, offset: i16) -> () {
        let val = match segment {
            Segment::CONSTANT => offset,
            Segment::LOCAL => self.dereference(LCL, offset),
            Segment::ARG => self.dereference(ARG, offset),
            Segment::THIS => self.dereference(THIS, offset),
            Segment::THAT => self.dereference(THAT, offset),
            Segment::POINTER => {
                let address_pointer = if offset == 0 {
                    THIS
                } else {
                    THAT
                };
                self.memory[address_pointer]
            },
            Segment::TEMP => {
                let address = TEMP_START + (offset as usize);
                self.memory[address]
            },
            Segment::STATIC => {
                let address = (self.get_static_base() as i16 + offset) as usize;
                self.memory[address]
            },
        };
        if self.is_debug {
            println!("{}", val);
        }
        self.stack_push(val);
    }

    #[allow(unused)]
    fn stack_pop_segment(&mut self, segment: Segment, offset: i16) -> () {
        let base_address = match segment {
            // TODO: it's an error to "pop" to constant.
//            Segment::CONSTANT => offset,
            Segment::LOCAL => self.memory[LCL] + offset,
            Segment::ARG => self.memory[ARG] + offset,
            Segment::THIS => self.memory[THIS] + offset,
            Segment::THAT => self.memory[THAT] + offset,
            Segment::POINTER => {
                if offset == 0 {
                    THIS as i16
                } else {
                    THAT as i16
                }
            },
            Segment::TEMP => {
                (TEMP_START as i16) + offset
            },
            Segment::STATIC => {
                self.get_static_base() as i16 + offset
            }
            _ => panic!("unexpected segment"),
        };
        let address = base_address as usize;
        let val = self.stack_pop();
        if self.is_debug {
            println!("{}", val);
        }
        self.memory[address] = val;
    }

    fn get_static_base(&self) -> i32 {
        let static_start = STATIC_START as i32;
        match self.call_stack.last() {
            Some(ref cur_fn) => {
                *self.static_addresses.get(cur_fn.get_class_name()).unwrap()
            },
            None => {
                println!("no current function use STATIC_START = {}", static_start);
                static_start
            }
        }
    }

    fn dereference(&self, base: usize, offset: i16) -> i16 {
        let address = (self.memory[base] + offset) as usize;
        self.memory[address]
    }

    fn stack_pop(&mut self) -> i16 {
        let address = (self.memory[SP] - 1) as usize;
        self.memory[SP] = address as i16;
        self.memory[address]
    }

    #[allow(unused)]
    fn stack_peek(&self) -> i16 {
        let address = (self.memory[SP] - 1) as usize;
        self.memory[address]
    }

    fn process_arithmetic(&mut self, operator: Operator) -> () {
        match operator {
            Operator::NOT | Operator::NEG => {
                let arg1 = self.stack_pop();
                let result = match operator {
                    Operator::NOT => !arg1,
                    Operator::NEG => (Wrapping(0) - Wrapping(arg1)).0,
                    _ => panic!("Unexpected operator encountered"), // won't get here.
                };
                self.stack_push(result);
            },
            _ => {
                let arg2 = self.stack_pop();
                let arg1 = self.stack_pop();
                let result = match operator {
                    Operator::ADD => {
                        // println!("adding {} + {}", arg1, arg2);
                        (Wrapping(arg1) + Wrapping(arg2)).0
                    },
                    Operator::SUB => {
                        (Wrapping(arg1) - Wrapping(arg2)).0
                    },
                    Operator::EQ => if arg1 == arg2 { VM_TRUE } else { VM_FALSE },
                    Operator::LT => if arg1 < arg2 { VM_TRUE } else { VM_FALSE },
                    Operator::GT => if arg1 > arg2 { VM_TRUE } else { VM_FALSE },
                    Operator::AND => arg1 & arg2,
                    Operator::OR => arg1 | arg2,
                    _ => panic!("Unexpected operator encountered"), // won't get here.
                };
                self.stack_push(result);
            },
        };
    }
}

fn get_static_addresses(prog: &[Command]) -> HashMap<String, i32> {
    let mut addresses = HashMap::new();
    let mut current_class: String = "STATIC".to_string();
    for command in prog.iter() {
        match command {
            &Command::Function(ref name, _) => {
                let mut parts = name.split(".");
                current_class = parts.next().unwrap().to_string();
            },
            &Command::Push(Segment::STATIC, addr) | &Command::Pop(Segment::STATIC, addr) => {
                let max_addr = addresses.entry(current_class.to_string()).or_insert(0);
                *max_addr = max(*max_addr, addr);
            },
            _ => (),
        }
    }

    // Update starting addresses so that that each segment has the appropriate
    // slots for each class.
    let mut base_addr = STATIC_START as i32;
    for (_, val) in &mut addresses {
        let static_count = *val;
        *val = base_addr;
        base_addr += static_count + 1;
    }

    addresses
}

#[cfg(test)]
mod test {
    use super::*;

    fn load_and_execute(prog: &[Command]) -> VirtualMachine {
        let mut vm = VirtualMachine::new();
        vm.load(prog);

        for _ in 0..100 {
            vm.tick();
        }

        vm
    }

    #[test]
    pub fn function_call_prases_names() {
        let call = FunctionCall { name: "Sys.init".to_string() };
        assert_eq!(call.get_class_name(), "Sys");
        assert_eq!(call.get_function_name(), "init");
    }

    #[test]
    pub fn test_add_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 8),
            Command::Push(Segment::CONSTANT, 7),
            Command::Arithmetic(Operator::ADD),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 15)
    }

    #[test]
    pub fn test_arithemetic_and_overflows() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 2),
            Command::Push(Segment::CONSTANT, 2),
            Command::Arithmetic(Operator::ADD),
            Command::Pop(Segment::STATIC, 0),
            Command::Push(Segment::CONSTANT, 4),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::ADD),
            Command::Pop(Segment::STATIC, 1),
            Command::Push(Segment::CONSTANT, 1024),
            Command::Push(Segment::CONSTANT, 1024),
            Command::Arithmetic(Operator::ADD),
            Command::Pop(Segment::STATIC, 2),
            Command::Push(Segment::CONSTANT, 16384),
            Command::Push(Segment::CONSTANT, 16384),
            Command::Arithmetic(Operator::ADD),
            Command::Pop(Segment::STATIC, 3),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::NEG),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::SUB),
            Command::Pop(Segment::STATIC, 4),
            Command::Push(Segment::CONSTANT, 16384),
            Command::Arithmetic(Operator::NEG),
            Command::Push(Segment::CONSTANT, 32767),
            Command::Arithmetic(Operator::SUB),
            Command::Pop(Segment::STATIC, 5),
        ]);

        assert_eq!(vm.peek(STATIC_START + 0), 2 + 2);
        assert_eq!(vm.peek(STATIC_START + 1), 4 + 4);
        assert_eq!(vm.peek(STATIC_START + 2), 1024 + 1024);
        assert_eq!(vm.peek(STATIC_START + 3), (Wrapping(16384i16) + Wrapping(16384i16)).0);
        assert_eq!(vm.peek(STATIC_START + 4), -4 - 4);
        assert_eq!(vm.peek(STATIC_START + 5), (Wrapping(-16384i16) - Wrapping(32767i16)).0);
    }


    #[test]
    pub fn test_sub_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 16),
            Command::Push(Segment::CONSTANT, 7),
            Command::Arithmetic(Operator::SUB),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 9);
    }

    #[test]
    pub fn test_and_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 3),
            Command::Push(Segment::CONSTANT, 5),
            Command::Arithmetic(Operator::AND),
            Command::Pop(Segment::STATIC, 0),

            Command::Push(Segment::CONSTANT, -32768),
            Command::Push(Segment::CONSTANT, 32767),
            Command::Arithmetic(Operator::AND),
            Command::Pop(Segment::STATIC, 1), // should be 0, no bits overlap

            Command::Push(Segment::CONSTANT, -32768),
            Command::Push(Segment::CONSTANT, -1),
            Command::Arithmetic(Operator::AND),
            Command::Pop(Segment::STATIC, 2), // should be -32768, only last bit overlaps
        ]);

        assert_eq!(vm.peek(STATIC_START + 0), 1);
        assert_eq!(vm.peek(STATIC_START + 1), 0);
        assert_eq!(vm.peek(STATIC_START + 2), -32768);
    }

    #[test]
    pub fn test_or_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 3),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::OR),
            Command::Pop(Segment::STATIC, 0),

            Command::Push(Segment::CONSTANT, -32768),
            Command::Push(Segment::CONSTANT, 32767),
            Command::Arithmetic(Operator::OR),
            Command::Pop(Segment::STATIC, 1), // should be -1 (all bits set)
        ]);
        assert_eq!(vm.peek(STATIC_START + 0), 7);
        assert_eq!(vm.peek(STATIC_START + 1), -1);
    }

    #[test]
    pub fn test_eq_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 88),
            Command::Push(Segment::CONSTANT, 89),
            Command::Arithmetic(Operator::EQ),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_FALSE);

        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 101),
            Command::Push(Segment::CONSTANT, 101),
            Command::Arithmetic(Operator::EQ),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_TRUE);
    }

    #[test]
    pub fn test_lt_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 88),
            Command::Push(Segment::CONSTANT, 89),
            Command::Arithmetic(Operator::LT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_TRUE);

        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 101),
            Command::Push(Segment::CONSTANT, 101),
            Command::Arithmetic(Operator::LT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_FALSE);
    }

    #[test]
    pub fn test_gt_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 105),
            Command::Push(Segment::CONSTANT, 55),
            Command::Arithmetic(Operator::GT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_TRUE);

        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 9998),
            Command::Push(Segment::CONSTANT, 9999),
            Command::Arithmetic(Operator::GT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), VM_FALSE);
    }

    #[test]
    pub fn test_if_goto() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
            Command::Push(Segment::LOCAL, 2), // 0
            Command::Push(Segment::ARG, 0),
            Command::Arithmetic(Operator::GT),
            Command::Arithmetic(Operator::NOT),
            Command::Push(Segment::LOCAL, 2),
            Command::Push(Segment::CONSTANT, 0), // 5
            Command::Arithmetic(Operator::LT),
            Command::Arithmetic(Operator::NOT),
            Command::Arithmetic(Operator::AND),
            Command::IfGoto("IF_TRUE1".to_string()),
            Command::Goto("IF_FALSE1".to_string()), // 10
//            Command::Label("IF_TRUE1".to_string()), // 11
            Command::Push(Segment::CONSTANT, 1), // 11
            Command::Pop(Segment::STATIC, 0),
            Command::Goto("END_IF1".to_string()),
//            Command::Label("IF_FALSE1".to_string()), // 14
            Command::Push(Segment::CONSTANT, 2), // 14
            Command::Pop(Segment::STATIC, 0),
//            Command::Label("END_IF1".to_string()), // 16
        ]);
        vm.addresses.insert("IF_TRUE1".to_string(), 11);
        vm.addresses.insert("IF_FALSE1".to_string(), 14);
        vm.addresses.insert("END_IF1".to_string(), 16);

        vm.memory[ARG] = 256;
        vm.memory[256] = 9;
        vm.memory[LCL] = 257;
        vm.memory[259] = 4;
        vm.memory[SP] = 261;

        for _ in 0..100 {
            vm.tick();
        }

        assert_eq!(vm.peek(STATIC_START + 0), 1);
    }

    #[test]
    pub fn test_not_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, -1),
            Command::Arithmetic(Operator::NOT),
            Command::Pop(Segment::STATIC, 0),

            Command::Push(Segment::CONSTANT, 0),
            Command::Arithmetic(Operator::NOT),
            Command::Pop(Segment::STATIC, 1),

            Command::Push(Segment::CONSTANT, 32767),
            Command::Arithmetic(Operator::NEG),
            Command::Push(Segment::CONSTANT, 1),
            Command::Arithmetic(Operator::SUB),
            Command::Arithmetic(Operator::NOT),
            Command::Pop(Segment::STATIC, 2),
        ]);

        assert_eq!(vm.peek(STATIC_START + 0), 0);
        assert_eq!(vm.peek(STATIC_START + 1), -1);
        assert_eq!(vm.peek(STATIC_START + 2), 32767);
    }

    #[test]
    pub fn test_neg_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, -99),
            Command::Arithmetic(Operator::NEG),
            Command::Pop(Segment::STATIC, 0),

            Command::Push(Segment::CONSTANT, 54),
            Command::Arithmetic(Operator::NEG),
            Command::Pop(Segment::STATIC, 1),

            Command::Push(Segment::CONSTANT, -32768),
            Command::Arithmetic(Operator::NEG),
            Command::Pop(Segment::STATIC, 2),
        ]);


        assert_eq!(vm.memory[STATIC_START + 0], 99);
        assert_eq!(vm.memory[STATIC_START + 1], -54);
        assert_eq!(vm.memory[STATIC_START + 2], -32768i16);
    }

    #[test]
    pub fn test_execute_simple_addition() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
            Command::Push(Segment::CONSTANT, 8),
            Command::Push(Segment::CONSTANT, 7),
            Command::Arithmetic(Operator::ADD),
        ]);

        vm.tick();
        vm.tick();
        vm.tick();

        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 15);
    }

    #[test]
    pub fn test_basic_stack_push() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
        ]);

        let mut address: usize = 256;
        vm.memory[LCL] = address as i16;
        vm.memory[address] = 100;

        address += 1;
        vm.memory[ARG] = address as i16;
        vm.memory[address] = 200;

        address += 1;
        vm.memory[THIS] = address as i16;
        vm.memory[address] = 300;

        address += 1;
        vm.memory[THAT] = address as i16;
        vm.memory[address] = 400;

        vm.memory[STATIC_START + 4] = 500;

        vm.memory[TEMP_START + 2] = 600;

        address += 1;
        vm.memory[SP] = address as i16;

        vm.stack_push_segment(Segment::LOCAL, 0);
        vm.stack_push_segment(Segment::ARG, 0);
        vm.stack_push_segment(Segment::THIS, 0);
        vm.stack_push_segment(Segment::THAT, 0);
        vm.stack_push_segment(Segment::POINTER, 0);
        vm.stack_push_segment(Segment::POINTER, 1);
        vm.stack_push_segment(Segment::STATIC, 4);
        vm.stack_push_segment(Segment::TEMP, 2);

        assert_eq!(vm.stack_pop(), vm.memory[TEMP_START + 2]);
        assert_eq!(vm.stack_pop(), vm.memory[STATIC_START + 4]);
        assert_eq!(vm.stack_pop(), vm.memory[THAT]);
        assert_eq!(vm.stack_pop(), vm.memory[THIS]);
        assert_eq!(vm.stack_pop(), 400);
        assert_eq!(vm.stack_pop(), 300);
        assert_eq!(vm.stack_pop(), 200);
        assert_eq!(vm.stack_pop(), 100);

        assert_eq!(vm.memory[SP], (STACK_START + 4) as i16);
    }

    #[test]
    pub fn test_basic_stack_pop() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
        ]);

        let n_local = 2;
        let n_args = 3;
        let n_fields = 2;

        let mut address: usize = 256;
        vm.memory[LCL] = address as i16;
        address += n_local;

        vm.memory[ARG] = address as i16;
        address += n_args;

        vm.memory[THIS] = address as i16;
        address += n_fields;

        vm.memory[THAT] = address as i16;
        address += 1;

        vm.memory[SP] = address as i16;

        vm.stack_push(400);
        vm.stack_push(300);
        vm.stack_push(200);
        vm.stack_push(100);

        vm.stack_pop_segment(Segment::LOCAL, 0);
        vm.stack_pop_segment(Segment::ARG, 0);
        vm.stack_pop_segment(Segment::THIS, 0);
        vm.stack_pop_segment(Segment::THAT, 0);

        assert_eq!(vm.dereference(THAT, 0), 400);
        assert_eq!(vm.dereference(THIS, 0), 300);
        assert_eq!(vm.dereference(ARG, 0), 200);
        assert_eq!(vm.dereference(LCL, 0), 100);

        assert_eq!(vm.memory[SP], (STACK_START + n_local + n_args + n_fields + 1) as i16);

        vm.memory[THIS] = -1;
        vm.memory[THAT] = -1;
        vm.memory[STATIC_START + 4] = -1;
        vm.memory[TEMP_START + 2] = -1;

        vm.stack_push(40);
        vm.stack_push(30);
        vm.stack_push(20);
        vm.stack_push(10);

        vm.stack_pop_segment(Segment::POINTER, 0);
        vm.stack_pop_segment(Segment::POINTER, 1);
        vm.stack_pop_segment(Segment::STATIC, 4);
        vm.stack_pop_segment(Segment::TEMP, 2);

        assert_eq!(vm.memory[THIS], 10);
        assert_eq!(vm.memory[THAT], 20);
        assert_eq!(vm.memory[STATIC_START + 4], 30);
        assert_eq!(vm.memory[TEMP_START + 2], 40);
    }

    #[test]
    pub fn test_get_static_addresses() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
            Command::Function("Test1.main".to_string(), 0),
            Command::Push(Segment::CONSTANT, 2),
            Command::Push(Segment::CONSTANT, 1),
            Command::Push(Segment::STATIC, 0),
            Command::Push(Segment::STATIC, 1),
            Command::Return,
            Command::Function("Test2.main".to_string(), 0),
            Command::Push(Segment::CONSTANT, 4),
            Command::Push(Segment::CONSTANT, 3),
            Command::Push(Segment::STATIC, 0),
            Command::Push(Segment::STATIC, 1),
            Command::Return,
        ]);

        assert_eq!(vm.static_addresses.len(), 2);
        assert_ne!(*vm.static_addresses.get("Test1").unwrap(),
            *vm.static_addresses.get("Test2").unwrap());

        let test1 = *vm.static_addresses.get("Test1").unwrap();
        if test1 != 16 && test1 != 18 {
            panic!("Test1 does not have expected static segment.\n\nExpected [16, 18] but got {}", test1);
        }
        let test2 = *vm.static_addresses.get("Test2").unwrap();
        if test2 != 16 && test2 != 18 {
            panic!("Test2 does not have expected static segment.\n\nExpected [16, 18] but got {}", test2);
        }
    }
}