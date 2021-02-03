use vm::vm;
// pub mod compiler;
mod utils;
// mod jack_os;
// pub mod vm;

use js_sys;
extern crate web_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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
    screen_pixels: js_sys::Uint32Array,
}

#[wasm_bindgen]
impl JackVirtualMachine {
    pub fn new(screen: JsValue) -> JackVirtualMachine {
        utils::set_panic_hook();

        JackVirtualMachine {
            jack_vm: vm::VirtualMachine::new(),
            screen_pixels: js_sys::Uint32Array::new_with_byte_offset_and_length(
                &screen,
                0,
                512 * 256,
            ),
        }
    }

    pub fn load(&mut self, program: &str) -> CompilationResult {
        self.render_screen();
        match self.jack_vm.compile_and_load(program) {
            Err(errors) => {
                let messages: Vec<String> = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.line_number, e.get_message()))
                    .collect();
                CompilationResult {
                    succeeded: false,
                    errors: messages,
                }
            }
            _ => CompilationResult {
                succeeded: true,
                errors: vec![],
            },
        }
    }

    pub fn restart(&mut self) {
        self.jack_vm.restart()
    }

    pub fn get_instruction(&self) -> String {
        self.jack_vm.get_instruction()
    }

    pub fn render_screen(&mut self) {
        // for y in 0..256 {
        //     for x in 0..512 {
        //         let loc = 512 * y + x;
        //         match (x, y) {
        //             (x, y) if x % 2 == 0 && y % 2 == 0 =>
        //                 self.screen_pixels.set_index(loc , 0xFF000000),
        //             (x, y) if x % 2 == 1 && y % 2 == 0 =>
        //                 self.screen_pixels.set_index(loc , 0xFFFFFFFF),
        //             (x, y) if x % 2 == 1 && y % 2 == 0 =>
        //                 self.screen_pixels.set_index(loc , 0xFFFFFFFF),
        //             (x, y) if x % 2 == 1 && y % 2 == 1 =>
        //                 self.screen_pixels.set_index(loc , 0xFF000000),
        //             _ => (),
        //         }
        //     }
        //     // count += 4;
        // }
        // if self.jack_vm.is_halted() {
        //     return;
        // }

        let screen = &mut self.jack_vm.memory[vm::SCREEN_START..vm::KEYBOARD_START + 1];
        /*
        // Simple JackVM instructions for drawing a space invader alien sprite drawn at (0, 0)
        push constant 16384
        pop pointer 1

        push constant 1040
        pop that 0

        push constant 544
        pop that 32

        push constant 2032
        pop that 64

        push constant 3544
        pop that 96

        push constant 8188
        pop that 128

        push constant 8188
        pop that 160

        push constant 6132
        pop that 192

        push constant 5140
        pop that 224

        push constant 864
        pop that 256
        */

        // bytes representing a space invader alien sprite drawn at (0, 0)
        // screen[0] = 1040;
        // screen[32] = 544;
        // screen[64] = 2032;
        // screen[96] = 3544;
        // screen[128] = 8188;
        // screen[160] = 8188;
        // screen[192] = 6132;
        // screen[224] = 5140;
        // screen[256] = 864;
        // screen[256] = 864;
        // screen[288] = 0;
        // screen[320] = -1;

        // screen.fill(0xFFFFFFFF);
        // for i in 0..screen.len() {
        //     screen[i] = 0;
        // }

        for y in 0..256 {
            for x in 0..32 {
                let i = 32 * y + x;
                // if screen[i] != 0 {
                //   log!("slot {} is {}; x = {}, y = {}", i, screen[i], x, y);
                let mut value = screen[i];
                for j in 0..16 {
                    let loc = ((512 * y) + (16 * x) + j) as u32;
                    if (value & 0x1) == 1 {
                        //    log!("writing x = {}, y = {} at loc = {}", (16 * x) + j, y, loc);
                        self.screen_pixels.set_index(loc, 0xFF000000);
                    } else {
                        // TODO: consider drawing white pixels
                        self.screen_pixels.set_index(loc, 0xFFFFFFFF);
                    }

                    value = value >> 1;
                }
                // } else {

                // }
            }
        }
        // log!("wrote {} bytes", count);
    }

    pub fn tick(&mut self) -> () {
        self.jack_vm.tick();
    }

    #[wasm_bindgen(js_name = isHalted)]
    pub fn is_halted(&self) -> bool {
        self.jack_vm.is_halted()
    }

    pub fn tick_times(&mut self, times: u32) -> () {
        for _ in 0..times {
            self.jack_vm.tick();
        }
    }

    pub fn set_key(&mut self, key: i16) {
        self.jack_vm.poke(vm::KEYBOARD_START, key);
    }

    pub fn peek(&self, address: usize) -> i16 {
        self.jack_vm.peek(address)
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jackvm-wasm!");
}
