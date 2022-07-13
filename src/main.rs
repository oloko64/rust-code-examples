use rayon::prelude::*;

#[derive(Debug)]
enum Teste {
    A,
    B(i32),
    C { x: i32, y: i32 },
}

struct Teste2 {
    a: Teste,
}

fn main () {
    let mut count = 0;
    let vec = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    let teste: Vec<i32> = vec.par_iter().map(|&i| ten_times(i) ).collect();

    println!("{:?}", teste);
    
    // Testing Enums and Enum Structs
    let a = Teste::B(1);
    let b = Teste::C{x: 10, y:11};
    let c = Teste2 { a: Teste::C{x: 10, y:11} };
    if let Teste::B(x) = a {
        println!("{}", x);
    }
    if let Teste::C{x, y} = b {
        println!("{}", x);
        println!("{}", y);
    }
    println!("{:?}", b);
    println!("{:?}", c.a);

    let add = |x: i32, y: i32| x + y;

    println!("{}", add(1, 2));
    loop {
        println!("Before -> {}", count);
        count += 1;
        if count == 5 {
            assert_eq!(count, 5);
            break;
        }
        println!("After -> {}", count);
    }
}

fn ten_times(value: i32) -> i32 {
    value * 10
}
