use vm::vm;
// pub mod compiler;
mod utils;
mod web;
// mod jack_os;
// pub mod vm;
pub mod web_vm;

use js_sys;
use std::rc::Rc;
use std::cell::RefCell;
extern crate web_sys;

use web_sys::{ Window, Document, HtmlElement, HtmlCanvasElement, CanvasRenderingContext2d };
use wasm_bindgen::{Clamped, JsCast};

use wasm_bindgen::prelude::*;
// // A macro to provide `println!(..)`-style syntax for `console.log` logging.

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


const TICKS_PER_STEP: u32 = 40000;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct JackVmPlayer {
    js_global: web::JsGlobal,
    vm: Rc<RefCell<web_vm::JackVirtualMachine>>,
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
impl JackVmPlayer {
    pub fn new(screen: JsValue, container: JsValue) -> JackVmPlayer {
        let js_global = web::JsGlobal::create().expect("Can't initialize JS global environment.");
        let vm = Rc::new(RefCell::new(web_vm::JackVirtualMachine::new(screen, container)));

        {
            let vm = Rc::clone(&vm);
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                vm.borrow_mut().handle_key_down(event);
            }) as Box<dyn FnMut(_)>);

            js_global.document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("add event listener");
            closure.forget();
        }

        {
            let vm = Rc::clone(&vm);
            let closure = Closure::wrap(Box::new(move |_: JsValue| {
                vm.borrow_mut().handle_key_up();
            }) as Box<dyn FnMut(_)>);

            js_global.document.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).expect("add event listener");
            closure.forget();
        }

        JackVmPlayer {
            js_global,
            vm: Rc::clone(&vm),
        }
    }

    pub fn load(&mut self, program: &str) -> () {
        self.vm.borrow_mut().load(program)
    }

    #[wasm_bindgen(js_name = addHaltListener)]
    pub fn add_halt_listener(&mut self, f: js_sys::Function) {
        self.vm.borrow_mut().add_halt_listener(f);
    }

    #[wasm_bindgen(js_name = handleHalt)]
    pub fn handle_halt(&mut self) {
        self.vm.borrow_mut().handle_halt();
    }

    #[wasm_bindgen(js_name = isHalted)]
    pub fn is_halted(&self) -> bool {
        self.vm.borrow().is_halted()
    }

    // needed?
    pub fn pause(&mut self) -> () {
        self.vm.borrow_mut().pause();
    }
    // handleHalt

    #[wasm_bindgen(js_name = isStopped)]
    pub fn is_stopped(&self) -> bool {
        self.vm.borrow().is_stopped()
    }

    #[wasm_bindgen(js_name = isPaused)]
    pub fn is_paused(&self) -> bool {
        self.vm.borrow().is_paused()
    }

    #[wasm_bindgen(js_name = setIsPaused)]
    pub fn set_is_paused(&mut self, paused: bool) {
        self.vm.borrow_mut().set_is_paused(paused)
    }

    pub fn restart(&mut self) {
        self.vm.borrow_mut().restart()
    }

    #[wasm_bindgen(js_name = copyScreen)]
    pub fn copy_screen(&mut self) {
        self.vm.borrow_mut().copy_screen()
    }

    #[wasm_bindgen(js_name = nextFrame)]
    pub fn next_frame(&mut self) -> () {
        self.vm.borrow_mut().next_frame()

        // let vm = self.vm.borrow_mut();
        // if !vm.is_stopped() {
        //     self.js_global.window.request_animation_frame(this.nextFrame.bind(this));
        // } else {
        //     vm.handle_halt();
        // }

        // vm.next_frame();
    }

    pub fn run(&mut self) {
        log!("running from RUST!");
        if !self.vm.borrow().is_paused() {
            return;
        }

        self.vm.borrow_mut().set_is_paused(false);
        self.copy_screen();

        self.start_animation_loop();
    }

    // Sets up an animation loop. The structure for this was take from
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    fn start_animation_loop(&mut self) {
        // setup animation loop.
        let vm = Rc::clone(&self.vm);

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if vm.borrow().is_stopped() {
                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                vm.borrow_mut().handle_halt();
                return;
            }

            vm.borrow_mut().next_frame();

            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    #[wasm_bindgen(js_name = handleKeyDown)]
    pub fn handle_key_down(&mut self, e: JsValue) {
        self.vm.borrow_mut().handle_key_down(e)
    }

    #[wasm_bindgen(js_name = handleKeyUp)]
    pub fn handle_key_up(&mut self) {
        self.vm.borrow_mut().handle_key_up()
    }

//     setIsPaused
    // load
//    isStopped
//    restart
//    addHaltListener
//    handleHalt
//    isPaused
//    copyScreen
    // nextFrame
    // handleKeyDown
    // handleKeyUp
}


