struct PrintVals<T> {
    iter: T,
    sum: i32,
}

impl<T> Iterator for PrintVals<T>
where
    T: Iterator,
    T::Item: std::fmt::Display,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.iter.next();
        if let Some(ref val) = val {
            println!("{} - {}", val, self.sum);
        }
        val
    }
}

trait PrintValsIterator: Iterator {
    fn print_vals(self, sum: i32) -> PrintVals<Self>
    where
        Self: Sized;
}

impl<T> PrintValsIterator for T
where
    T: Sized + Iterator,
{
    fn print_vals(self, sum: i32) -> PrintVals<Self> {
        PrintVals { iter: self, sum }
    }
}
/// Reference: https://burgers.io/extending-iterator-trait-in-rust
pub fn print_vals_example() {
    let vec = vec![1, 2, 3, 4, 5];
    let val = vec.iter().print_vals(2).map(|x| x * 2).collect::<Vec<_>>();

    dbg!(val);
}
