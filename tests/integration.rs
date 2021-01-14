#[cfg(test)]
mod test {
    use jackvm_wasm::vm;

    fn compile_program(program: &str) -> vm::VirtualMachine {
        let mut jack_vm = vm::VirtualMachine::new();
        let compile_result = jack_vm.compile_and_load(program);

        match compile_result {
            Err(errors) => {
                // TODO: figure out how to return array of string errors to JS.
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.line_number, e.get_message()))
                    .collect();
                let empty_vec: Vec<String> = vec!();
                assert_eq!(empty_vec, messages);
                // println!("{}", &messages.join("\n"));
            },
            _ => ()
        };

        jack_vm
    }

    fn execute_program(program: &str) -> vm::VirtualMachine {
        // let mut jack_vm = vm::VirtualMachine::new();
        // let compile_result = jack_vm.compile_and_load(program);

        // match compile_result {
        //     Err(errors) => {
        //         // TODO: figure out how to return array of string errors to JS.
        //         let messages: Vec<String> = errors
        //             .iter()
        //             .map(|e| format!("{}: {}", e.line_number, e.get_message()))
        //             .collect();
        //         let empty_vec: Vec<String> = vec!();
        //         assert_eq!(empty_vec, messages);
        //         // println!("{}", &messages.join("\n"));
        //     },
        //     _ => ()
        // };
        let mut jack_vm = compile_program(program);

        jack_vm.poke(0, 256);
        jack_vm.poke(1, 300);
        jack_vm.poke(2, 400);
        jack_vm.poke(3, 3000);
        jack_vm.poke(4, 3010);

        for _ in 0..100 {
            jack_vm.tick();
        }

        jack_vm
    }

    #[test]
    pub fn test_memory_access_basic_test() {
        let jack_vm = execute_program("
            push constant 10
            pop local 0
            push constant 21
            push constant 22
            pop argument 2
            pop argument 1
            push constant 36
            pop this 6
            push constant 42
            push constant 45
            pop that 5
            pop that 2
            push constant 510
            pop temp 6
            push local 0
            push that 5
            add
            push argument 1
            sub
            push this 6
            push this 6
            add
            sub
            push temp 6
            add");

        assert_eq!(jack_vm.peek(256), 472);
        assert_eq!(jack_vm.peek(300), 10);
        assert_eq!(jack_vm.peek(401), 21);
        assert_eq!(jack_vm.peek(402), 22);
        assert_eq!(jack_vm.peek(3006), 36);
        assert_eq!(jack_vm.peek(3012), 42);
        assert_eq!(jack_vm.peek(3015), 45);
        assert_eq!(jack_vm.peek(11), 510);
    }

    #[test]
    pub fn test_memory_access_pointer_test() {
        let jack_vm = execute_program("
            push constant 3030
            pop pointer 0
            push constant 3040
            pop pointer 1
            push constant 32
            pop this 2
            push constant 46
            pop that 6
            push pointer 0
            push pointer 1
            add
            push this 2
            sub
            push that 6
            add");

        assert_eq!(jack_vm.peek(256), 6084);
        assert_eq!(jack_vm.peek(3), 3030);
        assert_eq!(jack_vm.peek(4), 3040);
        assert_eq!(jack_vm.peek(3032), 32);
        assert_eq!(jack_vm.peek(3046), 46);
    }

    #[test]
    pub fn test_memory_access_static_test() {
        let jack_vm = execute_program("
            push constant 111
            push constant 333
            push constant 888
            pop static 8
            pop static 3
            pop static 1
            push static 3
            push static 1
            sub
            push static 8
            add");

        assert_eq!(jack_vm.peek(256), 1110);
    }

    #[test]
    pub fn test_stack_arithmetic_simple_add() {
        let jack_vm = execute_program("
            push constant 8
            push constant 7
            add");

        assert_eq!(jack_vm.peek(0), 257);
        assert_eq!(jack_vm.peek(256), 15);
    }

    #[test]
    pub fn test_stack_arithmetic_stack_test() {
        let jack_vm = execute_program("
            push constant 17
            push constant 17
            eq
            push constant 17
            push constant 16
            eq
            push constant 16
            push constant 17
            eq
            push constant 892
            push constant 891
            lt
            push constant 891
            push constant 892
            lt
            push constant 891
            push constant 891
            lt
            push constant 32767
            push constant 32766
            gt
            push constant 32766
            push constant 32767
            gt
            push constant 32766
            push constant 32766
            gt
            push constant 57
            push constant 31
            push constant 53
            add
            push constant 112
            sub
            neg
            and
            push constant 82
            or
            not");

        assert_eq!(jack_vm.peek(0), 266);
        assert_eq!(jack_vm.peek(256), -1);
        assert_eq!(jack_vm.peek(257), 0);
        assert_eq!(jack_vm.peek(258), 0);
        assert_eq!(jack_vm.peek(259), 0);
        assert_eq!(jack_vm.peek(260), -1);
        assert_eq!(jack_vm.peek(261), 0);
        assert_eq!(jack_vm.peek(262), -1);
        assert_eq!(jack_vm.peek(263), 0);
        assert_eq!(jack_vm.peek(264), 0);
        assert_eq!(jack_vm.peek(265), -91);
    }

    #[test]
    pub fn test_program_flow_basic_loop() {
        let mut jack_vm = compile_program("
            // Computes the sum 1 + 2 + ... + argument[0] and pushes the
            // result onto the stack. Argument[0] is initialized by the test
            // script before this code starts running.
            push constant 0
            pop local 0         // initializes sum = 0
            label LOOP_START
            push argument 0
            push local 0
            add
            pop local 0        // sum = sum + counter
            push argument 0
            push constant 1
            sub
            pop argument 0     // counter--
            push argument 0
            if-goto LOOP_START // If counter > 0, goto LOOP_START
            push local 0
        ");

        jack_vm.poke(0, 256);
        jack_vm.poke(1, 300);
        jack_vm.poke(2, 400);
        jack_vm.poke(400, 3);

        /*
        | RAM[0] |RAM[256]|
        |    257 |      6 |
        */
        for _ in 0..600 {
            jack_vm.tick();
        }

        assert_eq!(jack_vm.peek(0), 257);
        assert_eq!(jack_vm.peek(256), 6);
    }

    #[test]
    pub fn test_program_flow_fibonacci_series() {
        let mut jack_vm = compile_program("
            // Puts the first argument[0] elements of the Fibonacci series
            // in the memory, starting in the address given in argument[1].
            // Argument[0] and argument[1] are initialized by the test script
            // before this code starts running.

            push argument 1
            pop pointer 1           // that = argument[1]

            push constant 0
            pop that 0              // first element in the series = 0
            push constant 1
            pop that 1              // second element in the series = 1

            push argument 0
            push constant 2
            sub
            pop argument 0          // num_of_elements -= 2 (first 2 elements are set)

            label MAIN_LOOP_START

            push argument 0
            if-goto COMPUTE_ELEMENT // if num_of_elements > 0, goto COMPUTE_ELEMENT
            goto END_PROGRAM        // otherwise, goto END_PROGRAM

            label COMPUTE_ELEMENT

            push that 0
            push that 1
            add
            pop that 2              // that[2] = that[0] + that[1]

            push pointer 1
            push constant 1
            add
            pop pointer 1          // that += 1

            push argument 0
            push constant 1
            sub
            pop argument 0         // num_of_elements--

            goto MAIN_LOOP_START

            label END_PROGRAM
        ");

        jack_vm.poke(0, 256);
        jack_vm.poke(1, 300);
        jack_vm.poke(2, 400);
        jack_vm.poke(400, 6);
        jack_vm.poke(401, 3000);

        for _ in 0..1100 {
            jack_vm.tick();
        }

        assert_eq!(jack_vm.peek(3000), 0);
        assert_eq!(jack_vm.peek(3001), 1);
        assert_eq!(jack_vm.peek(3002), 1);
        assert_eq!(jack_vm.peek(3003), 2);
        assert_eq!(jack_vm.peek(3004), 3);
        assert_eq!(jack_vm.peek(3005), 5);
    }
}