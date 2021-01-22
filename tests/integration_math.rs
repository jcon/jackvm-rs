mod helper;

#[cfg(test)]
mod test {
    use super::*;
    use jackvm_wasm::vm;
    use std::panic;
    use helper::compile_program;

    #[test]
    pub fn test_os_math() {
        let mut jack_vm = compile_program("
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
            function Main.mod 1
            push constant 8000
            pop local 0
            push constant 0
            push local 0
            add
            push argument 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 1
            push local 0
            add
            push argument 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push argument 0
            push argument 0
            push argument 1
            call Math.divide 2
            push argument 1
            call Math.multiply 2
            sub
            return
            function Main.main 1
            push constant 8000
            pop local 0
            push constant 0
            push local 0
            add
            push constant 2
            push constant 3
            call Math.multiply 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 1
            push local 0
            add
            push constant 0
            push local 0
            add
            pop pointer 1
            push that 0
            push constant 30
            neg
            call Math.multiply 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 2
            push local 0
            add
            push constant 1
            push local 0
            add
            pop pointer 1
            push that 0
            push constant 100
            call Math.multiply 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 3
            push local 0
            add
            push constant 1
            push constant 2
            push local 0
            add
            pop pointer 1
            push that 0
            call Math.multiply 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 4
            push local 0
            add
            push constant 3
            push local 0
            add
            pop pointer 1
            push that 0
            push constant 0
            call Math.multiply 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 5
            push local 0
            add
            push constant 9
            push constant 3
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 6
            push local 0
            add
            push constant 18000
            neg
            push constant 6
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 7
            push local 0
            add
            push constant 32766
            push constant 32767
            neg
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 8
            push local 0
            add
            push constant 9
            call Math.sqrt 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 9
            push local 0
            add
            push constant 32767
            call Math.sqrt 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 10
            push local 0
            add
            push constant 345
            push constant 123
            call Math.min 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 11
            push local 0
            add
            push constant 123
            push constant 345
            neg
            call Math.max 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 12
            push local 0
            add
            push constant 27
            call Math.abs 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 13
            push local 0
            add
            push constant 32767
            neg
            call Math.abs 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 15
            push local 0
            add
            push constant 100
            push constant 7
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 16
            push local 0
            add
            push constant 200
            push constant 400
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 17
            push local 0
            add
            push constant 64
            push constant 4
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 18
            push local 0
            add
            push constant 50
            push constant 7
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push constant 19
            push local 0
            add
            push constant 700
            push constant 99
            call Math.divide 2
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
//            push constant 15
//            push local 0
//            add
//            push constant 128
//            push constant 32767
//            neg
//            push constant 1
//            sub
//            call Math.divide 2
//            pop temp 0
//            pop pointer 1
//            push temp 0
//            pop that 0
            push constant 0
            return
            function Math.init 2
            push constant 7000
            pop static 3
            push constant 0
            pop static 4
            push constant 16
            call Array.new 1
            pop static 2
            push constant 0
            pop local 0
            push constant 1
            pop local 1
            label WHILE_EXP0
            push local 0
            push constant 16
            lt
            not
            if-goto WHILE_END0
            push local 0
            push static 2
            add
            push local 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push local 1
            push local 1
            add
            pop local 1
            push local 0
            push constant 1
            add
            pop local 0
            goto WHILE_EXP0
            label WHILE_END0
            push constant 3
            pop static 0
            push constant 4
            pop static 1
            push constant 0
            return
            function Math.abs 0
            push argument 0
            push constant 0
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            push argument 0
            sub
            return
            goto IF_END0
            label IF_FALSE0
            push argument 0
            return
            label IF_END0
            function Math.multiply 4
            push argument 0
            pop local 1
            push constant 1
            pop local 2
            push constant 0
            pop local 3
            label WHILE_EXP0
            push local 3
            push constant 16
            lt
            not
            if-goto WHILE_END0
            push argument 1
            push local 2
            and
            push local 2
            eq
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push local 0
            push local 1
            add
            pop local 0
            label IF_FALSE0
            push local 1
            push local 1
            add
            pop local 1
            push local 2
            push local 2
            add
            pop local 2
            push local 3
            push constant 1
            add
            pop local 3
            goto WHILE_EXP0
            label WHILE_END0
            push local 0
            return
            function Math.divide 1
            push argument 0
            call Math.abs 1
            push argument 1
            call Math.abs 1
            lt
            push argument 0
            push argument 1
            eq
            not
            push argument 1
            push constant 32767
            neg
            push constant 1
            sub
            eq
            and
            or
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            return
            label IF_FALSE0
            push argument 0
            call Math.abs 1
            push argument 1
            call Math.abs 1
            call Math.divideHelper 2
            pop local 0
            push argument 0
            push constant 0
            lt
            push argument 1
            push constant 0
            gt
            and
            push argument 1
            push constant 0
            lt
            push argument 0
            push constant 0
            gt
            and
            or
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push constant 0
            push local 0
            sub
            return
            goto IF_END1
            label IF_FALSE1
            push local 0
            return
            label IF_END1
            function Math.divideHelper 2
            push static 4
            push static 3
            add
            push argument 0
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push static 4
            push constant 1
            add
            push static 3
            add
            push argument 1
            pop temp 0
            pop pointer 1
            push temp 0
            pop that 0
            push static 4
            push constant 2
            add
            pop static 4
            push argument 1
            push constant 0
            lt
            push argument 1
            push argument 0
            gt
            or
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            return
            label IF_FALSE0
            push argument 1
            push constant 0
            eq
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push static 0
            call Sys.error 1
            pop temp 0
            push constant 0
            return
            label IF_FALSE1
            push argument 0
            push argument 1
            push argument 1
            add
            call Math.divideHelper 2
            pop local 0
            push local 0
            push argument 1
            call Math.multiply 2
            pop local 1
            push argument 0
            push local 1
            push local 1
            add
            sub
            push argument 1
            lt
            if-goto IF_TRUE2
            goto IF_FALSE2
            label IF_TRUE2
            push local 0
            push local 0
            add
            return
            goto IF_END2
            label IF_FALSE2
            push local 0
            push local 0
            add
            push constant 1
            add
            return
            label IF_END2
            function Math.sqrt 4
            push argument 0
            push constant 0
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push static 1
            call Sys.error 1
            pop temp 0
            push constant 0
            return
            label IF_FALSE0
            push constant 16
            push constant 2
            call Math.divide 2
            push constant 1
            sub
            pop local 0
            push constant 0
            pop local 1
            label WHILE_EXP0
            push local 0
            push constant 1
            neg
            gt
            not
            if-goto WHILE_END0
            push local 1
            push local 0
            push static 2
            add
            pop pointer 1
            push that 0
            add
            pop local 2
            push local 2
            push local 2
            call Math.multiply 2
            pop local 3
            push local 3
            push constant 0
            lt
            not
            push local 3
            push argument 0
            gt
            not
            and
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push local 2
            pop local 1
            label IF_FALSE1
            push local 0
            push constant 1
            sub
            pop local 0
            goto WHILE_EXP0
            label WHILE_END0
            push local 1
            return
            function Math.max 0
            push argument 0
            push argument 1
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push argument 1
            return
            goto IF_END0
            label IF_FALSE0
            push argument 0
            return
            label IF_END0
            function Math.min 0
            push argument 0
            push argument 1
            lt
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push argument 0
            return
            goto IF_END0
            label IF_FALSE0
            push argument 1
            return
            label IF_END0
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
            call Math.init 0
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
        ");

        for _ in 0..1000000 {
            jack_vm.tick();
        }

        helper::debug_stack(&jack_vm);

        /*
|RAM[8000]|RAM[8001]|RAM[8002]|RAM[8003]|RAM[8004]|RAM[8005]|RAM[8006]|RAM[8007]|RAM[8008]|RAM[8009]|RAM[8010]|RAM[8011]|RAM[8012]|RAM[8013]|
|       6 |    -180 |  -18000 |  -18000 |       0 |       3 |   -3000 |       0 |       3 |     181 |     123 |     123 |      27 |   32767 |
*/


        assert_eq!(jack_vm.peek(8000), 6);
        assert_eq!(jack_vm.peek(8001), -180);
        assert_eq!(jack_vm.peek(8002), -18000);
        assert_eq!(jack_vm.peek(8003), -18000);
        assert_eq!(jack_vm.peek(8004), 0);
        assert_eq!(jack_vm.peek(8005), 3);
        assert_eq!(jack_vm.peek(8006), -3000);
        assert_eq!(jack_vm.peek(8007), 0);
        assert_eq!(jack_vm.peek(8008), 3);
        assert_eq!(jack_vm.peek(8009), 181);
        assert_eq!(jack_vm.peek(8010), 123);
        assert_eq!(jack_vm.peek(8011), 123);
        assert_eq!(jack_vm.peek(8012), 27);
        assert_eq!(jack_vm.peek(8013), 32767);
        assert_eq!(jack_vm.peek(8015), 100i16 / 7i16);
        assert_eq!(jack_vm.peek(8016), 200i16 / 400i16);
        assert_eq!(jack_vm.peek(8017), 64i16 / 4i16);
        assert_eq!(jack_vm.peek(8018), 50i16 / 7i16);
        assert_eq!(jack_vm.peek(8019), 700i16 / 99i16);
        assert_eq!(jack_vm.peek(8020), 0);
    }
}
