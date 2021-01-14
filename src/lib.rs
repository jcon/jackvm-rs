mod utils;
pub mod compiler;
pub mod vm;

use js_sys;

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
pub struct CompilationResult {
    pub succeeded: bool,
    errors: Vec<String>,
}

#[wasm_bindgen]
impl CompilationResult {
    pub fn get_errors(&self) -> js_sys::Array {
        let arr = js_sys::Array::new_with_length(self.errors.len() as u32);
        for (i, s) in self.errors.iter().enumerate() {
            arr.set(i as u32, JsValue::from_str(s));
        }
        arr
    }
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
            jack_vm: vm::VirtualMachine::new(),
        }
    }

    pub fn load(&mut self, program: &str) -> CompilationResult {
        match self.jack_vm.compile_and_load(program) {
            Err(errors) => {
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.line_number, e.message))
                    .collect();
                CompilationResult {
                    succeeded: false,
                    errors: messages,
                }
            },
            _ => CompilationResult {
                succeeded: true,
                errors: vec!(),
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
