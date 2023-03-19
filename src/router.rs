pub(crate) use crate::request::Request;
pub(crate) use crate::response::Response;
pub(crate) use crate::types::content_type::ContentType;
pub(crate) use crate::types::status_code::StatusCode;

use std::{fmt::Error, io::Write, net::TcpStream};
pub fn base_handler(response: &mut Response) {
    //response.headers = format!("HTTP/1.1 {} OK\nContent-Length: {}\nContent-Type: text/plain\nHost: localhost:8080\nUser-Agent: xfxWeb",response.status_code,response.body.len());
    response.send_text("Deneme".to_string());
    response.pack_response();
}
pub fn json_handler(response: &mut Response) {
    let json_string = r#"
    {
        "name": "John",
        "age": 30,
        "city": "New York"
    }
    "#;

    response.send_json(json_string.to_string());
}
pub fn not_found_handler(response: &mut Response) {
    //response.headers = String::from("Content-Type: text/plain");
    response.send_text("Not found".to_string());
    response.set_status_code(StatusCode::NotFound);
    response.pack_response();
}
pub struct Router {
    pub routes: Vec<Route>,
}
impl Router {
    pub fn handle_request(&mut self, request: Request, mut stream: TcpStream) {
        let mut response: Response = Response::new();
        for route in self.routes.iter_mut() {
            if request.path == route.path {
                json_handler(&mut response);
            } else {
                not_found_handler(&mut response);
            }
        }
        stream.write(response.raw_string.as_bytes());
        stream.flush();
        println!("HOCAM HOCAM HOCAM \n{:?}", response);
    }
}
pub struct Handler {
    request: Request,
    response: Response,
}

pub struct Route {
    pub path: String,
    pub method: String,
}
impl Route {
    // pub fn new(path: &str, method: &str, request: Request, response: Response) -> Self {}
    pub fn start_handling(&self) {}
}
