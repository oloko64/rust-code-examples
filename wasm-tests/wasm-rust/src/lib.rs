use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Testing {
    sum: String,
    b: Vec<u32>,
}

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

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> Testing {
    Testing { sum: a.to_string(), b: vec![a as u32, b as u32] }
}

#[wasm_bindgen]
pub fn calc_fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        calc_fibonacci(n - 1) + calc_fibonacci(n - 2)
    }
}