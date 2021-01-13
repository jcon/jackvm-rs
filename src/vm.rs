use crate::compiler::Command;
use crate::compiler::Segment;
use crate::compiler::Operator;

pub struct VirtualMachine {
    memory: [i16; KEYBOARD_START + 1],
    pc: usize,
    program: Vec<Command>,
}

const SP: usize = 0;
const LCL: usize = 1;
const ARG: usize = 2;
const THIS: usize = 3;
const THAT: usize = 4;
const _TEMP_START: usize = 5;
const _STATIC_START: usize = 16;
const STACK_START: usize = 256;
const _HEAP_START: usize = 2048;
const _SCREEN_START: usize = 16384;
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
            Command::Push(segment, arg2) => self.stack_push_segment(segment, arg2 as i16),
            Command::Arithmetic(operator) => self.process_arithmetic(operator),
            _ => println!("un implemented command"),
        };
        self.pc = self.pc + 1;
    }

    pub fn peek(&self, address: usize) -> i16 {
        self.memory[address]
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
            _ => 0
        };
        self.stack_push(val);
    }

    fn stack_pop_segment(&mut self, segment: Segment, offset: i16) -> () {
        let base_address = match segment {
            // TODO: it's an error to "pop" to constant.
//            Segment::CONSTANT => offset,
            Segment::LOCAL => self.memory[LCL],
            Segment::ARG => self.memory[ARG],
            Segment::THIS => self.memory[THIS],
            Segment::THAT => self.memory[THAT],
            _ => 0,
        };
        let address = (base_address + offset) as usize;
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

    fn stack_peek(&self) -> i16 {
        let address = (self.memory[SP] - 1) as usize;
        self.memory[address]
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

        address += 1;
        vm.memory[SP] = address as i16;

        vm.stack_push_segment(Segment::LOCAL, 0);
        vm.stack_push_segment(Segment::ARG, 0);
        vm.stack_push_segment(Segment::THIS, 0);
        vm.stack_push_segment(Segment::THAT, 0);

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

        let nLocal = 2;
        let nArgs = 3;
        let nFields = 2;

        let mut address: usize = 256;
        vm.memory[LCL] = address as i16;
        address += nLocal;

        vm.memory[ARG] = address as i16;
        address += nArgs;

        vm.memory[THIS] = address as i16;
        address += nFields;

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

        assert_eq!(vm.memory[SP], (STACK_START + nLocal + nArgs + nFields + 1) as i16);
    }
}