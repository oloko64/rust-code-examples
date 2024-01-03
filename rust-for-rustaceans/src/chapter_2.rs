// Memory alignment

use std::{collections::HashMap, sync::Arc};

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

// implementing traits for external types

trait ForeignTrait<T, U> {}

struct MyOtherStruct;

// In this case if it was a external trait, it would work as we have our type as the first type
impl<T> ForeignTrait<MyOtherStruct, u64> for Vec<T> {}

// In this case if it was a external trait, it would not work as we have our type as the second type
impl<T> ForeignTrait<u64, MyOtherStruct> for Vec<T> {}

// Trait Bounds
fn create_hashmap<K, V>() -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    HashMap::default()
}

fn create_hashmap_2<K, V>() -> HashMap<K, V>
where
    HashMap<K, V>: Default,
{
    HashMap::default()
}

// derive macros

struct Shared<T> {
    value: Arc<T>,
}

struct Aux;

// If we use the default derive macro, it will not work as we would put a Clone bound on the generic T, even if it is not needed as it is already a Arc<T>
impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Shared {
            value: self.value.clone(),
            // value: Arc::clone(&self.value),
        }
    }
}

// If we were to add `#[derive(Clone)]` to the struct, the following code would be generated, not allowing that non clone types to be used on a Arc
// impl<T> Clone for Shared<T>
// where
//     T: Clone,
// {
//     fn clone(&self) -> Self {
//         Shared {
//             value: self.value.clone(),
//             // value: Arc::clone(&self.value),
//         }
//     }
// }

fn clone_ex() {
    let s = Shared { value: Arc::new(5) };
    let s2 = s.clone();

    let s = Shared {
        value: Arc::new(String::from("hello")),
    };
    let s2 = s.clone();

    // This would not would with the default derive macro `#[derive(Clone)]` as `Aux` does not implement `Clone`
    let s = Shared {
        value: Arc::new(Aux),
    };
    let s2 = s.clone();
}
