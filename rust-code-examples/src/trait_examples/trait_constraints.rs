use std::fmt::Display;

// https://youtu.be/izXf9-CTAfc?t=1870
pub fn with_impl_constraint() {
    // If we comment the Display implementation for `Username`, we will get an error when trying to use the trait method
    let user = ImplConstraint {
        name: Username {
            first_name: "John",
            last_name: "Doe",
        },
    };
    user.print();
    user.debug();

    // If we comment the Display implementation for `username2`, we will get an error when trying declare the struct
    let user_2 = StructConstraint {
        name: Username2 {
            first_name: "John",
            last_name: "Doe",
        },
    };
    user_2.print();
}
// -----------------------------------------------------------------------------------------------------
// Using implementation constraints allow us to use the struct with types that don't implement the trait
// And when using types that implement the trait, we can use the trait methods
// -----------------------------------------------------------------------------------------------------

// This also works with traits
trait MyTrait {
    fn print(&self);
    fn debug(&self)
    where
        Self: std::fmt::Debug;
}

#[derive(Default, Debug)]
struct Username<T> {
    first_name: T,
    last_name: T,
}

// When we implement Default and Display, it allow us to use the struct with the trait methods
// Comment the Display implementation to see the error
impl<T: Display> Display for Username<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

// Here we don't constrain the generic type to implement the trait
#[derive(Debug)]
struct ImplConstraint<T> {
    name: T,
}

// But is better to constrain the generic type in the implementation
// As it allow us to use the struct even with types that don't implement the trait
// And when using types that implement the trait, we can use the trait methods
impl<T: Default + Display> MyTrait for ImplConstraint<T> {
    fn print(&self) {
        println!("name: {}", self.name);
    }

    // We can use the debug method from the trait when the generic type implement the Debug trait
    // If we use a type that don't implement the Debug trait, it will allow us to use the trait methods, but not the debug method
    fn debug(&self)
    where
        Self: std::fmt::Debug,
    {
        println!("debug: {:?}", self);
    }
}

// -----------------------------------------------------------------------------------------------------------
// We can also require the trait bound in the struct, not allowing to use types that don't implement the trait
// -----------------------------------------------------------------------------------------------------------

trait MyTrait2 {
    fn print(&self);
}

#[derive(Default)]
struct Username2<T> {
    first_name: T,
    last_name: T,
}

// When we implement Default and Display, it allow us to use the struct with the trait methods
// Comment the Display implementation to see the error
impl<T: Display> Display for Username2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

// Here we constrain the generic type to implement the trait
struct StructConstraint<T: Default + Display> {
    name: T,
}

// But is better to constrain the generic type in the implementation
// As it allow us to use the struct even with types that don't implement the trait
// And when using types that implement the trait, we can use the trait methods
impl<T: Default + Display> MyTrait for StructConstraint<T> {
    fn print(&self) {
        println!("name: {}", self.name);
    }

    // We can use the debug method from the trait when the generic type implement the Debug trait
    // If we use a type that don't implement the Debug trait, it will allow us to use the trait methods, but not the debug method
    fn debug(&self)
    where
        Self: std::fmt::Debug,
    {
        println!("debug: {:?}", self);
    }
}
