use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};
mod parse_route;
mod request;
mod response;
mod router;
mod types;
use request::{parse_headers, Request};
use router::{Response, Route, Router};
use types::http_methods::HttpMethod;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

    fn handle_client(mut stream: TcpStream) {
        let routes: Vec<Route> = vec![Route {
            method: HttpMethod::get(HttpMethod::GET),
            path: "/hello".to_string(),
        }];

        let mut router: Router = Router { routes };
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_string = String::from_utf8_lossy(&mut buffer[..]);
        println!("{request_string}");
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
