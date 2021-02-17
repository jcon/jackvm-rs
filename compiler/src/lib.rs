mod utils;
pub mod tokens;

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
pub fn greet() {
    alert("Hello, compiler!");
}


#[wasm_bindgen]
pub fn compile(prog: &str) -> String {
    let result = tokens::tokenize(prog);
    if result.len() == 0 {
        "Failed to compile".to_string()
    } else {
        "Succeeded".to_string()
    }
}
