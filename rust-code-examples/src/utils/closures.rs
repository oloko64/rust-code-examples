fn test(val: impl Into<String>) -> Box<dyn Fn() -> String> {
    let base = "Hi there ";
    let val = val.into();

    Box::new(move || {
        let mut string = String::with_capacity(base.len() + val.len());
        string.push_str(base);
        string.push_str(&val);
        string
    })
}

fn test2() -> Box<dyn FnMut(i32)> {
    let mut sum = 0;
    let mut count = 0;

    Box::new(move |val| {
        sum += val;
            count += 1;

        if count < 3 {
            return;
        }

        println!("{}", sum as f32 / count as f32);
    })
}

fn example_exec() {
    let mut t2 = test2();
    t2(1);
    t2(2);
    t2(3);
    t2(4);
    t2(5);
}

// Javascript version

// function test() {
//     let sum = 0
//     let count = 0

//     return (val: number) => {
//         sum += val
//         count += 1

//         if (count < 3) {
//             return
//         }

//         console.log(sum / count)
//     }
// }

// const t = test()

// t(1)
// t(2)
// t(3)
// t(4)
// t(5)