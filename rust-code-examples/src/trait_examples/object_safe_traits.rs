use static_assertions::assert_obj_safe;

pub fn object_safe_traits_example() {
    // How to know if a trait is object safe?
    // You can use the `assert_obj_safe` macro from the `static_assertions` crate
    // https://youtu.be/izXf9-CTAfc?t=3656

    // Default is not object safe
    // assert_obj_safe!(Default);

    // Trait `Foo` is object safe
    assert_obj_safe!(Foo);

    // Trait `Bar` is not object safe
    // assert_obj_safe!(Bar);

    // Trait `Baz` is not object safe
    // assert_obj_safe!(Baz);

    // Trait `Baz2` is object safe
    assert_obj_safe!(Baz2);

    // Trait `Baz3` is not object safe
    // assert_obj_safe!(Baz3);
}

trait Foo {
    fn foo(&self);
}

// The following example fails because object safe traits can't have const
trait Bar {
    const BAR: usize;

    fn bar(&self);
}

// The following example fails to compile because generics without where Self: Sized are not allowed in object-safe trait methods
trait Baz {
    fn baz<T>(&self) -> T;
}

// We can fix that by adding the where Self: Sized
trait Baz2 {
    fn baz<T>(&self) -> T
    where
        Self: Sized;
}

// The following example fails to compile because object safe traits must have a receiver type of `self`, `&self` or `&mut self`
// Methods without a receiver don't have a reference to self
// So they don't have access to a vtable to call the method during dynamic dispatch
trait Baz3 {
    fn baz();
}

// This is the usefulness of object safe traits, it allows us to use trait objects
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
//
// The following commented code fails to compile because the compiler can't know the size of the type, as T could be anything
// And in the Foo case there could be multiple declarations of that const, which would be a problem
struct Screen {
    components: Vec<Box<dyn Foo>>,
    // components_2: Vec<Box<dyn Bar>>,
    // components_3: Vec<Box<dyn Baz>>,
    components_4: Vec<Box<dyn Baz2>>,
    // components_5: Vec<Box<dyn Baz3>>,
}
