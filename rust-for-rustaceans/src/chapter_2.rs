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

// Bounds on associated types
trait MyTraitAssociatedType {
    type Item;

    // The trait bound on the associated type is opting out in case it does not implement `Default`
    fn get(&self) -> Self::Item
    where
        Self::Item: Default;
}

struct MyStructAssociatedType;

// Non Default struct
struct AuxNoDefault;

impl MyTraitAssociatedType for MyStructAssociatedType {
    type Item = AuxNoDefault;

    // This would not work as `AuxNoDefault` does not implement `Default`
    // type Item = AuxNoDefault;

    fn get(&self) -> Self::Item {
        AuxNoDefault
    }
}

fn associated_type_ex() {
    let s = MyStructAssociatedType;

    // This does not work as `AuxNoDefault` does not implement `Default`
    // let _ = s.get();
}

// Derive traits that are good to implement

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]     // All of them
// #[derive(Debug, PartialEq, PartialOrd, Clone, Default)]                          // Most common
// #[derive(Eq, Ord, Hash)]                                                         // Rarely, check if it is needed
// #[derive(Clone, Copy)]                                                           // Users may not expect a type to be copied, and i'ts a breaking change to remove it later
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
struct DefaultImplementors {
    a: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_default_impl() {
        let s = DefaultImplementors { a: 5 };

        fn all_implementors<
            T: std::fmt::Debug
                + std::cmp::PartialEq
                + std::cmp::Eq
                + std::cmp::PartialOrd
                + std::cmp::Ord
                + std::hash::Hash
                + Clone
                + Copy
                + Default,
        >(
            _v: T,
        ) {
        }
        let _ = all_implementors(s); // This would not compile if any of the traits were not implemented

        fn most_common_implementors<
            T: std::fmt::Debug + std::cmp::PartialEq + std::cmp::PartialOrd + Clone + Default,
        >(
            _v: T,
        ) {
        }
        let _ = most_common_implementors(s); // This would not compile if any of the traits were not implemented

        fn rarely_implementors<T: std::cmp::Eq + std::cmp::Ord + std::hash::Hash>(_v: T) {}
        let _ = rarely_implementors(s); // This would not compile if any of the traits were not implemented

        fn copy_implementors<T: Clone + Copy>(_v: T) {}
        let _ = copy_implementors(s); // This would not compile if any of the traits were not implemented

        fn normal_implementors<T: Send + Sync + Sized + Unpin>(_v: T) {}
        let _ = normal_implementors(s); // This would not compile if any of the traits were not implemented
    }
}

// Ergonomic traits
// Is good to implement traits to & and &mut references to the type, as it is more ergonomic for the user

struct MyErgonomicStruct {
    a: Vec<u8>,
}

impl IntoIterator for MyErgonomicStruct {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter()
    }
}

impl<'a> IntoIterator for &'a MyErgonomicStruct {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.iter()
    }
}

impl<'a> IntoIterator for &'a mut MyErgonomicStruct {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.iter_mut()
    }
}

// This allows the user to use the type as a Vec<u8>
impl std::ops::Deref for MyErgonomicStruct {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

// This allows the user to switch from a Vec<u8> to the type using `From` or `Into`
impl From<Vec<u8>> for MyErgonomicStruct {
    fn from(v: Vec<u8>) -> Self {
        Self { a: v }
    }
}

impl From<MyErgonomicStruct> for Vec<u8> {
    fn from(v: MyErgonomicStruct) -> Self {
        v.a
    }
}

fn ex_ergonomic() {
    let val = MyErgonomicStruct { a: vec![1, 2, 3] };
    let iter = val.into_iter();

    let val = &MyErgonomicStruct { a: vec![1, 2, 3] };
    let iter = val.into_iter();

    let val = &mut MyErgonomicStruct { a: vec![1, 2, 3] };
    let iter = val.into_iter();

    // Using deref allows the user to use `.len()` on the type
    let val = MyErgonomicStruct { a: vec![1, 2, 3] };
    let _ = val.len();

    // Using `From` allows the user to switch from a Vec<u8> to the type
    let val = MyErgonomicStruct::from(vec![1, 2, 3]);
    let val_arr: Vec<u8> = val.into();
}
