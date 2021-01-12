pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

extern crate assert_matches;
use assert_matches::assert_matches;

extern crate jackvm_wasm;
// use jackvm_wasm::compiler::Parser;
use jackvm_wasm::compiler::Command;
use jackvm_wasm::compiler::Segment;
use jackvm_wasm::compiler::Operator;
use jackvm_wasm::compiler::compile;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_segment_match() {
        let ct = Command::Push(Segment::LOCAL, 1);
        assert_matches!(ct, Command::Push(Segment::LOCAL, 1));
    }

    #[test]
    fn test_simple_add() {
        let source = "push constant 5\npush constant 4\nadd";
        let prog = compile(&source[..]);
        assert_eq!(vec!(
            Command::Push(Segment::CONSTANT, 5),
            Command::Push(Segment::CONSTANT, 4),
            Command::Operate(Operator::ADD),
        ), prog);
    }
}