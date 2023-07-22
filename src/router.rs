pub use crate::Request;
pub use crate::Response;

pub use crate::types::{ContentType, HttpMethod, StatusCode};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener as AsyncTcpListener, TcpStream};

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

pub async fn not_found_handler(response: Arc<Mutex<Response>>) -> HandlerPtr {
    Box::new(async move {
        let mut response_locked = response.lock().await;
        response_locked.set_status_code(StatusCode::NotFound);
        response_locked.render("notfound.html");
    })
}

fn match_route_path(route_path: &String, request_path: &String) -> bool {
    let route_parts: Vec<&str> = route_path.split("/").collect();
    let request_parts: Vec<&str> = request_path.split("/").collect();

    let mut state = true;

    for (index, route_part) in route_parts.iter().enumerate() {
        if index < request_parts.len() {
            if !(*route_part == request_parts[index] || route_part.contains(":")) {
                state = false;
            }
        }
    }

    return state;
}

fn extract_path_parameters(url: &String) -> HashMap<u32, String> {
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

fn extract_params_from_client_path(
    url: &String,
    path_params: HashMap<u32, String>,
) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();

    let mut path = url.clone().to_string();
    path.remove(0);
    let parts: Vec<&str> = path.split('/').collect();
    for (index, part) in parts.iter().enumerate() {
        let item = path_params.get_key_value(&(index as u32));
        match item {
            Some(item) => {
                if index as u32 == *item.0 {
                    params.insert(item.1.to_string(), part.to_string());
                }
            }
            None => {
                continue;
            }
        }
    }
    params
}

fn path_params(server_path: &String, client_path: &String) -> HashMap<String, String> {
    let path_params = extract_path_parameters(&server_path);
    let params = extract_params_from_client_path(&client_path, path_params);

    return params;
}

async fn no_method_handler(
    request: Arc<Mutex<Request>>,
    response: Arc<Mutex<Response>>,
) -> HandlerPtr {
    Box::new(async move {
        let mut response_locked = response.lock().await;
        let request_locked = request.lock().await;

        let body = format!(
            "No avaible path for {} method, you can try another methods",
            request_locked.method.to_string()
        );
        response_locked.send_text(body.as_str());
    })
}

pub async fn handle_top_level_handlers(
    handlers: Vec<Vec<Handler>>,
    res: Arc<Mutex<Response>>,
    req: Arc<Mutex<Request>>,
) {
    let response_lock = res.lock().await;

    if handlers.len() > 0 {
        for handlers in handlers.iter() {
            for handler in handlers.iter() {
                let cloned_response = Arc::clone(&res);
                let cloned_request = Arc::clone(&req);

                let handler_fn: Pin<Box<dyn Future<Output = ()> + Send>> =
                    Box::into_pin(handler(cloned_request, cloned_response));
                handler_fn.await;
            }
        }
    }
}

pub async fn handle_route_handlers(
    routes: &mut Vec<Route>,
    res: Arc<Mutex<Response>>,
    req: Arc<Mutex<Request>>,
) {
    let mut not_found = true;
    let cloned_response = res.clone();
    let cloned_request = req.clone();

   
    for route in routes.iter_mut() {
        let mut request_lock = cloned_request.lock().await;
        let req_path = request_lock.path.clone();
        let req_method = request_lock.method.to_string();
        let params = path_params(&route.path, &req_path);
        request_lock.params = params;
        drop(request_lock);

        if route.path == "*"
            ||  req_path == route.path
            || match_route_path(&route.path, &req_path)
        {
            if route.method == HttpMethod::ALL || req_method == route.method.to_string() {
                not_found = false;
                for handler in route.handlers.iter_mut() {
                    let cloned_response = Arc::clone(&res);
                    let cloned_request = Arc::clone(&req);
                    //Box::into_pin(handler(request_base.clone(), response_base.clone()).await).await;
                    // drop(request_lock);
                    // drop(response_lock);
                    {
                        Box::into_pin(Box::into_pin(Box::new(handler))(
                            cloned_request,
                            cloned_response,
                        ))
                        .await;
                    }
                }
            } else {
                not_found = false;
                let cloned_response = Arc::clone(&res);
                let cloned_request = Arc::clone(&req);

                Box::into_pin(no_method_handler(cloned_request, cloned_response).await).await;
            }
        }
    }

    //404 handler
    if not_found {
        Box::into_pin(not_found_handler(cloned_response).await).await;
    }
}
async fn end_stream(stream: &mut TcpStream, response: Arc<Mutex<Response>>) {
    println!("end stream");
    let buffer = {
        let lock: tokio::sync::MutexGuard<'_, Response> = response.lock().await;
        lock.raw_string.as_bytes().to_vec()
    };
    println!("buffer is {:?}", buffer);
    stream.write(&buffer).await.unwrap();
    stream.flush().await.unwrap();
    println!("stream flushed");
}

