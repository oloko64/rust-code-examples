#[derive(Debug)]
struct MyStruct {
    a: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with a = {}", self.a);
    }
}

pub fn example() {
    let val = MyStruct {
        a: "Hello".to_string(),
    };
    let leaked: &'static MyStruct = Box::leak(Box::new(val));

    println!("{:?}", leaked);

    let boxed = unsafe { Box::from_raw(leaked as *const MyStruct as *mut MyStruct) };
    // This will cause a double free error
    // let boxed = unsafe { Box::from_raw(leaked as *const MyStruct as *mut MyStruct) };

    println!("{:?}", boxed);
}
