mod utils;
mod web;
mod web_vm;

use js_sys;
use std::rc::Rc;
use std::cell::RefCell;
extern crate web_sys;

use wasm_bindgen::{JsCast};

use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
    #[wasm_bindgen(constructor)]
    pub fn new(container: JsValue) -> JackVmPlayer {
        log!("version 0.1.8.2");
        let js_global = web::JsGlobal::create().expect("Can't initialize JS global environment.");
        let vm = Rc::new(RefCell::new(web_vm::JackVirtualMachine::new(container)));

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

    #[wasm_bindgen(js_name = isStopped)]
    pub fn is_stopped(&self) -> bool {
        self.vm.borrow().is_stopped()
    }

    pub fn restart(&mut self) {
        {
            let mut vm = self.vm.borrow_mut();
            // needed by vm restart logic.
            vm.set_is_paused(true);
            vm.restart();
        }

        self.run();
    }

    fn run(&mut self) {
        log!("running from RUST!");
        if !self.vm.borrow().is_paused() {
            return;
        }

        self.vm.borrow_mut().set_is_paused(false);
        self.vm.borrow_mut().copy_screen();

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
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jackvm-wasm!");
}
