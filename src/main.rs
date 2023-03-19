use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
mod parse_route;
mod router;
use router::{parse_headers, Request, Response, Route, Router};

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    fn handle_client(mut stream: TcpStream) {
        let routes: Vec<Route> = vec![Route {
            method: "GET".to_string(),
            path: "/hello".to_string(),
            response: Response::new(),
        }];
        let mut router: Router = Router { routes };
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_string = String::from_utf8_lossy(&mut buffer[..]);

        let message = String::from("Hello world!");
        let response = format!(
            "HTTP/1.1 200 OK\nContent-Length: {}\nContent-Type: text/plain\n\n{}",
            message.len(),
            message
        );

        let (first_line, headers, headers_str, body) =
            parse_headers(request_string.to_string()).unwrap();
        let request = Request::new(request_string.to_string());
        router.handle_request(request, stream);
        // println!(
        //     "First Line: method:{},path:{},version{}",
        //     first_line.0, first_line.1, first_line.2
        // );
        // println!("Headers: {:?}", headers);
        // println!("Body: {}", body);
        // println!("Incoming request");
    }
}
