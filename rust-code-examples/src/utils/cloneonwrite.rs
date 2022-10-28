use std::borrow::Cow;

/// Choose between returning a String or a &str.
fn module_cow(input: u8) -> Cow<'static, str> {
    if input % 2 == 0 {
        Cow::Borrowed("even")
    } else {
        Cow::Owned("odd".to_string())
    }
}

/// Match between the Borrowed and Owned variants.
pub fn run_cow() {
    for i in 0..10 {
        match module_cow(i) {
            Cow::Borrowed(s) => println!("Borrow -> {} is {}", i, s),
            Cow::Owned(s) => println!("Owned -> {} is {}", i, s),
        }
    }
}
