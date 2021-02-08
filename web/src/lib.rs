mod utils;
mod web;
mod web_vm;

use js_sys;
use std::cell::RefCell;
use std::rc::Rc;
extern crate web_sys;

use wasm_bindgen::JsCast;

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

#[wasm_bindgen]
impl JackVmPlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(container: JsValue, options: &JsValue) -> JackVmPlayer {
        log!("version 0.1.8.2");
        let options: web_vm::Options = if options.is_undefined() {
            web_vm::Options {
               on_color: 0x000000ff,
               off_color: 0xffffffff,
            }
        } else {
            options.into_serde().unwrap()
        };

        let js_global = web::JsGlobal::create().expect("Can't initialize JS global environment.");
        let vm = Rc::new(RefCell::new(web_vm::JackVirtualMachine::new(container, options)));

        {
            let vm = Rc::clone(&vm);
            let closure = Closure::wrap(Box::new(move |event: JsValue| {
                vm.borrow_mut().handle_key_down(event);
            }) as Box<dyn FnMut(_)>);

            js_global
                .document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .expect("add keydown listener");
            closure.forget();
        }

        {
            let vm = Rc::clone(&vm);
            let closure = Closure::wrap(Box::new(move |_: JsValue| {
                vm.borrow_mut().handle_key_up();
            }) as Box<dyn FnMut(_)>);

            js_global
                .document
                .add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())
                .expect("add keyup listener");
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

    pub fn peek(&self, address: usize) -> i16 {
        self.vm.borrow().peek(address)
    }

    fn run(&mut self) {
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
        // Clone VM so we can share it with the callback requestAnimationFrame receives
        let vm = Rc::clone(&self.vm);

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        // This closure borrows the VM instance for each animation frame.
        // This code will be safe as long as the VM is running inside the main thread of the browser.
        // This will be guaranteed because:
        // - JavaScript is single threaded, so two threads cannot access the VM at the same time.
        // - The entry point for all logic for all IO to the VM is excuted within `vm.next_frame`.
        // - The keyboard listener callbacks also happen in the JavaScript event loop, so cannot
        //   happen at the same time as each animation frame.
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
            web::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        web::request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, jackvm-wasm!");
}
