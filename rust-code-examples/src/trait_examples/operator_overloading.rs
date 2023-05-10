pub fn operator_overloading_example() {
    let val = 1.my_add(2);
    let val_2 = 1.my_add(2.0);

    println!("{}", val);
    println!("{}", val_2);
}

// Generic parameters can have defaults
trait MyAdd<Rhs = Self> {
    type Output;

    fn my_add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd for u8 {
    type Output = u8;

    fn my_add(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

// In this example we personalize the implementation for f64
impl MyAdd<f64> for u8 {
    type Output = f64;

    fn my_add(self, rhs: f64) -> Self::Output {
        self as f64 + rhs * 2.0
    }
}
