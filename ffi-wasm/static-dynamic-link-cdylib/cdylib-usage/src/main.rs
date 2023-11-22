fn main() {
    // using dynamic library
    println!("Using dynamic library");
    using_dynamic_library();

    // using static library
    println!("Using static library");
    using_static_library();
}

#[link(name = "cdylib_example", kind = "static")]
extern "C" {
    fn add_one(x: i32) -> i32;
}
fn using_static_library() {
    let result = unsafe { add_one(1) };

    println!("Result: {}", result);
}

fn using_dynamic_library() {
    unsafe {
        let lib =
            libloading::Library::new("/home/oloko64/Documents/git-repositories/rust-apps/rust-code-tests/ffi-wasm/static-dynamic-link-cdylib/cdylib-example/target/release/libcdylib_example.so").unwrap();
        let func = lib
            .get::<unsafe extern "C" fn(i32) -> i32>(b"add_one\0")
            .unwrap();

        let result = func(1);

        println!("Result: {}", result);
    }
}