// #[wasm_bindgen]
// pub struct CompilationResult {
//     pub succeeded: bool,
//     errors: Vec<String>,
// }

// #[wasm_bindgen]
// impl CompilationResult {
//     pub fn get_errors(&self) -> js_sys::Array {
//         let arr = js_sys::Array::new_with_length(self.errors.len() as u32);
//         for (i, s) in self.errors.iter().enumerate() {
//             arr.set(i as u32, JsValue::from_str(s));
//         }
//         arr
//     }
// }

// #[wasm_bindgen]
// pub struct JackVirtualMachine {
//     jack_vm: vm::VirtualMachine,
//     canvas: HtmlCanvasElement,
//     // screen_buffer: js_sys::ArrayBuffer,
//     screen_bytes: Box<[u8; 512 * 256 * 4]>,
//     image_data: web_sys::ImageData,
//     main_context: CanvasRenderingContext2d,
//     screen_pixels: js_sys::Uint32Array,
//     paused: bool,
//     halt_listeners: Vec<js_sys::Function>,

//     callback_machine: Rc<RefCell<Option<JackVirtualMachine>>>,
// }

// #[wasm_bindgen]
// impl JackVirtualMachine {
//     pub fn new(screen: JsValue, container: JsValue) -> JackVirtualMachine {
//         utils::set_panic_hook();

//         let container: HtmlElement = container
//             .dyn_into::<HtmlElement>()
//             .expect("Can't create container");

//         let js_global = web::JsGlobal::create().expect("Can't create JS global environment");

//         let canvas = web::create_canvas(&js_global.document, 256, 512).expect("can't create canvas");
//         container.append_child(&canvas).expect("Can't append canvas to parent");

//         // let screen_buffer = js_sys::ArrayBuffer::new(256 * 512 * 4);
//         // let screen_bytes = js_sys::Uint8Array::new_with_byte_offset_and_length(&screen_buffer, 0, 256 * 512 * 4);
//         let mut screen_bytes = Box::new([0; 256*512*4]);
//         let image_data = web_sys::ImageData::new_with_u8_clamped_array(Clamped(screen_bytes.as_mut()), 512).expect("Can't create image data");

//         let main_context = canvas
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<web_sys::CanvasRenderingContext2d>()
//             .unwrap();

//         let callback_machine = Rc::new(RefCell::new(None));

//         let player = JackVirtualMachine {
//             jack_vm: vm::VirtualMachine::new(),
//             canvas,
//             // screen_buffer,
//             screen_bytes,
//             image_data,
//             main_context,
//             screen_pixels: js_sys::Uint32Array::new_with_byte_offset_and_length(
//                 &screen,
//                 0,
//                 512 * 256,
//             ),
//             paused: true,
//             halt_listeners: vec!(),
//             callback_machine: Rc::clone(&callback_machine),
//         };

//         // *callback_machine.borrow_mut() = Some(player);

//         {
//             // let context = context.clone();
//             // let pressed = pressed.clone();
//             // let my_callback_machine = callback_machine.clone();
//             let closure = Closure::wrap(Box::new(move |event: JsValue| {
//                 // my_callback_machine.borrow_mut().unwrap().handle_key_down(event);

//                 log!("clicked! ");
//                 // context.begin_path();
//                 // context.move_to(event.offset_x() as f64, event.offset_y() as f64);
//                 // pressed.set(true);
//             }) as Box<dyn FnMut(_)>);

//             js_global.document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).expect("add event listener");
//             closure.forget();
//         }

//         // let a = Closure::wrap(Box::new(move |e: &JsValue| {
//         //     log!("clicked!");
//         // }) as Box<dyn FnMut()>);

