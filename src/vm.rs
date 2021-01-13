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
const TEMP_START: usize = 5;
const STATIC_START: usize = 16;
const STACK_START: usize = 256;
const _HEAP_START: usize = 2048;
const _SCREEN_START: usize = 16384;
const KEYBOARD_START: usize = 24575;

const VM_TRUE: i16 = -1;
const VM_FALSE: i16 = 0;

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
            Command::Pop(segment, arg2) => self.stack_pop_segment(segment, arg2 as i16),
            Command::Arithmetic(operator) => self.process_arithmetic(operator),
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