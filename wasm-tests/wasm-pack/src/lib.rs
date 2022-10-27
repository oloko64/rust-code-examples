mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Exposes the "alert" from JavaScript to the Rust code, enabling the Rust code to call the "alert" function.
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// A auto generated example that shows how to create a wasm_bindgen function that calls the alert from JavaScript.
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-pack!");
}