pub struct Router {
    pub routes: Vec<Route>,
    pub render_path: String,
    pub static_paths: Vec<String>,
    pub top_level_handlers: Vec<Vec<Handler>>,
}

impl Router {
    pub fn new() -> Router {
        let routes: Vec<Route> = vec![];
        Router {
            routes: routes,
            render_path: "./src/views/".to_string(),
            static_paths: vec![],
            top_level_handlers: vec![],
        }
    }
    pub async fn listen(self, port: u32) {
        let d = Arc::new(Mutex::new(self));
        let hostname = format!("127.0.0.1:{}", port.to_string());
        let listener = AsyncTcpListener::bind(hostname).await;
        match listener {
            Ok(listener) => {
                while let Ok((mut stream, _)) = listener.accept().await {
                    let s = d.clone();

                    tokio::task::spawn(async move {
                        let mut lock = s.lock().await;
                        lock.handle_request(&mut stream).await;
                    });

                }
            }
            Err(e) => panic!("Listening error {:?}", e),
        }
    }

    pub async fn handle_request(&mut self, stream: &mut tokio::net::TcpStream) {
        let req: Request = parse_buffer(stream).await.unwrap();
        let res = Response::new("qwe".to_string(), vec!["qweqwe".to_string()]);

        let request_base = Arc::new(Mutex::new(req));
        let response_base = Arc::new(Mutex::new(res));

        let res_clone = Arc::clone(&response_base);
        let req_clone = Arc::clone(&request_base);

        let handle_top_level_handlers_task = handle_top_level_handlers(
            self.top_level_handlers.clone(),
            res_clone.clone(),
            req_clone.clone(),
        );

        let handle_route_handlers_task =
            handle_route_handlers(&mut self.routes, res_clone.clone(), req_clone.clone());

        let end_stream_task = end_stream(stream, response_base);

        // tokio::join! işlevini kaldırıyoruz, yerine tokio::select! ile handle_top_level_handlers_task,
        // handle_route_handlers_task ve end_stream_task task'larının tamamının bitmesini bekliyoruz.

        //tokio::join!(handle_route_handlers_task, end_stream_task);

        handle_top_level_handlers_task.await;
        handle_route_handlers_task.await;
        end_stream_task.await;

        println!("All tasks completed.");
    }

    pub fn set_render_path(&mut self, path: &str) {
        self.render_path = path.to_string();
    }
    pub fn add_static_path(&mut self, path: &str) {
        self.static_paths.push(path.to_string());
    }
    pub fn top_level_handler(&mut self, handlers: Vec<Handler>) {
        self.top_level_handlers.push(handlers);
    }
    pub fn all(&mut self, path: &str, handlers: Vec<Handler>) {
        let route = Route {
            path: path.to_string(),
            method: HttpMethod::ALL,
            handlers: handlers,
        };
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

    pub fn post(&mut self, path: &str, handlers: Vec<Handler>) {
        let route = Route {
            path: path.to_string(),
            method: HttpMethod::POST,
            handlers: handlers,
        };
        self.routes.push(route);
    }
    pub fn put(&mut self, path: &str, handlers: Vec<Handler>) {
        let route = Route {
            path: path.to_string(),
            method: HttpMethod::PUT,
            handlers: handlers,
        };
        self.routes.push(route);
    }
    pub fn delete(&mut self, path: &str, handlers: Vec<Handler>) {
        let route = Route {
            path: path.to_string(),
            method: HttpMethod::DELETE,
            handlers: handlers,
        };
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
pub fn pack_handler(func: Handler) -> Box<Handler> {
    Box::new(func)
}
//pub type Handler = fn(req:&mut Request,res:&mut Response);
pub type HandlerFn =
    fn(Arc<Mutex<Request>>, Arc<Mutex<Response>>) -> Box<dyn Future<Output = ()> + Send + 'static>;
pub type Handler = Box<HandlerFn>;
pub type HandlerPtr = Box<dyn Future<Output = ()> + Send + 'static>;
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub handlers: Vec<Handler>,
}
impl Route {
    pub fn new() {}
}
