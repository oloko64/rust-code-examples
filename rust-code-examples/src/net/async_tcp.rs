use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening for connections on port {}", 3000);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let conn = handle_connection_async(stream).await;
            if let Err(e) = conn {
                println!("An error occurred while handling connection: {}", e);
            }
        });
    }

    Ok(())
}
async fn handle_connection_async(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let buffer = &mut [0; 512];
        stream.read(buffer).await?;
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let response = "OK";
        stream.write(response.as_bytes()).await?;
    }
}
