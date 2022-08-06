
use reqwest::Response;

#[tokio::main]
async fn main() {
    let url = "https://fastcruddev.simbioseventures.com/";
    let num_of_requests = 10000;
    let mut tasks = Vec::new();

    println!("Starting {} requests to {}...", num_of_requests, url);

    for i in 0..num_of_requests {
        let task = tokio::spawn(async move {
            let res = make_request(url).await;
            match res {
                Ok(_) => print!("."),
                Err(err) => println!("{} - {}", i, err)
            }
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await.unwrap();
    }
    println!("\nMade {} requests", num_of_requests);
}

async fn make_request(url: &str) -> Result<Response, reqwest::Error> {
    reqwest::get(url).await
}
