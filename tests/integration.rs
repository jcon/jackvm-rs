#[cfg(test)]
mod test {
    use jackvm_wasm::vm;

    pub fn execute_program(program: &str) -> vm::VirtualMachine {
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
}