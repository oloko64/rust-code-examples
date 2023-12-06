use std::any::{Any, TypeId};

pub fn example() {
    log(&String::from("test"));
    log(&"test");
    log(&1);
    log(&1.0_f32);
    log(&vec![1, 2, 3]);
    log(&1.0_f64);
}

fn log<T: Any + std::fmt::Debug>(value: &T) {
    if TypeId::of::<T>() == TypeId::of::<String>() {
        println!("String: {:?}", value);
    } else if TypeId::of::<T>() == TypeId::of::<&str>() {
        println!("&str: {:?}", value);
    } else if TypeId::of::<T>() == TypeId::of::<i32>() {
        println!("i32: {:?}", value);
    } else if TypeId::of::<T>() == TypeId::of::<f32>() {
        println!("f32: {:?}", value);
    } else if TypeId::of::<T>() == TypeId::of::<Vec<i32>>() {
        println!("Vec<i32>: {:?}", value);
    } else {
        println!("Other: {:?}", value);
    }
}
