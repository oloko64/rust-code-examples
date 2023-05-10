pub fn generic_traits_example() {
    println!("{}", u8::my_from(1u16));
    println!("{}", u16::my_from(1u8));

    let val_1: u8 = 1.my_into();
    let val_2: u16 = 1.my_into();

    println!("{}", val_1);
    println!("{}", val_2);
}

// Can be implemented for every T separately
// This is object safe as at compile time the compiler knows the size of T, as it will be implemented for a concrete type
trait MyFrom<T>: Sized {
    fn my_from(value: T) -> Self;
}

impl MyFrom<u8> for u16 {
    fn my_from(val: u8) -> Self {
        val as Self
    }
}

impl MyFrom<u16> for u8 {
    fn my_from(val: u16) -> Self {
        val as Self
    }
}

// Implement one get another for free
// This is the case with the `From` and `Into` from the std library
trait MyInto<T>: Sized {
    fn my_into(self) -> T;
}

impl<T, U> MyInto<U> for T
where
    U: MyFrom<T>,
{
    fn my_into(self) -> U {
        U::my_from(self)
    }
}
