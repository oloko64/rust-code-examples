pub fn listing_1_9() {
    let mut x = Box::new(42);
    let mut z = &x;

    for i in 0..100 {
        println!("{}", z);
        x = Box::new(i);
        z = &x;
    }

    println!("{}", z);
}

// listing 1.10
struct StrSplit<'a, 'b> {
    delimiter: &'a str,
    document: &'b str,
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn str_before(s: &str, c: char) -> Option<&str> {
    StrSplit {
        document: s,
        delimiter: &c.to_string(),
    }
    .next()
}

// listing 1.11
struct MutStr<'a, 'b> {
    s: &'a mut &'b str,
}

fn listing_1_11() {
    let mut s = "Hello";
    *MutStr { s: &mut s }.s = "world";

    println!("{}", s);
}
