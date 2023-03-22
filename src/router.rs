pub(crate) use crate::request::Request;
pub(crate) use crate::response::Response;
pub(crate) use crate::types::content_type::ContentType;
use crate::types::http_methods::HttpMethod;
pub(crate) use crate::types::status_code::StatusCode;
use std::{fmt::Error, io::{Write, Read}, net::{TcpStream, TcpListener}, os};
pub fn parse_buffer(stream:&mut TcpStream)->Request{
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request_string = String::from_utf8_lossy(&mut buffer[..]);
    let request = Request::new(request_string.to_string());
    return request;
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
    response.set_status_code(StatusCode::NotFound);
    response.send_text("Not found");
}
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new()->Router{
        let routes :Vec<Route>= vec![];
        Router { routes: routes }
    }
    pub fn listen(&mut self,port:u32){    
        let hostname = format!("127.0.0.1:{}",port.to_string());
        let listener = TcpListener::bind(hostname).unwrap();
        
        for stream in listener.incoming() {
            self.handle_request(&mut stream.unwrap());    
        }

    }
    pub fn handle_request(&mut self, stream: &mut TcpStream) {
        let mut request = parse_buffer(stream);
        let mut response: Response = Response::new();
        let mut not_found = true;
        for route in self.routes.iter_mut() {
            if request.path == route.path{
                if request.method.to_string() == route.method.to_string(){
                    not_found = false;
                    (route.handler)(&mut request,&mut response);
                }else{
                    not_found = false;
                    fn handler(request: &mut Request, response: &mut Response){
                        let body = format!("No avaible path for {} method, you can try another methods",request.method.to_string());
                        response.send_text(body.as_str());
                    }
                    handler(&mut request,&mut response);
                }
            } 
        }
        if not_found{
            not_found_handler(&mut response);
        }
        // fn write_response(stream:&mut TcpStream,response: &mut Response)->Result<(),(String)>{
        //     stream.write(response.raw_string.as_bytes()).expect_err("can't write stream");
        //     stream.flush().expect_err("can't flush stream");
        //     Ok(())
        // }

        stream.write(response.raw_string.as_bytes());
        stream.flush();

        println!("HOCAM HOCAM HOCAM \n{:?}", response);
    }
    pub fn get(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::GET ,handler:handler};
        println!("route = {},{}",route.method.to_string(),route.path);
        self.routes.push(route);
    }
    pub fn post(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::POST ,handler:handler};
        self.routes.push(route);
    }
    pub fn put(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::PUT ,handler:handler};
        self.routes.push(route);
    }
    pub fn delete(&mut self,path:&str,handler:Handler){
        let route = Route { path: path.to_string(), method: HttpMethod::DELETE ,handler:handler};
        self.routes.push(route);
    }
}

pub type Handler = fn(req:&mut Request,res:&mut Response);

pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handler:Handler
}
impl Route {
    // pub fn new(path: &str, method: &str, request: Request, response: Response) -> Self {}
    pub fn start_handling(&self) {}
    pub fn get(path:&str,handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::GET, handler:handler}
    }
    pub fn post(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::POST, handler:handler}
    }
    pub fn put(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::PUT, handler:handler}
    }
    pub fn delete(path:&str, handler:Handler)->Route{
        Route { path: path.to_string(), method: HttpMethod::DELETE, handler:handler}
    }
}
