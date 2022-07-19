use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn calc_fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        calc_fibonacci(n - 1) + calc_fibonacci(n - 2)
    }
}