//         // js_global.document.add_event_listener_with_callback("keydown", a.as_ref().unchecked_ref()).expect("add event listener");

//         // a.forget();

//         player
//     }

//     #[wasm_bindgen(js_name = loadRaw)]
//     pub fn load_raw(&mut self, program: &str) -> CompilationResult {
//         self.render_screen();
//         match self.jack_vm.compile_and_load(program) {
//             Err(errors) => {
//                 let messages: Vec<String> = errors
//                     .iter()
//                     .map(|e| format!("{}: {}", e.line_number, e.get_message()))
//                     .collect();
//                 CompilationResult {
//                     succeeded: false,
//                     errors: messages,
//                 }
//             }
//             _ => CompilationResult {
//                 succeeded: true,
//                 errors: vec![],
//             },
//         }
//     }

//     pub fn load(&mut self, program: &str) -> () {
//         let result = self.load_raw(program);
//         if !result.succeeded {
//             let mut message = String::from("JackVmPlayer could not load program due to the following errors:\n\n");
//             for e in result.errors {
//                 message.push_str(&e);
//                 message.push_str("\n");
//             }
//             alert(&message);
//         }
//     }

//     pub fn restart(&mut self) {
//         self.jack_vm.restart()
//     }

//     pub fn get_instruction(&self) -> String {
//         self.jack_vm.get_instruction()
//     }

//     #[wasm_bindgen(js_name = copyScreen)]
//     pub fn copy_screen(&mut self) {
//         // this.imageData.data.set(this.screenBytes);
//         self.image_data = web_sys::ImageData::new_with_u8_clamped_array(Clamped(self.screen_bytes.as_mut()), 512).expect("Can't create image data");
//         self.main_context.put_image_data(&self.image_data, 0.0, 0.0).expect("Can't put image data");
//     }

//     pub fn render_screen(&mut self) {
//         // for y in 0..256 {
//         //     for x in 0..512 {
//         //         let loc = 512 * y + x;
//         //         match (x, y) {
//         //             (x, y) if x % 2 == 0 && y % 2 == 0 =>
//         //                 self.screen_pixels.set_index(loc , 0xFF000000),
//         //             (x, y) if x % 2 == 1 && y % 2 == 0 =>
//         //                 self.screen_pixels.set_index(loc , 0xFFFFFFFF),
//         //             (x, y) if x % 2 == 1 && y % 2 == 0 =>
//         //                 self.screen_pixels.set_index(loc , 0xFFFFFFFF),
//         //             (x, y) if x % 2 == 1 && y % 2 == 1 =>
//         //                 self.screen_pixels.set_index(loc , 0xFF000000),
//         //             _ => (),
//         //         }
//         //     }
//         //     // count += 4;
//         // }
//         // if self.jack_vm.is_halted() {
//         //     return;
//         // }

//         let screen = &mut self.jack_vm.memory[vm::SCREEN_START..vm::KEYBOARD_START + 1];
//         /*
//         // Simple JackVM instructions for drawing a space invader alien sprite drawn at (0, 0)
//         push constant 16384
//         pop pointer 1

//         push constant 1040
//         pop that 0

//         push constant 544
//         pop that 32

//         push constant 2032
//         pop that 64

//         push constant 3544
//         pop that 96

//         push constant 8188
//         pop that 128

//         push constant 8188
//         pop that 160

//         push constant 6132
//         pop that 192

//         push constant 5140
//         pop that 224

//         push constant 864
//         pop that 256
//         */

//         // bytes representing a space invader alien sprite drawn at (0, 0)
//         // screen[0] = 1040;
//         // screen[32] = 544;
//         // screen[64] = 2032;
//         // screen[96] = 3544;
//         // screen[128] = 8188;
//         // screen[160] = 8188;
//         // screen[192] = 6132;
//         // screen[224] = 5140;
//         // screen[256] = 864;
//         // screen[256] = 864;
//         // screen[288] = 0;
//         // screen[320] = -1;

//         // screen.fill(0xFFFFFFFF);
//         // for i in 0..screen.len() {
//         //     screen[i] = 0;
//         // }

