
pub use crate::Request;
pub use crate::Response;

pub use crate::types::{ContentType, StatusCode, HttpMethod};

use std::collections::HashMap;
use std::{io::{Write, Read}, net::{TcpStream, TcpListener}, path::Path};

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
    response.send_file("notfound.html");
}
pub struct Router {
    pub routes: Vec<Route>,
    pub render_path:String,
    pub static_paths:Vec<String>,
    pub top_level_handlers:Vec<Vec<Handler>>
}

impl Router {
    pub fn new()->Router{
        let routes :Vec<Route>= vec![];
        Router { routes: routes,render_path:"/src/views/".to_string(), static_paths:vec![], top_level_handlers:vec![] }
    }
    pub fn listen(&mut self,port:u32){    
            let hostname = format!("127.0.0.1:{}",port.to_string());
            let listener = TcpListener::bind(hostname);
            match
                listener {
                Ok(listener)=>{
                    for stream in listener.incoming() {
                        self.handle_request(&mut stream.unwrap());    
                    }
                }
                Err(e) => panic!("Port error {:?}",e),
                }   
            }
    
    pub fn handle_request(&mut self, stream: &mut TcpStream) {
        fn handle_path(server_path: &String, client_path: &String)-> HashMap<String, String>{
            fn handle_server_path(url: &String) -> HashMap<u32, String> {
                let mut path_params: HashMap<u32, String> = HashMap::new();
        
                let mut path = url.clone().to_string();
                path.remove(0);
                let mut parts: Vec<&str> = path.split('/').collect();
                for (index, part) in parts.iter_mut().enumerate() {
                    if part.contains(':') {
                        *part = &part[1..]; // Update the value in-place
                        path_params.insert(index as u32, part.to_string());
                    }
                    println!("part: {} index: {}", part, index);
                }
                path_params
            }
            fn handle_client_path(url: &String, path_params: HashMap<u32, String> ) -> HashMap<String, String> {
                let mut params: HashMap<String, String> = HashMap::new();
                
                let mut path = url.clone().to_string();
                path.remove(0);
                let parts: Vec<&str> = path.split('/').collect();
                for (index, part) in parts.iter().enumerate() {
                    let item = path_params.get_key_value(&(index as u32));
                    match item{
                        Some(item) => {
                            println!("client index = {} part = {:?}", index, part);
                            if index as u32 == *item.0 {
                                params.insert(item.1.to_string(), part.to_string());
                            }
                        },
                        None => {
                            continue;
                        }
                    }   
                }
                params
            }
            let path_params = handle_server_path(&server_path);
            handle_client_path(&client_path, path_params)
        }

        let mut request = parse_buffer(stream);
        let mut response: Response = Response::new(self.render_path.clone(),self.static_paths.clone());
        // if &request.path.chars().last().unwrap() != &'/' {
        //     let _ = &request.path.push_str("/");
        // }
        let mut not_found = true;
        //top level handlers
        for handlers in self.top_level_handlers.iter(){
            for handler in handlers.iter(){
                if !response.finish{
                    (handler)(&mut request,&mut response);
                }
            }
        }
        
        fn check_path_params(route_path: &String, request_path: &String) -> bool{
            let route_parts: Vec<&str> = route_path.split("/").collect();
            let request_parts: Vec<&str> = request_path.split("/").collect();
            let mut state = true;
            for (index, route_part) in route_parts.iter().enumerate(){
                if !(*route_part == request_parts[index] || route_part.contains(":")) {
                    state = false;
                }
            }
            return state;
        }
        
        for route in self.routes.iter_mut() {
            let params = handle_path(&route.path, &request.path);
            request.params = params;
            println!("CHECK PARAMS = {}", check_path_params(&route.path, &request.path));
            if route.path == "*" || request.path == route.path ||  check_path_params(&route.path, &request.path) {                
                
                if route.method == HttpMethod::ALL || request.method == route.method{
                    not_found = false;
                    for handler in route.handlers.iter_mut(){
                        if !response.finish{
                            (handler)(&mut request,&mut response);
                        }
                    }
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
        //404 handler
        if not_found{
            not_found_handler(&mut response);
        }
           
        // fn write_response(stream:&mut TcpStream,response: &mut Response)->Result<(),(String)>{
        //     stream.write(response.raw_string.as_bytes()).expect_err("can't write stream");
        //     stream.flush().expect_err("can't flush stream");
        //     Ok(())
        // }

        stream.write(response.raw_string.as_bytes()).unwrap();
        stream.flush().unwrap();

        //println!("HOCAM HOCAM HOCAM \n{:?}", response);
    }
    pub fn set_render_path(&mut self,path:&str){
        self.render_path = path.to_string();
    }
    pub fn add_static_path(&mut self,path:&str){
        self.static_paths.push(path.to_string());
    }
    pub fn top_level_handler(&mut self,handlers:Vec<Handler>){
        self.top_level_handlers.push(handlers);
    }
    pub fn all(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::ALL , handlers:handlers};
        self.routes.push(route);
    }
    pub fn get(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::GET , handlers:handlers};
        self.routes.push(route);
    }
    pub fn post(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::POST , handlers:handlers};
        self.routes.push(route);
    }
    pub fn put(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::PUT , handlers:handlers};
        self.routes.push(route);
    }
    pub fn delete(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::DELETE , handlers:handlers};
        self.routes.push(route);
    }
    
    pub fn static_handler(&mut self,path:&str,a:&str)->Handler{
        self.static_paths.push(path.to_string());
        fn handler(req:&mut Request,res:&mut Response){
            let static_paths = res.static_paths.clone();
            for dir_path in static_paths.iter(){
                let path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + dir_path + &req.path;
                let file_path = Path::new(&path);
                    if file_path.is_file(){
                        res.send_static_file(&path);
                        res.finish = true;
                    }else{
                        res.send_text("DAYANAMIYORUM");
                        res.finish = true;
                    }
            }
            res.finish = true;
        }
        return handler;
    }

   
}
pub type Handler = fn(req:&mut Request,res:&mut Response);

pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handlers:Vec<Handler>
}
impl Route{
    pub fn new(){}
}

