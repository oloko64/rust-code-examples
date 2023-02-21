use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image_processing::Image;

struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

impl<'a> Name<'a> {
    fn new(first: &'a str, last: &'a str) -> Name<'a> {
        Name { first, last }
    }
}

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));

    c.bench_function("new_name", |b| {
        b.iter(|| Name::new(black_box("John"), black_box("Doe")))
    });

    // c.bench_function("new_from_path", |b| {
    //     b.iter(|| Image::new_from_path("imgs/rgby.png"))
    // });

    // c.bench_function("negative_image", |b| {
    //     b.iter(|| {
    //         let mut img = Image::new_from_path("imgs/rgby.png").unwrap();
    //         black_box(img.negative());
    //     })
    // });

    // c.bench_function("blur_image", |b| {
    //     b.iter(|| {
    //         let mut img = Image::new_from_path("imgs/rgby.png").unwrap();
    //         black_box(img.blur(5.0));
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
