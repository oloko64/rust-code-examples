// Having this macro above every function that you want to export as an FFI.
// You need to call this file "lib.rs" and put it inside a folder called "src".
// In the "Cargo.toml" file you need to add the following lines to export correctly:
// [lib]
// crate-type = ["rlib", "cdylib"]

// Using the #[no_mangle] attribute to make the function available to the C code.
#[no_mangle]
pub extern fn add_numbers(n1: i32, n2: i32) -> i32 {
    println!("Rust is awesome!");
    n1 + n2
}

// Using the #[no_mangle] attribute to make the function available to the C code.
#[no_mangle]
pub extern fn calc_fibonacci(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        calc_fibonacci(n - 1) + calc_fibonacci(n - 2)
    }
}
