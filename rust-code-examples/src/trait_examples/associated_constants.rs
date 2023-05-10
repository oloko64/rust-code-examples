pub fn associated_types_example() {
    println!("{}", u8::MAX_VALUE);
    println!("{}", u8::MIN_VALUE);
}

// You can assign a const to a trait
trait LimitedNumber {
    const MIN_VALUE: usize;
    const MAX_VALUE: usize;
}

impl LimitedNumber for u8 {
    const MIN_VALUE: usize = 0;
    const MAX_VALUE: usize = 120;
}
