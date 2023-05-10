pub fn associated_types_example() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let deref = arr.my_deref();

    println!("{:?}", deref);
}

// Can only be implemented once for any given type
trait MyDeref {
    type Target: ?Sized;

    fn my_deref(&self) -> &Self::Target;
}

// This is a simplified version of the std library implementation
impl<T> MyDeref for Vec<T> {
    type Target = [T];

    fn my_deref(&self) -> &Self::Target {
        &self[..]
    }
}

trait MyIterator {
    type Item;

    fn my_next(&mut self) -> Option<Self::Item>;
}
