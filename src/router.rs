use std::net::TcpStream;

pub fn base_handler(route: &mut Route) {
    route.response.headers = String::from("Content-Type: text/plain");
    route.response.status_code = 200;
    route.response.body = String::from("hello world");
}
pub fn not_found_handler(route: &mut Route) {
    route.response.headers = String::from("Content-Type: text/plain");
    route.response.status_code = 404;
    route.response.body = String::from("page not found");
}
pub struct Router {
    routes: Vec<Route>,
}
impl Router {
    pub fn handle_request(&mut self, request: Request) {
        for mut route in self.routes.iter_mut() {
            if request.path == route.path {
                base_handler(route);
            } else {
                not_found_handler(route);
            }
        }
    }
}
pub struct Handler {
    request: Request,
    response: Response,
}
pub struct Request {
    path: String,
    headers: String,
    body: String,
}
pub struct Response {
    status_code: u16,
    headers: String,
    body: String,
}

pub struct Route {
    path: String,
    method: String,
    response: Response,
}
impl Route {
    // pub fn new(path: &str, method: &str, request: Request, response: Response) -> Self {}
    pub fn start_handling(&self) {}
}
