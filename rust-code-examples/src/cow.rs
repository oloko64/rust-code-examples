use std::borrow::Cow;

struct LazyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

// https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55

impl<'a> LazyBuffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: Cow::Borrowed(data),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn append(&mut self, data: &[u8]) {
        self.data.to_mut().extend(data)
    }
}

pub fn example() {
    let data = vec![0u8; 10];

    // No memory copied yet
    let mut buffer = LazyBuffer::new(&data);
    println!("{:?}", buffer.data());

    // The data is cloned
    buffer.append(&[1, 2, 3]);
    println!("{:?}", buffer.data());

    // The data is not cloned on further attempts
    buffer.append(&[4, 5, 6]);
    println!("{:?}", buffer.data());
}

// ----------------------------

pub fn fizz_buzz(n: u32) -> Vec<Cow<'static, str>> {
    let mut arr = vec![];
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => arr.push(Cow::Borrowed("FizzBuzz")),
            (0, _) => arr.push(Cow::Borrowed("Fizz")),
            (_, 0) => arr.push(Cow::Borrowed("Buzz")),
            (_, _) => arr.push(Cow::Owned(i.to_string())),
        };
    }

    arr
}
