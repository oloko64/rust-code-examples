use std::time::Duration;

use futures::{stream::FuturesUnordered, FutureExt, StreamExt};

pub async fn example() {
    wait_for_first_to_complete().await;
    wait_for_arbitrary_functions_to_complete().await;
    wait_for_arbitrary_functions_to_complete_with_index().await;
}

async fn wait_for_first_to_complete() {
    let vec = vec![
        foo(Duration::from_secs(3), 3),
        foo(Duration::from_secs(2), 2),
        foo(Duration::from_secs(1), 1),
        foo(Duration::from_secs(4), 4),
    ];

    // async functions for `select_all` must be pinned
    let pinned = vec.into_iter().map(|f| Box::pin(f));

    let (first, idx, _rest) = futures::future::select_all(pinned).await;

    assert_eq!(first, 1);
    assert_eq!(idx, 2);
    println!("first = {}", first);
}

async fn wait_for_arbitrary_functions_to_complete() {
    let vec = vec![
        foo(Duration::from_secs(3), 3),
        foo(Duration::from_secs(2), 2),
        foo(Duration::from_secs(1), 1),
        foo(Duration::from_secs(4), 4),
        foo(Duration::from_secs(5), 5),
        foo(Duration::from_secs(6), 6),
        foo(Duration::from_secs(7), 7),
    ];

    // create unordered collection of futures
    let futures = vec.into_iter().collect::<FuturesUnordered<_>>();

    // use collection as a stream, await only first 4 futures to complete
    let first_4 = futures.take(4).collect::<Vec<_>>().await;

    // note: any remaining futures will be cancelled automatically when the
    // stream is consumed

    // check with expected result, based on the order of completion
    assert_eq!(first_4, vec![1, 2, 3, 4]);
    println!("first_4 = {:?}", first_4);
}

async fn wait_for_arbitrary_functions_to_complete_with_index() {
    let vec = vec![
        foo(Duration::from_secs(3), 3),
        foo(Duration::from_secs(2), 2),
        foo(Duration::from_secs(1), 1),
        foo(Duration::from_secs(4), 4),
        foo(Duration::from_secs(5), 5),
        foo(Duration::from_secs(6), 6),
        foo(Duration::from_secs(7), 7),
    ];

    // create unordered collection of futures with indices
    let futures = vec
        .into_iter()
        .enumerate()
        .map(|(i, fut)| fut.map(move |res| (i, res)))
        .collect::<FuturesUnordered<_>>();

    // use collection as a stream, await only first 4 futures to complete
    let first_4 = futures.take(4).collect::<Vec<_>>().await;

    // note: any remaining futures will be cancelled automatically when the
    // stream is consumed

    // check with expected result, based on the order of completion
    assert_eq!(first_4, vec![(2, 1), (1, 2), (0, 3), (3, 4)]);
    println!("first_4 = {:?}", first_4);
}

async fn foo(time: Duration, n: usize) -> usize {
    tokio::time::sleep(time).await;
    println!("foo {} completed", n);

    n
}
