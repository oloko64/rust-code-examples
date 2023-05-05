use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

/// A example of various thread implementations.
pub fn threads() {
    // Thread example
    let thread_value = 3;
    let handle = thread::spawn(move || {
        println!("Hello from a thread!");
        thread_value
    });
    println!("Waiting for thread to finish...");
    let value = handle.join().unwrap();
    println!("Thread finished with value: {}", value);

    // Multiple threads
    let mut threads = Vec::with_capacity(10);

    for i in 0..10 {
        threads.push(thread::spawn(move || {
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

/// Example of a thread that makes use of a mutex to prevent concurrent access to a resource.
fn arcs_mutex() {
    let v = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..10 {
        let v = Arc::clone(&v);
        let handle = thread::spawn(move || {
            let mut data = v.lock().unwrap();
            *data += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

// -----------------------------------------------------------------------------

pub fn receiver_example() {
    let vec = Arc::new(Mutex::new(Vec::new()));
    let av_parallel = thread::available_parallelism().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();

    let start_time = Instant::now();

    let vec_clone = Arc::clone(&vec);
    let thread = thread::spawn(move || {
        for i in 0..10 {
            vec_clone.lock().unwrap().push(i);
            tx.send(i).unwrap();
        }
    });

    let vec_clone_2 = Arc::clone(&vec);
    let thread_2 = thread::spawn(move || {
        // for i in 0..1_000_000 {
        //     vec_clone_2.lock().unwrap().push(i);
        // }
        while let Ok(i) = rx.recv() {
            vec_clone_2.lock().unwrap().push(i + 100);
        }
    });

    thread.join().unwrap();
    thread_2.join().unwrap();

    println!("Time: {}ms", start_time.elapsed().as_millis());
    println!("Vec size {}", vec.lock().unwrap().len());
    println!("Available parallelism {}", av_parallel);
    println!("Vec: {:?}", vec.lock().unwrap());
}
