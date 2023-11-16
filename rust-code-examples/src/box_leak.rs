use std::thread;

#[derive(Debug)]
struct MyStruct {
    a: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with a = {}", self.a);
    }
}

fn example_fn(val: &'static MyStruct) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Hello from thread");
        println!("{:?}", val);
    })
}

pub fn example() {
    let val = Box::new(MyStruct {
        a: "Hello".to_string(),
    });
    let leaked: &'static MyStruct = Box::leak(val);

    let handle = example_fn(leaked);

    handle.join().unwrap();

    let boxed = unsafe { Box::from_raw(leaked as *const MyStruct as *mut MyStruct) };
    // This will cause a double free error
    // let boxed = unsafe { Box::from_raw(leaked as *const MyStruct as *mut MyStruct) };

    println!("{:?}", boxed);
}
