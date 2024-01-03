// Memory alignment

// #[repr(C)]
struct S1 {
    a: bool, // 1 byte
    b: u32,  // 4 bytes
    c: u8,   // 1 byte
    d: u64,  // 8 bytes
    e: u16,  // 2 bytes
}

// Traits
trait Foo {
    // Since this method has a generic type parameter, it cannot be made into a trait object.
    // So we opt out of with the Sized trait.
    fn foo<T: std::fmt::Display>(&self, t: T)
    where
        Self: Sized;

    fn bar(&self);
}

fn call_dyn_foo(f: &dyn Foo) {
    // f.foo("hello");
    f.bar();
}
