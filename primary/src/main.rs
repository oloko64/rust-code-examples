use types::User;
use unicode_segmentation::UnicodeSegmentation;

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

impl Car {
    fn new(name: String, year: i32) -> Car {
        Car {
            name,
            year,
        }
    }
}

fn main () {
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

    let v = vec![1, 2, 3];
    println!("{:?}", v);
    test(v.clone());
    println!("{v:?}");
}

fn test(mut vec: Vec<i32>) {
    vec.push(3);
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
