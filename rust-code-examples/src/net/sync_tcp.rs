use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn run() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    println!("Listening for connections on port {}", 3000);

    while let Ok((stream, _)) = listener.accept() {
        std::thread::spawn(|| {
            let conn = handle_connection(stream);
            if let Err(e) = conn {
                println!("An error occurred while handling connection: {}", e);
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut buffer = [0; 512];
        stream.read(&mut buffer)?;
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

        let response = "OK";
        stream.write(response.as_bytes())?;
    }
}
