mod helper;

#[cfg(test)]
mod test {
    use super::*;
    use helper::compile_program;
    use jackvm_rs::vm;
    use std::panic;

    #[test]
    pub fn test_os_memory() {
        let mut jack_vm = compile_program(
            "
            function Main.allocateBlocks 4
            push argument 0
            call Array.new 1
            pop local 0
            push constant 0
            pop local 2
            label WHILE_EXP0
            push local 2
            push argument 0
            lt
            not
            if-goto WHILE_END0
            push argument 1
            call Array.new 1
            pop local 1
            push local 2
            push local 0
            add
            push local 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 0
            pop local 3
            label WHILE_EXP1
            push local 3
            push argument 1
            lt
            not
            if-goto WHILE_END1
            push local 3
            push local 1
            add
            push local 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 3
            push constant 1
            add
            pop local 3
            goto WHILE_EXP1
            label WHILE_END1
            push local 2
            push constant 1
            add
            pop local 2
            goto WHILE_EXP0
            label WHILE_END0
            push argument 2
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            pop local 2
            label WHILE_EXP2
            push local 2
            push argument 0
            lt
            not
            if-goto WHILE_END2
            push local 2
            push local 0
            add
            pop pointer 1
            push that 0
            pop local 1
            push local 1
            call Array.dispose 1
            pop temp 0
            push local 2
            push constant 1
            add
            pop local 2
            goto WHILE_EXP2
            label WHILE_END2
            push local 0
            call Array.dispose 1
            pop temp 0
            label IF_FALSE0
            push local 0
            return
            function Main.main0 5
            push constant 80
            pop local 4
            push local 4
            push constant 49
            push constant 0
            not
            call Main.allocateBlocks 3
            pop temp 0
            push local 4
            push constant 69
            push constant 0
            not
            call Main.allocateBlocks 3
            pop temp 0
            push local 4
            push constant 2
            call Math.divide 2
            push constant 79
            push constant 0
            call Main.allocateBlocks 3
            pop local 0
            push constant 0
            pop local 2
            label WHILE_EXP0
            push local 2
            push local 4
            push constant 2
            call Math.divide 2
            lt
            not
            if-goto WHILE_END0
            push local 2
            push local 0
            add
            pop pointer 1
            push that 0
            pop local 1
            push constant 0
            pop local 3
            label WHILE_EXP1
            push local 3
            push constant 79
            lt
            not
            if-goto WHILE_END1
            push local 3
            push local 1
            add
            pop pointer 1
            push that 0
            push local 2
            eq
            not
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 19
            call String.new 1
            push constant 78
            call String.appendChar 2
            push constant 111
            call String.appendChar 2
            push constant 32
            call String.appendChar 2
            push constant 109
            call String.appendChar 2
            push constant 97
            call String.appendChar 2
            push constant 116
            call String.appendChar 2
            push constant 99
            call String.appendChar 2
            push constant 104
            call String.appendChar 2
            push constant 32
            call String.appendChar 2
            push constant 102
            call String.appendChar 2
            push constant 111
            call String.appendChar 2
            push constant 114
            call String.appendChar 2
            push constant 32
            call String.appendChar 2
            push constant 98
            call String.appendChar 2
            push constant 108
            call String.appendChar 2
            push constant 111
            call String.appendChar 2
            push constant 99
            call String.appendChar 2
            push constant 107
            call String.appendChar 2
            push constant 32
            call String.appendChar 2
            call Output.printString 1
            pop temp 0
            push local 3
            call Output.printInt 1
            pop temp 0
            push constant 1
            call String.new 1
            push constant 32
            call String.appendChar 2
            call Output.printString 1
            pop temp 0
            push local 3
            push local 1
            add
            pop pointer 1
            push that 0
            call Output.printInt 1
            pop temp 0
            push constant 4
            call String.new 1
            push constant 32
            call String.appendChar 2
            push constant 33
            call String.appendChar 2
            push constant 61
            call String.appendChar 2
            push constant 32
            call String.appendChar 2
            call Output.printString 1
            pop temp 0
            push local 2
            call Output.printInt 1
            pop temp 0
            call Sys.halt 0
            pop temp 0
            label IF_FALSE0
            push local 3
            push local 1
            add
            push local 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 3
            push constant 1
            add
            pop local 3
            goto WHILE_EXP1
            label WHILE_END1
            push local 2
            push constant 1
            add
            pop local 2
            goto WHILE_EXP0
            label WHILE_END0
            push constant 8
            call String.new 1
            push constant 70
            call String.appendChar 2
            push constant 105
            call String.appendChar 2
            push constant 110
            call String.appendChar 2
            push constant 105
            call String.appendChar 2
            push constant 115
            call String.appendChar 2
            push constant 104
            call String.appendChar 2
            push constant 101
            call String.appendChar 2
            push constant 100
            call String.appendChar 2
            call Output.printString 1
            pop temp 0
            push constant 10000
            call Sys.wait 1
            pop temp 0
            push constant 0
            return
            function Main.main 4
            push constant 8000
            push constant 333
            call Memory.poke 2
            pop temp 0
            push constant 8000
            call Memory.peek 1
            pop local 0
            push constant 8001
            push local 0
            push constant 1
            add
            call Memory.poke 2
            pop temp 0
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
            push constant 8002
            push constant 2
            push local 1
            add
            pop pointer 1
            push that 0
            call Memory.poke 2
            pop temp 0
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
            push constant 8003
            push constant 1
            push local 2
            add
            pop pointer 1
            push that 0
            call Memory.poke 2
            pop temp 0
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
            push constant 8004
            push constant 499
            push local 3
            add
            pop pointer 1
            push that 0
            call Memory.poke 2
            pop temp 0
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
            push constant 8005
            push constant 0
            push local 2
            add
            pop pointer 1
            push that 0
            call Memory.poke 2
            pop temp 0
            push local 3
            call Array.dispose 1
            pop temp 0
            push local 2
            call Array.dispose 1
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

        assert_eq!(jack_vm.peek(8000), 333);
        assert_eq!(jack_vm.peek(8001), 334);
        assert_eq!(jack_vm.peek(8002), 222);
        assert_eq!(jack_vm.peek(8003), 122);
        assert_eq!(jack_vm.peek(8004), 100);
        assert_eq!(jack_vm.peek(8005), 10);
    }

    // fn debug_stack(jack_vm: &vm::VirtualMachine) {
    //     let mut s = String::from("");
    //     for i in 0..5 {
    //         s.push_str(&i.to_string());
    //         s.push_str(": ");
    //         s.push_str(&jack_vm.peek(i).to_string());
    //         s.push_str("\n");
    //     }
    //     for i in 256..310 {
    //         //s.push_str(format!("{}: {}\n", i, jack_vm.peek(i)));
    //         s.push_str(&i.to_string());
    //         s.push_str(": ");
    //         s.push_str(&jack_vm.peek(i).to_string());
    //         s.push_str("\n");
    //     }
    //     println!("memory ****\n{}", s);
    // }

    // fn debug_address(jack_vm: &vm::VirtualMachine, symbol: &str) {
    //     println!("address of {} is {}", symbol, jack_vm.addresses.get(symbol).unwrap());
    // }
}