//         let screen_bytes = self.screen_bytes.as_mut();
//         for y in 0..256 {
//             for x in 0..32 {
//                 let i = 32 * y + x;
//                 // if screen[i] != 0 {
//                 //   log!("slot {} is {}; x = {}, y = {}", i, screen[i], x, y);
//                 let mut value = screen[i];
//                 for j in 0..16 {
//                     let loc = ((512 * y) + (16 * x) + j) as u32;
//                     if (value & 0x1) == 1 {
//                         // log!("writing x = {}, y = {} at loc = {}", (16 * x) + j, y, loc);
//                         // self.screen_pixels.set_index(loc, 0xFF000000);
//                         screen_bytes[loc as usize * 4] = 0x00;
//                         screen_bytes[loc as usize * 4 + 1] = 0x00;
//                         screen_bytes[loc as usize * 4 + 2] = 0x00;
//                         screen_bytes[loc as usize * 4 + 3] = 0xFF;
//                     } else {
//                         // TODO: consider drawing white pixels
//                         // self.screen_pixels.set_index(loc, 0xFFFFFFFF);
//                         screen_bytes[loc as usize * 4] = 0xFF;
//                         screen_bytes[loc as usize * 4 + 1] = 0xFF;
//                         screen_bytes[loc as usize * 4 + 2] = 0xFF;
//                         screen_bytes[loc as usize * 4 + 3] = 0xFF;
//                     }

//                     value = value >> 1;
//                 }
//                 // } else {

//                 // }
//             }
//         }
//         // log!("wrote {} bytes", count);
//     }

//     pub fn tick(&mut self) -> () {
//         self.jack_vm.tick();
//     }

//     #[wasm_bindgen(js_name = addHaltListener)]
//     pub fn add_halt_listener(&mut self, f: js_sys::Function) {
//         self.halt_listeners.push(f);
//     }

//     #[wasm_bindgen(js_name = handleHalt)]
//     pub fn handle_halt(&mut self) {
//         let this = JsValue::null();
//         for f in &self.halt_listeners {
//             f.call0(&this).expect("Can't call handler");
//         }
//     }

//     #[wasm_bindgen(js_name = isHalted)]
//     pub fn is_halted(&self) -> bool {
//         self.jack_vm.is_halted()
//     }

//     pub fn pause(&mut self) -> () {
//         self.paused = true;
//     }

//     #[wasm_bindgen(js_name = isStopped)]
//     pub fn is_stopped(&self) -> bool {
//         self.paused || self.jack_vm.is_halted()
//     }

//     #[wasm_bindgen(js_name = isPaused)]
//     pub fn is_paused(&self) -> bool {
//         self.paused
//     }

//     #[wasm_bindgen(js_name = setIsPaused)]
//     pub fn set_is_paused(&mut self, paused: bool) {
//         self.paused = paused;
//     }

//     #[wasm_bindgen(js_name = executeSteps)]
//     pub fn execute_steps(&mut self) -> () {
//         self.tick_times(TICKS_PER_STEP)
//     }

//     #[wasm_bindgen(js_name = nextFrame)]
//     pub fn next_frame(&mut self) -> () {
//         self.execute_steps();
//         self.render_screen();
//         self.copy_screen();
//     }

//     pub fn tick_times(&mut self, times: u32) -> () {
//         for _ in 0..times {
//             self.jack_vm.tick();
//         }
//     }

//     #[wasm_bindgen(js_name = handleKeyDown)]
//     pub fn handle_key_down(&mut self, e: JsValue) {
//         let mut key_code = js_sys::Reflect::get(&e, &JsValue::from_str("keyCode")).unwrap().as_f64().expect("Expected keyCode present on event") as i16;
//         if key_code == 37 {
//             key_code = 130;
//         }
//         if key_code == 39 {
//             key_code = 132;
//         }
//         self.set_key(key_code);
//     }

//     #[wasm_bindgen(js_name = handleKeyUp)]
//     pub fn handle_key_up(&mut self) {
//         self.set_key(0);
//     }

//     pub fn set_key(&mut self, key: i16) {
//         self.jack_vm.poke(vm::KEYBOARD_START, key);
//     }

//     pub fn peek(&self, address: usize) -> i16 {
//         self.jack_vm.peek(address)
//     }
// }

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jackvm-wasm!");
}
