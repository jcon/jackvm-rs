mod utils;
mod compiler;
mod vm;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct JackVirtualMachine {
    jack_vm: vm::VirtualMachine,
}

#[wasm_bindgen]
impl JackVirtualMachine {
    pub fn new() -> JackVirtualMachine {
        utils::set_panic_hook();

        JackVirtualMachine {
            jack_vm: vm::VirtualMachine::new()
        }
    }

    // TODO: this should communicate errors.
    pub fn load(&mut self, program: &str) -> () {
        match compiler::compile(program) {
            Ok(bytecode) => {
                self.jack_vm.load(&bytecode[..]);
            },
            Err(errors) => {
                // TODO: figure out how to return array of string errors to JS.
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.line_number, e.message))
                    .collect();
                alert(&messages.join("\n"));
            },
        }
    }

    pub fn tick(&mut self) -> () {
        self.jack_vm.tick();
    }

    pub fn peek(&self, address: usize) -> i16 {
        self.jack_vm.peek(address)
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jackvm-wasm!");
}
