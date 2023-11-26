// A trait that has a foo method that is not has `&self` as the first argument
// So we cannot call it like `struct.foo(&val)` but rather `Struct::foo(&val)`
trait PhantomExample {
    fn foo(x: &str) -> Self;
}

// Here we have to use a PhantomData marker to tell the compiler that we are using the generic
struct PhantomStruct<T: PhantomExample> {
    name: String,
    _marker: std::marker::PhantomData<T>,
}

impl<T: PhantomExample> PhantomStruct<T> {
    fn new(name: String) -> Self {
        Self {
            name,
            _marker: std::marker::PhantomData,
        }
    }

    // Here we can call the foo method on the generic type even though it is not on the vtable
    fn run(&self) {
        T::foo(&self.name);
    }
}

struct MyStruct3;

impl PhantomExample for MyStruct3 {
    fn foo(x: &str) -> Self {
        println!("x = {}", x);
        Self
    }
}

// Reference: https://doc.rust-lang.org/nomicon/phantom-data.html
// Example: https://www.youtube.com/watch?v=ub5vIPNMqI4
pub fn example() {
    let value = PhantomStruct::<MyStruct3>::new("Hello World".to_string());

    value.run();
}
