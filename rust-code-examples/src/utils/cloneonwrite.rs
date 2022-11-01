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

/// Official example from https://doc.rust-lang.org/std/borrow/enum.Cow.html
pub fn rust_cow_use_example() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // Clone occurs because `input` needs to be mutated.
    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]);
    abs_all(&mut input);

    // No clone occurs because `input` is already owned.
    let mut input = Cow::from(vec![-1, 0, 1]);
    abs_all(&mut input);
}

/// Returns absolute values inside a cloned array if necessary.
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
}
