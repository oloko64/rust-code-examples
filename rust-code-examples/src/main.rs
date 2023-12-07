mod any;
mod box_leak;
mod channels;
mod command_piped;
mod concurrency;
mod conditional_compilation;
mod conversions;
mod data_types;
mod error_examples;
mod ext_iterators;
mod file_manipulation;
mod hashmaps;
mod into_from;
mod lifetimes;
mod memoization;
mod mocks;
mod multiple_tokio_worker_types;
mod new_type;
mod parallelism;
mod random;
mod singleton;
mod state_machine;
mod string_padding;
mod trait_examples;
mod type_state;
mod types;
mod utils;

use rayon::prelude::*;
use std::{cell::RefCell, collections::HashSet, ptr, rc::Rc, thread, time::Duration};
use types::User;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    new_type::{
        important_num_consumer, important_num_consumer_new_type, important_num_generator,
        important_num_generator_new_type,
    },
    trait_examples::traits::test_trait,
};

struct Car {
    name: String,
    year: i32,
}

enum Mode<T, U> {
    Drive(T),
    Park(U),
    Stop(T),
}

enum Event {
    Click { x: i64, y: i64 },
    KeyPress(char),
    KeyRelease(char),
}

impl Car {
    fn new(name: String, year: i32) -> Car {
        Car { name, year }
    }

    fn drive(&self) {
        println!("{} is driving", self.name);
    }
}

// A example of Enum usage.
fn press_key(key: char) -> Event {
    Event::KeyPress(key)
}

/// Using the Rayon library to make parallel computations.
fn parallel_test() {
    ["oloko", "reinaldo"].into_par_iter().for_each(|i| {
        println!("{} is driving", i);
        thread::sleep(Duration::from_secs(2));
    });
}

// This is a mix of various tests that I've done to learn Rust.
fn main() {
    let generic_f64 = important_num_generator(32.0);
    important_num_consumer(generic_f64);

    let new_type = important_num_generator_new_type(32.0);
    important_num_consumer_new_type(new_type);

    singleton::test_singleton();
    test_trait();

    utils::ref_cell::test_ref_cell();
    clone_mutable_reference();

    let mut linked_official = std::collections::LinkedList::new();
    let teste = String::from("teste");
    linked_official.push_back(teste);
    // println!("{:?}", teste);

    // let _config = types::Configuration::new("path");
    types::user_state(3);
    // Linked List example
    let mut list = data_types::linked_list_enum::LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_back(4);

    // list.push(3);
    dbg!(list);

    let mut linked_official = std::collections::LinkedList::new();
    linked_official.push_front(1);
    linked_official.push_front(2);
    linked_official.push_front(3);
    linked_official.push_back(4);

    dbg!(linked_official);

    let pointer_to_number = 0;

    parallel_test();

    println!("{:?}", ptr::addr_of!(pointer_to_number));

    let a = Box::new(1);

    let b = &a;

    println!("{}", a);

    println!("{}", utils::testrust::second_number(3));

    'a: loop {
        loop {
            loop {
                break 'a;
            }
        }
    }
    // let cl = () => {}
    let cl = |x: i32| x + 3;
    cl(3);

    // random::random();
    // conversions::to_json();
    random::borrow();

    let _test = Box::from(3);

    let car = Car::new("Ferrari".to_string(), 2019);
    car.drive();

    println!("{:#?}", car.name);

    // concurrency::threads();
    // hashmaps::hashmaps();
    // iterate_strings();
    if let User::Banned(reason) = types::user_state(4) {
        println!("{}", reason);
    }
    println!("{:?}", types::user_state(4));
    prt(33);
    prt("sdfgs");
    // print_loops(); Vec<Vec<u8>>

    let mut v = vec!["1", "2", "3"];
    println!("{:?}", v);
    test(&mut v, "4");
    println!("{:?}", v);

    question("banana");

    // A example of the Clone on Write (Cow).
    utils::cloneonwrite::run_cow();

    let teste: &str;
    let mut vec = vec!["1", "2", "3"];
    {
        teste = "test";
        let teste2 = "oi";
        test(&mut vec, teste);
        test(&mut vec, teste2);
    }
    println!("{:?}", vec);

    dbg!(random::try_for_each_example(vec!["1", "2", "3", "as"]));
}

// This makes sure that the referenced array and the referenced string are not dropped.
// Reference: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html?highlight=lifetime#lifetime-elision
fn test<'a>(vec: &mut Vec<&'a str>, x: &'a str) {
    vec.push(x);
}

// Using HashSet to remove duplicates from a vector.
fn question(st: &str) {
    let mut set: HashSet<&str> = HashSet::new();
    for i in 0..st.len() {
        for j in i..st.len() {
            set.insert(&st[i..j + 1]);
        }
    }
    // Convert the HashSet into a vector as HashSet does not implement the sort method.
    let mut vec = set.into_iter().collect::<Vec<_>>();
    vec.sort();
    println!("{:?}", vec.last().unwrap());
}

fn iterate_strings() {
    // Not the correct way as may be errors depending on the char, as UTF-8 is not a fixed size of one byte.
    for c in "abcdefgã".chars() {
        println!("{}", c);
    }

    // The correct way to iterate over a string chars, using the unicode_segmentation crate.
    for c2 in "abcdefgã".graphemes(true) {
        println!("{}", c2);
    }
}

// A example of a function where the first argument receives the Debug Trait.
fn prt<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

// Iteration examples.
fn print_loops() {
    for i in 0..=100 {
        let mut out = String::new();

        if i % 3 == 0 && i % 5 == 0 {
            out.push_str("FizzBuzz");
        } else if i % 3 == 0 {
            out.push_str("Fizz");
        } else if i % 5 == 0 {
            out.push_str("Buzz");
        } else {
            out.push_str(&i.to_string());
        }
        println!("{}", out);
    }
}

fn clone_mutable_reference() {
    // Creates a reference counter with a RefCell with the value 1.
    let a = Rc::new(RefCell::new(1));

    // Creates a new reference counter with a RefCell with the value 1.
    let b = Rc::clone(&a);

    // Changes the value of the RefCell to 2.
    *a.borrow_mut() = 2;

    // The value of `a` is 2 and the value of `b` is 2.
    println!("a: {}, b: {}", a.borrow(), b.borrow());

    // ############################################

    // Creates a RefCell with the value 1.
    let c = RefCell::new(1);

    // Creates a new RefCell with the value 1.
    let d = RefCell::clone(&c);

    // Changes the value of the RefCell to 2.
    *c.borrow_mut() = 2;

    // The value of `c` is 2 and the value of `d` is 1.
    println!("c: {}, d: {}", c.borrow(), d.borrow());
}

// How to test a function even if they are private.
#[cfg(test)]

mod tests {
    // This allow us to use the private functions of this module.
    use super::*;

    #[test]
    fn press_key_test() {
        let event = press_key('a');
        match event {
            Event::KeyPress(c) => assert_eq!(c, 'a'),
            _ => panic!("Expected a key press event"),
        }
    }
}
