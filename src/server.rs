use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let html = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // blocks until all bytes are written to connection

    println!("{}", String::from_utf8_lossy(&buffer[..]));
}