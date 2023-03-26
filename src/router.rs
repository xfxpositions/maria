pub(crate) use crate::request::Request;
pub(crate) use crate::response::Response;
use crate::types::http_methods::HttpMethod;
pub(crate) use crate::types::status_code::StatusCode;
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
    pub static_paths:Vec<String>
}

impl Router {
    pub fn new()->Router{
        let routes :Vec<Route>= vec![];
        Router { routes: routes,render_path:"/src/views/".to_string(), static_paths:vec![] }
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
        let mut request = parse_buffer(stream);
        let mut response: Response = Response::new(self.render_path.clone(),self.static_paths.clone());
        let mut not_found = true;
        
        //first check the path is actually reffering a static file

        let mut is_static = false;
        for dir_path in self.static_paths.iter(){
            let path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + dir_path + &request.path;
            let file_path = Path::new(&path);
                if file_path.is_file(){
                    is_static = true;
                    response.send_static_file(&path);
                }
        }
        if !is_static{
            for route in self.routes.iter_mut() {
                if route.path == "*" || request.path == route.path{
                    if route.method == HttpMethod::ALL || request.method == route.method{
                        not_found = false;
                        for handler in route.handlers.iter_mut(){
                            (handler)(&mut request,&mut response);
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
        }
           
        // fn write_response(stream:&mut TcpStream,response: &mut Response)->Result<(),(String)>{
        //     stream.write(response.raw_string.as_bytes()).expect_err("can't write stream");
        //     stream.flush().expect_err("can't flush stream");
        //     Ok(())
        // }

        stream.write(response.raw_string.as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("HOCAM HOCAM HOCAM \n{:?}", response);
    }
    pub fn set_render_path(&mut self,path:&str){
        self.render_path = path.to_string();
    }
    pub fn add_static_path(&mut self,path:&str){
        self.static_paths.push(path.to_string());
    }
    pub fn all(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::ALL , handlers:handlers};
        println!("route = {},{}",route.method.to_string(),route.path);
        self.routes.push(route);
    }
    pub fn get(&mut self,path:&str, handlers:Vec<Handler>){
        let route = Route { path: path.to_string(), method: HttpMethod::GET , handlers:handlers};
        println!("route = {},{}",route.method.to_string(),route.path);
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
    
    pub fn static_handler(&mut self,path:&str)->Handler{
        self.static_paths.push(path.to_string());
        fn handler(req:&mut Request,res:&mut Response){
            let static_paths = res.static_paths.clone();
            for dir_path in static_paths.iter() {
                let path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + dir_path + &req.path;
                let file_path = Path::new(&path);
                if file_path.is_file(){
                    res.send_file(&path);
                }   
            }
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

