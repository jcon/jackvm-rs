use crate::compiler::Command;
use crate::compiler::Segment;
use crate::compiler::Operator;

pub struct VirtualMachine {
    memory: [i16; KEYBOARD_START + 1], // Vec<i16>,
    pc: usize,
    program: Vec<Command>,
}

const SP: usize = 0;
// const LCL: usize = 1;
// const ARG: usize = 2;
// const THIS: usize = 3;
// const THAT: usize = 4;
// const TEMP0: usize = 5;
// const STATIC_START: usize = 16;
const STACK_START: usize = 256;
// const HEAP_START: usize = 2048;
// const SCREEN_START: usize = 16384;
const KEYBOARD_START: usize = 24575;

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; KEYBOARD_START + 1],
            pc: 0,
            program: vec!(),
        }
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

        let command = self.program[self.pc];
        match command {
            Command::Push(segment, arg2) => self.stack_push(self.lookup_val(segment, arg2 as i16)),
            Command::Arithmetic(operator) => self.process_arithmetic(operator),
            _ => println!("un implemented command"),
        };
        self.pc = self.pc + 1;
    }

    fn stack_push(&mut self, val: i16) -> () {
        self.memory[self.memory[SP] as usize] = val;
        self.memory[SP] = self.memory[SP] + 1;
    }

    fn stack_pop(&mut self) -> i16 {
        let address = (self.memory[SP] - 1) as usize;
        self.memory[SP] = address as i16;
        self.memory[address]
    }

    fn stack_peek(&self) -> i16 {
        let address = (self.memory[SP] - 1) as usize;
        self.memory[address]
    }

    fn lookup_val(&self, segment: Segment, offset: i16) -> i16 {
        match segment {
            Segment::CONSTANT => offset,
            _ => 0
        }
    }

    fn process_arithmetic(&mut self, operator: Operator) -> () {
        match operator {
            Operator::ADD => {
                let arg2 = self.stack_pop();
                let arg1 = self.stack_pop();
                self.stack_push(arg1 + arg2);
            },
            _ => println!("unimplemented!")
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}