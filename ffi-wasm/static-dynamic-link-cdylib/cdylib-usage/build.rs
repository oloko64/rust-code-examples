fn main() {
    // https://doc.rust-lang.org/stable/cargo/reference/build-scripts.html#rustc-link-lib
    println!("cargo:rustc-link-search=/home/oloko64/Documents/git-repositories/rust-apps/rust-code-tests/ffi-wasm/static-dynamic-link-cdylib/cdylib-example/target/release");
    println!("cargo:rustc-link-lib=static=cdylib_example");
}
