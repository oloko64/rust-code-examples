// This is incorrect because &mut T is invariant in T.
// The lifetime of the &mut and T should not be the same.
fn wrong_lifetime<'a>(v: &'a mut &'a str) -> &'a str {
    *v = "hello";
    *v
}

// In this case, the lifetime of the &mut and T are different.
fn correct_lifetime<'a>(v: &mut &'a str) -> &'a str {
    *v = "hello";
    *v
}

fn example() {
    // let mut s = "hello";
    // let _ = wrong_lifetime(&mut s);
    // println!("{}", s);

    let mut s = "hello";
    let _ = correct_lifetime(&mut s);
    println!("{}", s);
}
