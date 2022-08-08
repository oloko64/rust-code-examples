mod arg_parser;
mod utils;

use std::sync::Arc;

use arg_parser::Args;
use clap::Parser;
use reqwest::Response;

fn main() {
    let args_parsed = Args::parse();
    send_requests(args_parsed);
}

#[tokio::main]
async fn send_requests(args: Args) {
    let arc_args = Arc::new(args);
    let pool_blocks = calc_req_blocks(arc_args.request_number, arc_args.pool_size);

    println!(
        "Starting {} requests to {}...",
        arc_args.request_number, &arc_args.url
    );
    println!(
        "{} {} requests will be made in {:?} pool blocks, starting now...\n",
        arc_args.request_number, &arc_args.type_request, &pool_blocks
    );

    for index in 0..pool_blocks.len() {
        let mut tasks = Vec::new();
        for req in 0..pool_blocks[index] {
            let cloned_arc_args = Arc::clone(&arc_args);
            let task = tokio::spawn(async move {
                let res = match cloned_arc_args.type_request.as_str() {
                    "GET" => make_get_request(&cloned_arc_args.url).await,
                    "POST" => make_post_request(&cloned_arc_args.url, &cloned_arc_args.body).await,
                    _ => panic!("Invalid request type. Must be GET or POST."),
                };
                match res {
                    Ok(_) => (),
                    Err(err) => println!("Request n.: {} - {}.", req, err),
                }
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await.unwrap();
        }
        println!(
            "Pool n.: {} of {} - Done | {} requests were sent.",
            index + 1,
            pool_blocks.len(),
            pool_blocks[index]
        );
    }
    println!("\nSent {} requests.", arc_args.request_number);
}

fn calc_req_blocks(number: u32, pool_size: u32) -> Vec<u32> {
    let mut pool_blocks = Vec::new();
    let division = number / pool_size;
    let remainder = number % pool_size;

    for _ in 0..division {
        pool_blocks.push(pool_size);
    }
    if remainder > 0 {
        pool_blocks.push(remainder);
    }
    pool_blocks
}

async fn make_get_request(url: &str) -> Result<Response, reqwest::Error> {
    reqwest::get(url).await
}

async fn make_post_request(
    url: &str,
    body_req: &Option<String>,
) -> Result<Response, reqwest::Error> {
    let body = match body_req {
        Some(body) => body.to_string(),
        None => "".to_string(),
    };
    let client = reqwest::Client::new();
    let json = utils::parse_to_json(&body).expect("Invalid JSON.");
    client.post(url).json(&json).body(body).send().await
}
