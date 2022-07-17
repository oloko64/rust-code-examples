#[no_mangle]
pub extern fn add_numbers(n1: i32, n2: i32) -> i32 {
    println!("Rust is awesome!");
    n1 + n2
}

#[no_mangle]
pub extern fn calc_fibonacci(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        calc_fibonacci(n - 1) + calc_fibonacci(n - 2)
    }
}