use std::thread;

pub fn threads() {
    // Thread example
    let thread_value = 3;
    let handle = thread::spawn(move ||{
        println!("Hello from a thread!");
        thread_value
    });
    println!("Waiting for thread to finish...");
    let value = handle.join().unwrap();
    println!("Thread finished with value: {}", value);

    // Multiple threads
    let mut threads = Vec::with_capacity(10);

    for i in 0..10 {
        threads.push( thread::spawn(move || {
            println!("Hello from a thread! {i:?}");
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }

    // Closure example
    let data = vec![1, 2, 3];
    let closure = || println!("captured {data:?} by value");
    closure();
    println!("captured {:?} by reference", data);
}