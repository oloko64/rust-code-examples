use futures::StreamExt;

pub async fn run() {
    // create an iterator of futures to execute
    let funcs = (0..100).map(|i| async move {
        println!("Task {} started", i);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Task {} finished", i);

        return i;
    });

    // create a buffered stream that will execute up to 10 futures in parallel
    // (without preserving the order of the results)
    let stream = futures::stream::iter(funcs).buffer_unordered(10);

    // wait for all futures to complete
    let results = stream.collect::<Vec<i32>>().await;

    println!("Results: {:?}", results);
}
