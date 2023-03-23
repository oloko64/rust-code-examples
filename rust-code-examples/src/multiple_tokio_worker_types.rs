use futures::future::join_all;
use reqwest::Error;
use tokio::task::JoinSet;

fn main() {
    println!("Hello, world!");

    run_with_multiple_workers().unwrap();
    println!("Done with multiple workers (4) (default)");
    run_with_single_worker().unwrap();
    println!("Done with single worker (1)");
    run_with_single_worker_no_task().unwrap();
    println!("Done with single worker (1) no task");
}

#[tokio::main(worker_threads = 4)]
async fn run_with_multiple_workers() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = JoinSet::new();

    for _ in 0..100 {
        tasks.spawn(make_request());
    }

    while let Some(task) = tasks.join_next().await {
        if let Err(e) = task {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn run_with_single_worker() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = JoinSet::new();

    for _ in 0..100 {
        tasks.spawn(make_request());
    }

    while let Some(task) = tasks.join_next().await {
        if let Err(e) = task {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn run_with_single_worker_no_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();

    for _ in 0..100 {
        tasks.push(make_request());
    }

    join_all(tasks).await;

    Ok(())
}

async fn make_request() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let _ = client.get("https://www.google.com").send().await?;
    Ok(())
}
