mod helper;

#[cfg(test)]
mod test {
    use super::*;
    use helper::compile_program;
    use jackvm_rs::vm;
    use std::panic;

    #[test]
    pub fn test_os_array() {
        let mut jack_vm = compile_program(
            "
            function Main.main 4
            push constant 8000
            pop local 0
            push constant 3
            call Array.new 1
            pop local 1
            push constant 2
            push local 1
            add
            push constant 222
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 0
            push local 0
            add
            push constant 2
            push local 1
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 3
            call Array.new 1
            pop local 2
            push constant 1
            push local 2
            add
            push constant 2
            push local 1
            add
            pop pointer 1
            push that 0
            push constant 100
            sub
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 1
            push local 0
            add
            push constant 1
            push local 2
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 500
            call Array.new 1
            pop local 3
            push constant 499
            push local 3
            add
            push constant 2
            push local 1
            add
            pop pointer 1
            push that 0
            push constant 1
            push local 2
            add
            pop pointer 1
            push that 0
            sub
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 2
            push local 0
            add
            push constant 499
            push local 3
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 1
            call Array.dispose 1
            pop temp 0
            push local 2
            call Array.dispose 1
            pop temp 0
            push constant 3
            call Array.new 1
            pop local 2
            push constant 0
            push local 2
            add
            push constant 499
            push local 3
            add
            pop pointer 1
            push that 0
            push constant 90
            sub
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 3
            push local 0
            add
            push constant 0
            push local 2
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 3
            call Array.dispose 1
            pop temp 0
            push local 2
            call Array.dispose 1
            pop temp 0
            push constant 0
            return
            function Array.new 0
            push argument 0
            push constant 0
            gt
            not
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 2
            call Sys.error 1
            pop temp 0
            label IF_FALSE0
            push argument 0
            call Memory.alloc 1
            return
            function Array.dispose 0
            push argument 0
            pop pointer 0
            push pointer 0
            call Memory.deAlloc 1
            pop temp 0
            push constant 0
            return
            function Memory.init 0
            push constant 0
            pop static 0
            push constant 2048
            push static 0
            add
            push constant 14334
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 2049
            push static 0
            add
            push constant 2050
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 0
            return
            function Memory.peek 0
            push argument 0
            push static 0
            add
            pop pointer 1
            push that 0
            return
            function Memory.poke 0
            push argument 0
            push static 0
            add
            push argument 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 0
            return
            function Memory.alloc 1
            push argument 0
            push constant 1
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 5
            call Sys.error 1
            pop temp 0
            label IF_FALSE0
            push constant 2048
            pop local 0
            label WHILE_EXP0
            push constant 0
            push local 0
            add
            pop pointer 1
            push that 0
            push argument 0
            lt
            not
            if-goto WHILE_END0
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            pop local 0
            goto WHILE_EXP0
            label WHILE_END0
            push local 0
            push argument 0
            add
            push constant 16379
            gt
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push constant 6
            call Sys.error 1
            pop temp 0
            label IF_FALSE1
            push constant 0
            push local 0
            add
            pop pointer 1
            push that 0
            push argument 0
            push constant 2
            add
            gt
            if-goto IF_TRUE2
            goto IF_FALSE2
            label IF_TRUE2
            push argument 0
            push constant 2
            add
            push local 0
            add
            push constant 0
            push local 0
            add
            pop pointer 1
            push that 0
            push argument 0
            sub
            push constant 2
            sub
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            push local 0
            push constant 2
            add
            eq
            if-goto IF_TRUE3
            goto IF_FALSE3
            label IF_TRUE3
            push argument 0
            push constant 3
            add
            push local 0
            add
            push local 0
            push argument 0
            add
            push constant 4
            add
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            goto IF_END3
            label IF_FALSE3
            push argument 0
            push constant 3
            add
            push local 0
            add
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            label IF_END3
            push constant 1
            push local 0
            add
            push local 0
            push argument 0
            add
            push constant 2
            add
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            label IF_FALSE2
            push constant 0
            push local 0
            add
            push constant 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 0
            push constant 2
            add
            return
            function Memory.deAlloc 2
            push argument 0
            push constant 2
            sub
            pop local 0
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            pop local 1
            push constant 0
            push local 1
            add
            pop pointer 1
            push that 0
            push constant 0
            eq
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            push local 0
            add
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            push local 0
            sub
            push constant 2
            sub
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            goto IF_END0
            label IF_FALSE0
            push constant 0
            push local 0
            add
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            push local 0
            sub
            push constant 0
            push local 1
            add
            pop pointer 1
            push that 0
            add
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 1
            push local 1
            add
            pop pointer 1
            push that 0
            push local 1
            push constant 2
            add
            eq
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push constant 1
            push local 0
            add
            push local 0
            push constant 2
            add
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            goto IF_END1
            label IF_FALSE1
            push constant 1
            push local 0
            add
            push constant 1
            push local 1
            add
            pop pointer 1
            push that 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            label IF_END1
            label IF_END0
            push constant 0
            return
            function Sys.init 0
            call Memory.init 0
            pop temp 0
            call Main.main 0
            pop temp 0
            call Sys.halt 0
            pop temp 0
            push constant 0
            return
            function Sys.halt 0
            label WHILE_EXP0
            push constant 0
            not
            not
            if-goto WHILE_END0
            goto WHILE_EXP0
            label WHILE_END0
            push constant 0
            return
            function Sys.wait 1
            push argument 0
            push constant 0
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 1
            call Sys.error 1
            pop temp 0
            label IF_FALSE0
            label WHILE_EXP0
            push argument 0
            push constant 0
            gt
            not
            if-goto WHILE_END0
            push constant 50
            pop local 0
            label WHILE_EXP1
            push local 0
            push constant 0
            gt
            not
            if-goto WHILE_END1
            push local 0
            push constant 1
            sub
            pop local 0
            goto WHILE_EXP1
            label WHILE_END1
            push argument 0
            push constant 1
            sub
            pop argument 0
            goto WHILE_EXP0
            label WHILE_END0
            push constant 0
            return
            function Sys.error 0
            push constant 3
            call String.new 1
            push constant 69
            call String.appendChar 2
            push constant 82
            call String.appendChar 2
            push constant 82
            call String.appendChar 2
            call Output.printString 1
            pop temp 0
            push argument 0
            call Output.printInt 1
            pop temp 0
            call Sys.halt 0
            pop temp 0
            push constant 0
            return
        ",
        );

        for _ in 0..1000000 {
            jack_vm.tick();
        }

        helper::debug_stack(&jack_vm);

        assert_eq!(jack_vm.peek(8000), 222);
        assert_eq!(jack_vm.peek(8001), 122);
        assert_eq!(jack_vm.peek(8002), 100);
        assert_eq!(jack_vm.peek(8003), 10);
    }
}
