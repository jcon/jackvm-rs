extern crate web_sys;

use std::collections::HashMap;

use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlElement};

use crate::utils;
use crate::web;
use vm::vm;

const TICKS_PER_STEP: u32 = 40000;

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

pub struct CompilationResult {
    pub succeeded: bool,
    errors: Vec<String>,
}

pub struct JackVirtualMachine {
    jack_vm: vm::VirtualMachine,
    screen_bytes: Box<[u8; 512 * 256 * 4]>,
    image_data: web_sys::ImageData,
    main_context: CanvasRenderingContext2d,
    paused: bool,
    halt_listeners: Vec<js_sys::Function>,

    special_keys: HashMap<i16, i16>,
}

impl JackVirtualMachine {
    pub fn new(container: JsValue) -> JackVirtualMachine {
        utils::set_panic_hook();

        let container: HtmlElement = container
            .dyn_into::<HtmlElement>()
            .expect("Can't create container");

        let js_global = web::JsGlobal::create().expect("Can't create JS global environment");

        let canvas =
            web::create_canvas(&js_global.document, 256, 512).expect("can't create canvas");
        container
            .append_child(&canvas)
            .expect("Can't append canvas to parent");

        let mut screen_bytes = Box::new([0; 256 * 512 * 4]);
        let image_data =
            web_sys::ImageData::new_with_u8_clamped_array(Clamped(screen_bytes.as_mut()), 512)
                .expect("Can't create image data");

        let main_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let player = JackVirtualMachine {
            jack_vm: vm::VirtualMachine::new(),
            screen_bytes,
            image_data,
            main_context,
            paused: true,
            halt_listeners: vec![],
            special_keys: setup_special_keys(),
        };

        player
    }

    pub fn load_raw(&mut self, program: &str) -> CompilationResult {
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

    pub fn load(&mut self, program: &str) -> () {
        let result = self.load_raw(program);
        if !result.succeeded {
            let mut message = String::from(
                "JackVmPlayer could not load program due to the following errors:\n\n",
            );
            for e in result.errors {
                message.push_str(&e);
                message.push_str("\n");
            }
            alert(&message);
        }
    }

    pub fn restart(&mut self) {
        self.jack_vm.restart()
    }

    pub fn copy_screen(&mut self) {
        self.image_data =
            web_sys::ImageData::new_with_u8_clamped_array(Clamped(self.screen_bytes.as_mut()), 512)
                .expect("Can't create image data");
        self.main_context
            .put_image_data(&self.image_data, 0.0, 0.0)
            .expect("Can't put image data");
    }

    pub fn render_screen(&mut self) {
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

        let screen_bytes = self.screen_bytes.as_mut();
        for y in 0..256 {
            for x in 0..32 {
                let i = 32 * y + x;
                let mut value = screen[i];
                for j in 0..16 {
                    let loc = ((512 * y) + (16 * x) + j) as u32;
                    if (value & 0x1) == 1 {
                        screen_bytes[loc as usize * 4] = 0x00;
                        screen_bytes[loc as usize * 4 + 1] = 0x00;
                        screen_bytes[loc as usize * 4 + 2] = 0x00;
                        screen_bytes[loc as usize * 4 + 3] = 0xFF;
                    } else {
                        screen_bytes[loc as usize * 4] = 0xFF;
                        screen_bytes[loc as usize * 4 + 1] = 0xFF;
                        screen_bytes[loc as usize * 4 + 2] = 0xFF;
                        screen_bytes[loc as usize * 4 + 3] = 0xFF;
                    }

                    value = value >> 1;
                }
            }
        }
    }

    pub fn add_halt_listener(&mut self, f: js_sys::Function) {
        self.halt_listeners.push(f);
    }

    pub fn handle_halt(&mut self) {
        let this = JsValue::null();
        for f in &self.halt_listeners {
            f.call0(&this).expect("Can't call handler");
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.paused || self.jack_vm.is_halted()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_is_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn execute_steps(&mut self) -> () {
        self.tick_times(TICKS_PER_STEP)
    }

    pub fn next_frame(&mut self) -> () {
        self.execute_steps();
        self.render_screen();
        self.copy_screen();
    }

    pub fn tick_times(&mut self, times: u32) -> () {
        for _ in 0..times {
            self.jack_vm.tick();
        }
    }

    pub fn handle_key_down(&mut self, e: JsValue) {
        let mut key_code = js_sys::Reflect::get(&e, &JsValue::from_str("keyCode"))
            .unwrap()
            .as_f64()
            .expect("Expected keyCode present on event") as i16;
        // Override keys that have different keymappings between JS <=> Hack.
        if self.special_keys.contains_key(&key_code) {
            key_code = *self.special_keys.get(&key_code).unwrap();
        }
        self.set_key(key_code);
    }

    pub fn handle_key_up(&mut self) {
        self.set_key(0);
    }

    pub fn set_key(&mut self, key: i16) {
        self.jack_vm.poke(vm::KEYBOARD_START, key);
    }

    pub fn peek(&self, address: usize) -> i16 {
        self.jack_vm.peek(address)
    }
}

fn setup_special_keys() -> HashMap<i16, i16> {
    let mut special_keys: HashMap<i16, i16> = [
            (13, 128), // newline / return
            (8, 129),  // backspace
            (37, 130), // left arrow
            (38, 131), // up arrow
            (39, 132), // right arrow
            (40, 133), // down arrow
            (36, 134), // home
            (35, 135), // end
            (33, 136), // page up
            (45, 138), // page up
            (46, 139), // delete
            (27, 140), // escape
        ]
        .iter()
        .copied()
        .collect();
    // Set keys f1 .. f12
    for i in 0..12 {
        // In JS they're 112..123, in Jack, they're 141..152
        special_keys.insert(112 + i, 141 + i);
    }

    special_keys
}
