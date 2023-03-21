pub(crate) use crate::request::Request;
pub(crate) use crate::response::Response;
pub(crate) use crate::types::content_type::ContentType;
use crate::types::http_methods::HttpMethod;
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
    response.set_status_code_raw(201);
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
    pub fn handle_request(&mut self, request: &mut Request, mut stream: TcpStream) {
        println!("ROUTES LEN{}",self.routes.len());
        let mut response: Response = Response::new();
        let mut not_found = true;
        for route in self.routes.iter_mut() {
            if request.path == route.path {
                not_found = false;
                (route.handler)(request,&mut response);
                //base_handler(&mut response);
            } 
        }
        if not_found{
            not_found_handler(&mut response);
        }
        stream.write(response.raw_string.as_bytes());
        stream.flush();
        println!("HOCAM HOCAM HOCAM \n{:?}", response);
    }
    pub fn add_get(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::GET),handler:handler};
        println!("route = {},{}",route.method,route.path);
        self.routes.push(route);
    }
    pub fn post(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::POST),handler:handler};
        self.routes.push(route);
    }
    pub fn put(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::PUT),handler:handler};
        self.routes.push(route);
    }
    pub fn delete(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::DELETE),handler:handler};
        self.routes.push(route);
    }
}
// pub struct Handler {
//     request: Request,
//     response: Response,
// }

pub type Handler = fn(req:&mut Request,res:&mut Response);

pub struct Route {
    pub path: String,
    pub method: String,
    pub handler:Handler
}
impl Route {
    // pub fn new(path: &str, method: &str, request: Request, response: Response) -> Self {}
    pub fn start_handling(&self) {}
    pub fn get(path:&str,handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::GET), handler:handler}
    }
    pub fn post(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::POST), handler:handler}
    }
    pub fn put(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::PUT), handler:handler}
    }
    pub fn delete(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::get(HttpMethod::DELETE), handler:handler}
    }
}
