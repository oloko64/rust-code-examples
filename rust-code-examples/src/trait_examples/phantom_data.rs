trait PhantomExample {
    // A trait that has a foo method that is not has `&self` as the first argument
    // So we cannot call it like `struct.foo(&val)` but rather `Struct::foo(&val)`
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
        // Here we can call a method on a generic type
        T::foo(&self.name);
    }
}

struct MyStruct;

impl PhantomExample for MyStruct {
    fn foo(x: &str) -> Self {
        println!("x = {}", x);
        Self
    }
}

// Reference: https://doc.rust-lang.org/nomicon/phantom-data.html
// Example: https://www.youtube.com/watch?v=ub5vIPNMqI4
pub fn example() {
    // Here we can pass the struct as a generic type, not as a value
    let value = PhantomStruct::<MyStruct>::new("Hello World".to_string());

    value.run();
}

// ------------------------------------------ Using trait objects with vtable ------------------------------------------------------

trait PhantomExample2 {
    // A trait that has a foo method that is not has `&self` as the first argument
    // So we cannot call it like `struct.foo(&val)` but rather `Struct::foo(&val)`
    // We also need to make the trait object safe by adding the Sized trait bound
    fn foo(x: &str) -> Self
    where
        Self: Sized;

    // Here we added a method that is on the vtable
    fn bar(&self) -> String;
}

// Here we have to use a PhantomData marker to tell the compiler that we are using the generic
struct PhantomStruct2<'a, T: PhantomExample2> {
    name: String,

    // Now we need to add a reference to the trait object, so we can have access to it's vtable methods
    struct_val: &'a dyn PhantomExample2,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: PhantomExample2> PhantomStruct2<'a, T> {
    fn new(val: &'a dyn PhantomExample2) -> Self {
        Self {
            name: String::from("hello"),
            struct_val: val,
            _marker: std::marker::PhantomData,
        }
    }

    // Here we can call the foo method on the generic type even though it is not on the vtable
    fn run(&self) {
        // Now we can call the bar method on the vtable
        let _ = self.struct_val.bar();

        T::foo(&self.name);
    }
}

struct MyStruct2;

impl PhantomExample2 for MyStruct2 {
    fn foo(x: &str) -> Self {
        println!("x = {}", x);
        Self
    }

    fn bar(&self) -> String {
        String::from("hello")
    }
}

pub fn example_2() {
    // Now we have no pass the struct to the function so it can have access to it
    let struct_val = MyStruct2;
    let value = PhantomStruct2::<MyStruct2>::new(&struct_val);

    value.run();
}
