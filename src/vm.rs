use std::collections::HashMap;
use crate::compiler::compile;
use crate::compiler::Command;
use crate::compiler::CompilationError;
use crate::compiler::Segment;
use crate::compiler::Operator;
extern crate web_sys;

pub struct VirtualMachine {
    pub memory: [i16; KEYBOARD_START + 1],
    pc: usize,
    program: Vec<Command>,
    pub addresses: HashMap<String, i16>,
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
                ();
            },
        }
        Ok(())
    }

    pub fn load(&mut self, prog: &[Command]) -> () {
        self.program = prog.iter().cloned().collect();
        self.pc = 0;
        self.memory[SP] = STACK_START as i16;
    }

    pub fn tick(&mut self) -> () {
        if self.pc >= self.program.len() {
            return ()
        }

        let command = &self.program[self.pc];
        // println!("running command {:?}", command);
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
            &Command::Function(_, n_locals) => self.process_function(n_locals),
            &Command::Call(ref name, n_args) => {
                let name_copy = name.clone();
                let result_pc = self.process_call(&name_copy, n_args);
                self.pc = result_pc;
                return; // don't increment the program counter automatically.
            },
            &Command::Return => {
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
        println!("return address: {}\n", return_addr);
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
                let address = STATIC_START + (offset as usize);
                self.memory[address]
            },
        };
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
                (STATIC_START as i16) + offset
            }
            _ => panic!("unexpected segment"),
        };
        let address = base_address as usize;
        let val = self.stack_pop();
        self.memory[address] = val;
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
                    Operator::NEG => 0 - arg1,
                    _ => panic!("Unexpected operator encountered"), // won't get here.
                };
                self.stack_push(result);
            },
            _ => {
                let arg2 = self.stack_pop();
                let arg1 = self.stack_pop();
                let result = match operator {
                    Operator::ADD => arg1 + arg2,
                    Operator::SUB => arg1 - arg2,
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
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 1);
    }

    #[test]
    pub fn test_or_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 3),
            Command::Push(Segment::CONSTANT, 4),
            Command::Arithmetic(Operator::OR),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 7);
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
    pub fn test_not_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, -1),
            Command::Arithmetic(Operator::NOT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 0);

        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 0),
            Command::Arithmetic(Operator::NOT),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), -1);
    }

    #[test]
    pub fn test_neg_instruction() {
        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, -99),
            Command::Arithmetic(Operator::NEG),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), 99);

        let vm = load_and_execute(&[
            Command::Push(Segment::CONSTANT, 54),
            Command::Arithmetic(Operator::NEG),
        ]);
        assert_eq!(vm.memory[SP], (STACK_START + 1) as i16);
        assert_eq!(vm.stack_peek(), -54);
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
}