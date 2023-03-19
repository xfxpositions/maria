use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
};
mod parse_route;
mod router;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    fn handle_client(mut stream: TcpStream) {
        let routes = vec!["/zibidi", "/"];
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_string = String::from_utf8_lossy(&mut buffer[..]);
        let message = String::from("Hello world!");
        let response = format!(
            "HTTP/1.1 200 OK\nContent-Length: {}\nContent-Type: text/plain\n\n{}",
            message.len(),
            message
        );
        // println!("Incoming request {}", request_string);
        println!("Incoming request");

        stream.write(response.as_bytes());
        stream.flush();
    }
}
