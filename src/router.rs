
pub use crate::Request;
pub use crate::Response;

pub use crate::types::{ContentType, StatusCode, HttpMethod};

use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::{path::Path, sync::Arc, sync::Mutex};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncWrite, io::AsyncRead, net::{TcpListener as AsyncTcpListener, TcpStream}};

pub async fn parse_buffer(stream: &mut TcpStream) -> Result<Request, Box<dyn Error>> {
    let mut buffer = Vec::new();

    loop {
        let mut chunk = vec![0; 1024]; // Create a temporary chunk buffer
        match stream.read(&mut chunk).await {
            Ok(bytes_read) if bytes_read > 0 => {
                println!("chunk is => {:?}", bytes_read);
                chunk.resize(bytes_read, 0);
                buffer.extend_from_slice(&chunk);

                if bytes_read < 1024 {
                    break;
                }
            }
            Ok(_) => {
                // Continue reading
            }
            Err(e) => {
                println!("something happened while reading the buffer {:?}", e);
                return Err(Box::new(e));
            }
        }
    }

    let request_string = String::from_utf8_lossy(&buffer);
    let request = Request::new(request_string.to_string());
    Ok(request)
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
pub fn not_found_handler(response: Arc<Mutex<Response>>) -> HandlerPtr {
    Box::new(async move{
        let mut response_locked = response.lock().unwrap();
        response_locked.set_status_code(StatusCode::NotFound);
        response_locked.render("notfound.html");
    })  
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
        Router { routes: routes,render_path:"./src/views/".to_string(), static_paths:vec![], top_level_handlers:vec![] }
    }
    pub async fn listen(&mut self,port:u32){    
        let hostname = format!("127.0.0.1:{}",port.to_string());
        let listener = AsyncTcpListener::bind(hostname).await;
        match
            listener {
            Ok(listener)=>{

                loop {
                    let (stream, _) = listener.accept().await.unwrap();
                    
                    tokio::spawn(Box::pin(self.handle_request(&mut stream)));
                    
                }

                
            }
            Err(e) => panic!("Port error {:?}",e),
        }   
    }
    
    pub async fn handle_request(&mut self, stream: &mut tokio::net::TcpStream) {
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

        let req = parse_buffer(stream).await.unwrap();
        let request_base = Arc::new(Mutex::new(req));
        let response_base = Arc::new(Mutex::new(Response::new(self.render_path.clone(), self.static_paths.clone())));

      

        let mut not_found = true;
        let a = response_base.clone();
        let locked_response = a.lock().unwrap();
        //top level handlers
        for handlers in self.top_level_handlers.iter() {
            for handler in handlers.iter() {
                if !locked_response.finish {
                    Box::into_pin((handler)(request_base.clone(), response_base.clone())).await;
                }
            }
        }
        
        fn check_path_params(route_path: &String, request_path: &String) -> bool{
            let route_parts: Vec<&str> = route_path.split("/").collect();
            let request_parts: Vec<&str> = request_path.split("/").collect();
            let mut state = true;
            for (index, route_part) in route_parts.iter().enumerate(){
                if(index < request_parts.len()){
                    if !(*route_part == request_parts[index] || route_part.contains(":")) {
                        state = false;
                    }
                }
            }
            return state;
        }
        let mut request = request_base.lock().unwrap();
        let response = response_base.lock().unwrap();
        
        for route in self.routes.iter_mut() {
           


            let params = handle_path(&route.path, &request.path);
            
            request.params = params;
            if route.path == "*" || request.path == route.path ||  check_path_params(&route.path, &request.path) {                
                
                if route.method == HttpMethod::ALL || request.method == route.method{
                    not_found = false;
                    for handler in route.handlers.iter_mut(){
                        if !response.finish{
                            Box::into_pin((handler)(request_base.clone(),response_base.clone())).await;
                        }
                    }
                }else{
                    not_found = false;
                    fn handler(request: Arc<Mutex<Request>>, response: Arc<Mutex<Response>>)-> HandlerPtr{
                        Box::new(async move{
                            let mut response_locked = response.lock().unwrap();
                            let request_locked = request.lock().unwrap();
    
                            let body = format!("No avaible path for {} method, you can try another methods", request_locked.method.to_string());
                            response_locked.send_text(body.as_str());
                        })
                    }
                    Box::into_pin(handler(request_base.clone(), response_base.clone())).await;
                }
            } 
        }
        //404 handler
        if not_found{
            Box::into_pin(not_found_handler(response_base.clone())).await;
        }

        stream.write(response.raw_string.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();
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
    pub fn get(&mut self, path: &str, handler_functions: Vec<HandlerFn>) {
        let mut handlers: Vec<Handler> = Vec::new();
    
        for handler_fn in handler_functions {
            handlers.push(Box::new(handler_fn));
        }
    
        let route = Route {
            path: path.to_string(),
            method: HttpMethod::GET,
            handlers: handlers,
        };
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
    
    // pub fn static_handler(&mut self,path:&str,a:&str)->Handler{
    //     self.static_paths.push(path.to_string());
    //     fn handler(req:&mut Request,res:&mut Response){
    //         let static_paths = res.static_paths.clone();
    //         for dir_path in static_paths.iter(){
    //             let path = std::env::current_dir().unwrap().to_str().unwrap().to_owned() + dir_path + &req.path;
    //             let file_path = Path::new(&path);
    //                 if file_path.is_file(){
    //                     res.send_static_file(&path);
    //                     res.finish = true;
    //                 }else{
    //                     res.send_text("DAYANAMIYORUM");
    //                     res.finish = true;
    //                 }
    //         }
    //         res.finish = true;
    //     }
    //     return handler;
    // }

   
}
pub fn pack_handler(func: Handler) -> Box<Handler>{
    Box::new(func)
}
//pub type Handler = fn(req:&mut Request,res:&mut Response);
pub type HandlerFn = fn(Arc<Mutex<Request>>, Arc<Mutex<Response>>) -> Box<dyn Future<Output = ()> + Send>;
pub type Handler = Box<HandlerFn>;
pub type HandlerPtr =  Box<dyn Future<Output = ()>>;
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handlers:Vec<Handler>
}
impl Route{
    pub fn new(){}
}

