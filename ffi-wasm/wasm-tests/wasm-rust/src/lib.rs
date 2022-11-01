use wasm_bindgen::prelude::*;

// Exposing the struct to JavaScript using wasm-bindgen.
#[wasm_bindgen]
pub struct Testing {
    sum: String,
    b: Vec<u32>,
}

// For a String it's necessary to create a Trait the implements the wasm-bindgen getter and setter.
// Reference: https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/getter-and-setter.html?highlight=getter#getter-and-setter
#[wasm_bindgen]
impl Testing {
    #[wasm_bindgen(getter)]
    pub fn a(&self) -> String {
        self.sum.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, sum: String) {
        self.sum = sum;
    }
}

// A simple example of a Rust function that is exposed to JavaScript via WASM bindings.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> Testing {
    Testing { sum: a.to_string(), b: vec![a as u32, b as u32] }
}

// A simple example of a Rust function that is exposed to JavaScript via WASM bindings.
#[wasm_bindgen]
pub fn calc_fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        calc_fibonacci(n - 1) + calc_fibonacci(n - 2)
    }
}