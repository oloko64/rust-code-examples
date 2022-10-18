use std::{collections::HashSet, ptr, time::Duration, thread};
mod teste;
use types::User;
use unicode_segmentation::UnicodeSegmentation;
use rayon::prelude::*;
mod random;
mod conversions;
mod concurrency;
mod hashmaps;
mod types;
mod lifehooks;

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
        Car {
            name,
            year,
        }
    }

    fn drive(&self) {
        println!("{} is driving", self.name);
    }
}

fn press_key(key: char) -> Event {
    Event::KeyPress(key)
}

fn parallel_test() {
    ["muriloo", "reinaldo"].into_iter().for_each(|i| {
        println!("{} is driving", i);
        thread::sleep(Duration::from_secs(2));
    });
}


fn main () {
    let pointer_to_number = 0;

    parallel_test();

    println!("{:?}", ptr::addr_of!(pointer_to_number));

    let a = Box::new(1);

    let b = &a;

    println!("{}", a);

    teste::teste::teste();
    println!("{}", teste::secondfile::second_number(3));

    'a:loop {
        loop {
            loop {
                break 'a;
            }
        }
    }
    // let cl = () => {}
    let cl = |x: i32| {x + 3};
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
}

fn test<'a>(vec: &mut Vec<&'a str>, x: &'a str) {
    vec.push(x);
}

fn question(st: &str) {
    let mut set: HashSet<&str> = HashSet::new();
    for i in 0..st.len() {
        for j in i..st.len() {
            set.insert(&st[i..j+1]);
        }
    }
    let mut vec = set.into_iter().collect::<Vec<_>>();
    vec.sort();
    println!("{:?}", vec.last().unwrap());
}


fn iterate_strings() {
    // Not the correct way as may be errors depending on the char
    for c in "abcdefgã".chars() {
        println!("{}", c);
    }

    for c2 in "abcdefgã".graphemes(true) {
        println!("{}", c2);
    }
}

fn prt<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}

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

#[cfg(test)]

mod tests {
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
