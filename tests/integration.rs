mod helper;

#[cfg(test)]
mod test {
    use super::*;
    use std::panic;
    use helper::{compile_program, execute_program, debug_stack, debug_address};

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
            pop local 0	        // sum = sum + counter
            push argument 0
            push constant 1
            sub
            pop argument 0      // counter--
            push argument 0
            if-goto LOOP_START  // If counter > 0, goto LOOP_START
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

    #[test]
    pub fn test_function_calls_simple_function() {
        let mut jack_vm = compile_program("
            // Performs a simple calculation and returns the result.
            function SimpleFunction.test 2
            push local 0
            push local 1
            add
            not
            push argument 0
            add
            push argument 1
            sub
            return
        ");
        /*
set sp 317,
set local 317,
set argument 310,
set this 3000,
set that 4000,
set argument[0] 1234,
set argument[1] 37,
set argument[2] 9,
set argument[3] 305,
set argument[4] 300,
set argument[5] 3010,
set argument[6] 4010,
*/

        jack_vm.poke(0, 317);
        jack_vm.poke(1, 317);
        jack_vm.poke(2, 310);
        jack_vm.poke(3, 3000);
        jack_vm.poke(4, 4000);
        jack_vm.poke(310, 1234);
        jack_vm.poke(311, 37);
        jack_vm.poke(312, 9);
        jack_vm.poke(313, 305);
        jack_vm.poke(314, 300);
        jack_vm.poke(315, 3010);
        jack_vm.poke(316, 4010);

        for _ in 0..10 {
            jack_vm.tick();
        }
/*
| RAM[0] | RAM[1] | RAM[2] | RAM[3] | RAM[4] |RAM[310]|
|    311 |    305 |    300 |   3010 |   4010 |   1196 |
*/
        // assert_eq!(jack_vm.peek(317), 0);
        // assert_eq!(jack_vm.peek(318), 0);
        // assert_eq!(jack_vm.peek(318), 0);
        // // assert_eq!(jack_vm.peek(319), -1);
        // // assert_eq!(jack_vm.peek(319), 1233); // after 7
        // assert_eq!(jack_vm.peek(319), 1196); // after 9
        // // assert_eq!(jack_vm.peek(320), 1234);
        // assert_eq!(jack_vm.peek(320), 37); // after 8
        // assert_eq!(jack_vm.peek(320), 37); // after 8

        // println!("MEMORY******\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        //     jack_vm.peek(310),
        //     jack_vm.peek(311),
        //     jack_vm.peek(312),
        //     jack_vm.peek(313),
        //     jack_vm.peek(314),
        //     jack_vm.peek(315),
        //     jack_vm.peek(316),
        //     jack_vm.peek(317),
        //     jack_vm.peek(318),
        //     jack_vm.peek(319),
        //     jack_vm.peek(320),
        //     jack_vm.peek(321),
        // );

        assert_eq!(jack_vm.peek(0), 311);
        assert_eq!(jack_vm.peek(1), 305);
        assert_eq!(jack_vm.peek(2), 300);
        assert_eq!(jack_vm.peek(3), 3010);
        assert_eq!(jack_vm.peek(4), 4010);
        assert_eq!(jack_vm.peek(310), 1196);
    }

    #[test]
    pub fn test_memory_access_static_addresses() {
        let jack_vm = execute_program("
            function Test1.main 0
            push constant 2
            push constant 1
            pop static 0
            pop static 1
            return

            function Test2.main 0
            push constant 4
            push constant 3
            pop static 0
            pop static 1
            return

            function Sys.init 0
            call Test1.main 0
            pop temp 0
            call Test2.main 0
            pop temp 0
            ");

        println!("
        16 => {}\n
        17 => {}\n
        18 => {}\n
        19 => {}\n
        ",
        jack_vm.peek(16),
        jack_vm.peek(17),
        jack_vm.peek(18),
        jack_vm.peek(19));

        if jack_vm.peek(16) == 1 {
            assert_eq!(jack_vm.peek(16), 1);
            assert_eq!(jack_vm.peek(17), 2);
            assert_eq!(jack_vm.peek(18), 3);
            assert_eq!(jack_vm.peek(19), 4);
        } else {
            assert_eq!(jack_vm.peek(16), 3);
            assert_eq!(jack_vm.peek(17), 4);
            assert_eq!(jack_vm.peek(18), 1);
            assert_eq!(jack_vm.peek(19), 2);

        }
    }

    #[test]
    pub fn test_function_calls_simple_function_from_sys_init() {
        let mut jack_vm = compile_program("
            call Sys.init 0 // dummy to align with Java implementation
            function Main.test 2
            push argument 1
            push argument 0
            pop local 0
            pop local 1
            push argument 0
            push argument 1
            add
            not
            push argument 0
            add
            push argument 1
            sub
            return
            function Sys.init 1
            push constant 4
            push constant 9
            call Main.test 2
            label WHILE
            goto WHILE              // loops infinitely
        ");


        for _ in 0..20{
            jack_vm.tick();
        }

        debug_stack(&jack_vm);

        assert_eq!(jack_vm.peek(0), 258);
        assert_eq!(jack_vm.peek(256), 0);
        assert_eq!(jack_vm.peek(257), -19);
        assert_eq!(jack_vm.peek(258), 9);
        assert_eq!(jack_vm.peek(259), 19);
        assert_eq!(jack_vm.peek(260), 0);
        assert_eq!(jack_vm.peek(261), 0);
        assert_eq!(jack_vm.peek(262), 0);
        assert_eq!(jack_vm.peek(263), 0);
        assert_eq!(jack_vm.peek(264), 4);
        assert_eq!(jack_vm.peek(265), 9);
        assert_eq!(jack_vm.peek(266), -19);
        assert_eq!(jack_vm.peek(267), 9);
        assert_eq!(jack_vm.peek(268), 0);
        assert_eq!(jack_vm.peek(269), 0);
        assert_eq!(jack_vm.peek(270), 0);
    }

    #[test]
    pub fn test_function_call_multiple_args() {
        let mut jack_vm = compile_program("
            call Sys.init 0 // dummy to align with Java implementation
            function Main.test 0
            push argument 0
            pop static 0
            push argument 1
            pop static 1
            push argument 2
            pop static 2
            return

            function Main.main 0
            push constant 9
            push constant 8
            push constant 7
            call Main.test 3
            pop temp 0
            return

            function Sys.init 1
            call Main.main 0
            pop temp 0

            label WHILE
            goto WHILE              // loops infinitely
        ");


        for _ in 0..50{
            jack_vm.tick();
        }

        debug_stack(&jack_vm);

        assert_eq!(jack_vm.peek(16), 9);
        assert_eq!(jack_vm.peek(17), 8);
        assert_eq!(jack_vm.peek(18), 7);
    }

    #[test]
    pub fn test_logic_in_function_called_from_sys_init() {
        let mut jack_vm = compile_program("
            call Sys.init 0 // dummy to align with Java implementation
            function Main.test 4
            push constant 4
            pop local 2
            push local 2
            push argument 0
            gt
            not
            push local 2
            push constant 0
            lt
            not
            and
            if-goto IF_TRUE1
            goto IF_FALSE1
            label IF_TRUE1
            push constant 1
            pop static 0
            goto END_IF1
            label IF_FALSE1
            push constant 2
            pop static 0
            label END_IF1
            return

            function Main.main 0
            push constant 9
            call Main.test 1
            pop temp 0
            push static 0
            pop static 1

            push constant 3
            call Main.test 1
            pop temp 0
            push static 0
            pop static 2
            return

            function Sys.init 1
            call Main.main 0
            pop temp 0

            label WHILE
            goto WHILE              // loops infinitely
        ");


        for _ in 0..50{
            jack_vm.tick();
        }

        debug_stack(&jack_vm);

        assert_eq!(jack_vm.peek(17), 1);
        assert_eq!(jack_vm.peek(18), 2);
    }

    #[test]
    pub fn test_function_calls_nested_call() {
        let mut jack_vm = compile_program("
            // Sys.vm for NestedCall test.

            // Sys.init()
            //
            // Calls Sys.main() and stores return value in temp 1.
            // Does not return.  (Enters infinite loop.)

            function Sys.init 0
            push constant 4000	// test THIS and THAT context save
            pop pointer 0
            push constant 5000
            pop pointer 1
            call Sys.main 0
            pop temp 1
            label LOOP
            goto LOOP

            // Sys.main()
            //
            // Sets locals 1, 2 and 3, leaving locals 0 and 4 unchanged to test
            // default local initialization to 0.  (RAM set to -1 by test setup.)
            // Calls Sys.add12(123) and stores return value (135) in temp 0.
            // Returns local 0 + local 1 + local 2 + local 3 + local 4 (456) to confirm
            // that locals were not mangled by function call.

            function Sys.main 5
            push constant 4001
            pop pointer 0
            push constant 5001
            pop pointer 1
            push constant 200
            pop local 1
            push constant 40
            pop local 2
            push constant 6
            pop local 3
            push constant 123
            call Sys.add12 1
            pop temp 0
            push local 0
            push local 1
            push local 2
            push local 3
            push local 4
            add
            add
            add
            add
            return

            // Sys.add12(int n)
            //
            // Returns n+12.

            function Sys.add12 0
            push constant 4002
            pop pointer 0
            push constant 5002
            pop pointer 1
            push argument 0
            push constant 12
            add
            return
        ");
        jack_vm.poke(0, 261);


        jack_vm.poke(0, 261);
        jack_vm.poke(1, 261);
        jack_vm.poke(2, 256);
        jack_vm.poke(3, -3);
        jack_vm.poke(4, -4);
        jack_vm.poke(5, -1); // test results
        jack_vm.poke(6, -1);
        jack_vm.poke(256, 1234); // fake stack frame from call Sys.init
        jack_vm.poke(257, -1);
        jack_vm.poke(258, -2);
        jack_vm.poke(259, -3);
        jack_vm.poke(260, -4);

        jack_vm.poke(261, -1); // Initialize stack to check for local segment
        jack_vm.poke(262, -1); // being cleared to zero.
        jack_vm.poke(263, -1);
        jack_vm.poke(264, -1);
        jack_vm.poke(265, -1);
        jack_vm.poke(266, -1);
        jack_vm.poke(267, -1);
        jack_vm.poke(268, -1);
        jack_vm.poke(269, -1);
        jack_vm.poke(270, -1);
        jack_vm.poke(271, -1);
        jack_vm.poke(272, -1);
        jack_vm.poke(273, -1);
        jack_vm.poke(274, -1);
        jack_vm.poke(275, -1);
        jack_vm.poke(276, -1);
        jack_vm.poke(277, -1);
        jack_vm.poke(278, -1);
        jack_vm.poke(279, -1);
        jack_vm.poke(280, -1);
        jack_vm.poke(281, -1);
        jack_vm.poke(282, -1);
        jack_vm.poke(283, -1);
        jack_vm.poke(284, -1);
        jack_vm.poke(285, -1);
        jack_vm.poke(286, -1);
        jack_vm.poke(287, -1);
        jack_vm.poke(288, -1);
        jack_vm.poke(289, -1);
        jack_vm.poke(290, -1);
        jack_vm.poke(291, -1);
        jack_vm.poke(292, -1);
        jack_vm.poke(293, -1);
        jack_vm.poke(294, -1);
        jack_vm.poke(295, -1);
        jack_vm.poke(296, -1);
        jack_vm.poke(297, -1);
        jack_vm.poke(298, -1);
        jack_vm.poke(299, -1);

        jack_vm.poke(0, 261);
        jack_vm.poke(1, 261);
        jack_vm.poke(2, 256);
        jack_vm.poke(3, 3000);
        jack_vm.poke(4, 4000);

        for _ in 0..50 {
            jack_vm.tick();
        }

        assert_eq!(jack_vm.peek(0), 261);
        assert_eq!(jack_vm.peek(1), 261);
        assert_eq!(jack_vm.peek(2), 256);
        assert_eq!(jack_vm.peek(3), 4000);
        assert_eq!(jack_vm.peek(4), 5000);
        assert_eq!(jack_vm.peek(5), 135);
        assert_eq!(jack_vm.peek(6), 246);
    }

    #[test]
    pub fn test_function_calls_fibonacci_element() {
        let mut jack_vm = compile_program("
            // Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
            // function, which computes the n'th element of the Fibonacci series.
            // Note that by convention, the Sys.init function is called \"automatically\"
            // by the bootstrap code.

            function Sys.init 0
            push constant 4
            call Main.fibonacci 1   // computes the 4'th fibonacci element
            label WHILE
            goto WHILE              // loops infinitely

            // Computes the n'th element of the Fibonacci series, recursively.
            // n is given in argument[0].  Called by the Sys.init function
            // (part of the Sys.vm file), which also pushes the argument[0]
            // parameter before this code starts running.

            function Main.fibonacci 0
            push argument 0
            push constant 2
            lt                     // checks if n<2
            if-goto IF_TRUE
            goto IF_FALSE
            label IF_TRUE          // if n<2, return n
            push argument 0
            return
            label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
            push argument 0
            push constant 2
            sub
            call Main.fibonacci 1  // computes fib(n-2)
            push argument 0
            push constant 1
            sub
            call Main.fibonacci 1  // computes fib(n-1)
            add                    // returns fib(n-1) + fib(n-2)
            return
        ");

        jack_vm.poke(0, 261);

        for _ in 0..175 {
            jack_vm.tick();
        }

        assert_eq!(jack_vm.peek(0), 262);
        assert_eq!(jack_vm.peek(261), 3);
    }

    #[test]
    pub fn test_function_calls_fibonacci_element_2() {
        let mut jack_vm = compile_program("
            // Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
            // function, which computes the n'th element of the Fibonacci series.
            // Note that by convention, the Sys.init function is called \"automatically\"
            // by the bootstrap code.

            function Sys.init 0
            push constant 4
            call Main.fibonacci 1   // computes the 4'th fibonacci element
            label WHILE
            goto WHILE              // loops infinitely

            // Computes the n'th element of the Fibonacci series, recursively.
            // n is given in argument[0].  Called by the Sys.init function
            // (part of the Sys.vm file), which also pushes the argument[0]
            // parameter before this code starts running.

            function Main.fibonacci 0
            push argument 0
            push constant 2
            lt                     // checks if n<2
            if-goto IF_TRUE
            goto IF_FALSE
            label IF_TRUE          // if n<2, return n
            push argument 0
            return
            label IF_FALSE         // if n>=2, returns fib(n-2)+fib(n-1)
            push argument 0
            push constant 2
            sub
            call Main.fibonacci 1  // computes fib(n-2)
            push argument 0
            push constant 1
            sub
            call Main.fibonacci 1  // computes fib(n-1)
            add                    // returns fib(n-1) + fib(n-2)
            return
        ");

        jack_vm.poke(0, 261);

        for _ in 0..250{
            jack_vm.tick();
        }


        let mut s = String::from("");
        for i in 261..300 {
            //s.push_str(format!("{}: {}\n", i, jack_vm.peek(i)));
	    s.push_str(&i.to_string());
	    s.push_str(": ");
	    s.push_str(&jack_vm.peek(i).to_string());
	    s.push_str("\n");
        }
        println!("memory ****\n{}", s);

        // assert_eq!(jack_vm.peek(0), 262);
        // assert_eq!(jack_vm.peek(261), 3);
    }

    #[test]
    pub fn test_function_calls_recursive_add() {
        let mut jack_vm = compile_program("
            // Pushes a constant, say n, onto the stack, and calls the Main.fibonacii
            // function, which computes the n'th element of the Fibonacci series.
            // Note that by convention, the Sys.init function is called \"automatically\"
            // by the bootstrap code.
            call Sys.init 0 // dummy

            function Main.main 1
            push constant 1
            call Main.add 1
            pop local 0
            label WHILE_EXP0
            push constant 0
            not
            not
            if-goto WHILE_END0
            goto WHILE_EXP0
            label WHILE_END0
            push constant 0
            return
            function Main.add 0
            push argument 0
            push constant 0
            eq
            if-goto IF_TRUE0
            goto IF_FALSE0
            label IF_TRUE0
            push constant 0
            return
            goto IF_END0
            label IF_FALSE0
            push constant 1
            push argument 0
            push constant 1
            sub
            call Main.add 1
            add
            return
            label IF_END0

            function Sys.init 0
            //push constant 4
            call Main.main 0   // computes the 4'th fibonacci element
            label WHILE
            goto WHILE              // loops infinitely
        ");

        jack_vm.poke(0, 256);

        debug_address(&jack_vm, "Sys.init");
        debug_address(&jack_vm, "Main.main");
        debug_address(&jack_vm, "Main.add");

        for _ in 0..102 {
            jack_vm.tick();
        }

        debug_stack(&jack_vm);

        // assert_eq!(jack_vm.peek(0), 262);
        // assert_eq!(jack_vm.peek(261), 3);
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
