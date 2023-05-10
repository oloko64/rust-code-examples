pub fn with_dynamic_dispatch() {
    print(Box::new(5));
    print(Box::new("Hello"));
    print_ref(&"Hello");

    // Here we can't use the `print` function because `User` doesn't implement `Display`
    // print(Box::new(User {
    //     name: "John".to_string(),
    //     age: 30,
    // }));

    make_printer(false)("Hello");

    let user_2 = User2 {
        name: &"John",
        age: 30,
    };
}

// Owned version of dynamic dispatch
fn print(val: Box<dyn std::fmt::Display>) {
    println!("{}", val);
}

// Reference version of dynamic dispatch
fn print_ref(val: &dyn std::fmt::Display) {
    println!("{}", val);
}

// Type erasure
// If we didn't use the `dyn` keyword, the compiler would complain about the fact that the return types are different
fn make_printer(is_err: bool) -> Box<dyn Fn(&str)> {
    if is_err {
        Box::new(|x| eprintln!("{}", x))
    } else {
        Box::new(|x| println!("{}", x))
    }
}

// You can also have nested `dyn`
fn make_printer_nested(is_err: bool) -> Box<dyn Fn(&dyn std::fmt::Debug)> {
    if is_err {
        Box::new(|x| eprintln!("{:?}", x))
    } else {
        Box::new(|x| println!("{:?}", x))
    }
}

// Not all trait can be used as trait objects
// For example, the `Default` trait can't be used as a trait object
// fn default_trait_obj(val: Box<dyn Default>) {
//     println!("{:?}", val);
// }

// -----------------------------------------------------------------------------------------------------

struct User {
    name: String,
    age: u8,
}

// We can use the `?Sized` to tell the compiler that we don't know the size of the type
// Object safety: https://youtu.be/izXf9-CTAfc?t=3200
struct User2<'a, T: ?Sized> {
    name: &'a T,
    age: u8,
}

// These two `Foo` an `Bar` are equivalent, by default the trait is object safe
// Methods needs to allocate stack space for the `self` parameter
trait Foo {
    fn take_self(self);
}

trait Bar {
    fn take_self_ref(self)
    where
        Self: Sized;
}
