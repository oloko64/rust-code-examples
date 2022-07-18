mod random;
mod conversions;
mod concurrency;
mod hashmaps;

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
    // random::random();
    // conversions::to_json();
    random::borrow();

    let _test = Box::from(3);

    let car = Car::new("Ferrari".to_string(), 2019);

    println!("{:#?}", car.name);

    concurrency::threads();
    hashmaps::hashmaps();
}