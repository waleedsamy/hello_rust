use std::{fs, net::TcpListener};
use std::{io::prelude::*, net::TcpStream};

///
/// ```no_run
///  curl 'http://localhost:8080/'
/// ```
fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).unwrap();

    eprintln!("listening {}", addr);

    for conn_attempt in listener.incoming() {
        handle_conn_attempt(conn_attempt?)?;
    }

    Ok(())
}

fn handle_conn_attempt(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    println!("request:\n{}", request);

    let content = fs::read_to_string("hello.html")?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Lenght: {}\r\n\r\n{}",
        content.len(),
        content,
    );

    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
