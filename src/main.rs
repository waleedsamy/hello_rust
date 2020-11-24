use std::net::TcpListener;
use std::{io::prelude::*, net::TcpStream};

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();

    eprintln!("listening {}", addr);

    for conn_attempt in listener.incoming() {
        let mut conn_attempt = conn_attempt.unwrap();
        handle_conn_attempt(&mut conn_attempt)?;
    }

    Ok(())
}

fn handle_conn_attempt(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    println!("{}\n", request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
