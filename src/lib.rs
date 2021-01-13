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
struct JackVirtualMachine {
    jack_vm: vm::VirtualMachine,
}

#[wasm_bindgen]
impl JackVirtualMachine {
    pub fn new() -> JackVirtualMachine {
        JackVirtualMachine {
            jack_vm: vm::VirtualMachine::new()
        }
    }

    // TODO: this should be an error.
    pub fn load(&mut self, program: &str) -> () {
        match compiler::compile(program) {
            Ok(bytecode) => self.jack_vm.load(&bytecode[..]),
            Err(errors) => {
                let compiler::CompilationError { message, .. } = errors[0];
                alert(message);
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
