use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use tokio::task::JoinHandle;

// https://github.com/tokio-rs/axum/tree/main/examples/websockets
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new().route("/ws", get(ws_handler));

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("`User connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    //send a ping (unsupported by some browsers) just to kick things off and get a response
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged you...");
    } else {
        println!("Could not send ping to connected user!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        return;
    }

    let (mut sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        for i in 1..=5 {
            if sender
                .send(Message::Text(format!("Hi {i} times!")))
                .await
                .is_err()
            {
                panic!("Could not send message!");
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    });

    let mut recv_task = tokio::spawn(async move {
        if let Some(message) = receiver.next().await {
            if let Ok(message) = message {
                let message_text = message
                    .into_text()
                    .expect("Could not convert message to text!");
                let message: QueryMessage =
                    serde_json::from_str(&message_text).expect("Could not deserialize message!");
                println!("Received message: {:?}", message);
            } else {
                panic!("Could not receive message!");
            }
        }
    });

    let res = tokio::try_join!(&mut send_task, &mut recv_task);

    if let Err(e) = res {
        println!("Error: {:?}", e);
        send_task.abort();
        recv_task.abort();
    }

    println!("`User disconnected.");
}

#[derive(serde::Deserialize, Debug)]
struct QueryMessage {
    name: String,
}
