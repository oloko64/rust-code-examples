use std::cell::RefCell;

pub fn test_ref_cell() {
    let a = RefCell::new(1);

    let b = RefCell::clone(&a);

    *b.borrow_mut() = 2;

    println!("ref_cell test: a: {}, b: {}", a.borrow(), b.borrow());
}
