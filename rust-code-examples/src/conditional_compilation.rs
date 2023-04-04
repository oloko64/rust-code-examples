#[cfg(any(target_os = "linux", target_os = "macos"))]
fn os_specific_func() {
    println!("Hello, world! Linux or Mac");
}

#[cfg(target_os = "windows")]
fn os_specific_func() {
    println!("Hello, world! Windows");
}

// https://doc.rust-lang.org/reference/conditional-compilation.html
pub fn test_os_specific() {
    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unknown"
    };
    println!("OS: {}", os);

    os_specific_func();
}
