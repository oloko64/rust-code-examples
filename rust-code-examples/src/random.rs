use std::num::ParseIntError;

use rayon::prelude::*;

#[derive(Debug)]
pub enum Teste {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

pub struct Teste2 {
    pub a: Teste,
}
// Random tests that are small enough to be put in the main function.
pub fn random() {
    let mut count = 0;
    let vec = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    let teste: Vec<i32> = vec.par_iter().map(|&i| ten_times(i)).collect();

    let vec_str = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
    // FromIterator can really be unexpected coming from other languages where you don't expect to be able to use one method to produce options or results. I spent a long time mucking about with fold (reduce) to achieve similar results before this clicked.
    //If you have a list of Result<T, E>s, you can use collect() to see if any of them failed:

    // let results = [Ok(1), Err("nope"), Ok(3), Err("bad")];
    // let result: Result<Vec<_>, &str> = results.iter().cloned().collect();

    // https://www.reddit.com/r/rust/comments/r6dpkz/using_result_inside_a_map_closure/
    let vec_int = vec_str
        .into_iter()
        .map(|i| i.parse::<i32>())
        .collect::<Result<Vec<_>, ParseIntError>>();

    println!("{:?}", teste);

    // Testing Enums and Enum Structs
    let a = Teste::B(1);
    let b = Teste::C { x: 10, y: 11 };
    let c = Teste2 {
        a: Teste::C { x: 10, y: 11 },
    };
    if let Teste::B(x) = a {
        println!("{}", x);
    }
    if let Teste::C { x, y } = b {
        println!("{}", x);
        println!("{}", y);
    }
    println!("{:?}", b);
    println!("{:?}", c.a);

    // A example of a closure.
    let add = |x: i32, y: i32| x + y;

    println!("{}", add(1, 2));

    // How to use the loop keyword.
    loop {
        println!("Before -> {}", count);
        count += 1;
        if count == 5 {
            assert_eq!(count, 5);
            break;
        }
        println!("After -> {}", count);
    }

    match optional() {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

// A simple return statement.
pub fn ten_times(value: i32) -> i32 {
    value * 10
}

// A simple function that returns an Option.
pub fn optional() -> Option<String> {
    let value = 10;
    if value == 102 {
        Some("10".to_string())
    } else {
        None
    }
}

// Testing the borrow checker and its rules.
pub fn borrow() {
    let s = vec![1, 2, 3, 4];
    let _f = String::from("teste");

    let iter: Vec<i32> = s.into_iter().filter(|x| x == &1).collect();

    println!("{:?}", iter);
}

// Return a cloned vector.
fn official_sort(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort();
    arr
}

// A manual implementation of a sort algorithm, using the bubble sort algorithm and no extern crates.
fn my_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let arr_len = arr.len();
    let mut swapped = false;

    for i in 0..arr_len - 1 {
        for j in 0..arr_len - i - 1 {
            if arr[j] > arr[j + 1] {
                swapped = true;
                (arr[j], arr[j + 1]) = (arr[j + 1], arr[j]);
                // arr.swap(j, j + 1);
            }
        }
        if !swapped {
            break;
        }
    }
    arr
}

// Using try_for_each to iterate over a vector and return an error if the value is not a number.
pub fn try_for_each_example(arr: Vec<&str>) -> Vec<i32> {
    let mut vec = Vec::new();
    arr.into_iter().try_for_each(|el| {
        vec.push(el.parse::<i32>().ok()?);
        Some(())
    });
    vec
}
