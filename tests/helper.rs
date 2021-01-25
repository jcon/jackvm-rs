use jackvm_rs::vm;

pub fn compile_program(program: &str) -> vm::VirtualMachine {
    let mut jack_vm = vm::VirtualMachine::new();
    let compile_result = jack_vm.compile_and_load(program);

    match compile_result {
        Err(errors) => {
            // TODO: figure out how to return array of string errors to JS.
            let messages: Vec<String> = errors
                .iter()
                .map(|e| format!("{}: {}", e.line_number, e.get_message()))
                .collect();
            let empty_vec: Vec<String> = vec![];
            assert_eq!(empty_vec, messages);
            // println!("{}", &messages.join("\n"));
        }
        _ => (),
    };

    jack_vm
}

pub fn execute_program(program: &str) -> vm::VirtualMachine {
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

pub fn debug_stack(jack_vm: &vm::VirtualMachine) {
    let mut s = String::from("");
    for i in 0..5 {
        s.push_str(&i.to_string());
        s.push_str(": ");
        s.push_str(&jack_vm.peek(i).to_string());
        s.push_str("\n");
    }
    for i in 16..21 {
        s.push_str(&i.to_string());
        s.push_str(": ");
        s.push_str(&jack_vm.peek(i).to_string());
        s.push_str("\n");
    }
    for i in 256..310 {
        //s.push_str(format!("{}: {}\n", i, jack_vm.peek(i)));
        s.push_str(&i.to_string());
        s.push_str(": ");
        s.push_str(&jack_vm.peek(i).to_string());
        s.push_str("\n");
    }
    for i in 8000..8020 {
        //s.push_str(format!("{}: {}\n", i, jack_vm.peek(i)));
        s.push_str(&i.to_string());
        s.push_str(": ");
        s.push_str(&jack_vm.peek(i).to_string());
        s.push_str("\n");
    }
    println!("memory ****\n{}", s);
}

pub fn debug_address(jack_vm: &vm::VirtualMachine, symbol: &str) {
    println!(
        "address of {} is {}",
        symbol,
        jack_vm.addresses.get(symbol).unwrap()
    );
}
