use crate::compiler::Command;
use crate::compiler::Segment;
use crate::compiler::Operator;

pub struct VirtualMachine {
    memory: [i16; KEYBOARD_START + 1], // Vec<i16>,
    pc: i32,
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_load() {
        let mut vm = VirtualMachine::new();
        vm.load(&[
            Command::Push(Segment::CONSTANT, 8),
            Command::Push(Segment::CONSTANT, 7),
            Command::Arithmetic(Operator::ADD),
        ]);

        assert_eq!(vm.memory[SP], STACK_START as i16);
    }
}