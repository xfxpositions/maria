use std::{fmt::Error, io::Write, net::TcpStream};

pub fn base_handler(response: &mut Response) {
    //response.headers = format!("HTTP/1.1 {} OK\nContent-Length: {}\nContent-Type: text/plain\nHost: localhost:8080\nUser-Agent: xfxWeb",response.status_code,response.body.len());
    response.send_text("Deneme".to_string());
    response.pack_response();
}
pub fn not_found_handler(response: &mut Response) {
    //response.headers = String::from("Content-Type: text/plain");
    response.headers = vec![("Content-Type".to_string(), "text/plain".to_string())];
    response.status_code = 404;
    response.body = String::from("page not found");
}
pub struct Router {
    pub routes: Vec<Route>,
}
impl Router {
    pub fn handle_request(&mut self, request: Request, mut stream: TcpStream) {
        let mut response: Response = Response::new();
        for route in self.routes.iter_mut() {
            if request.path == route.path {
                base_handler(&mut response);
            } else {
                base_handler(&mut response);
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
pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>,
    headers_raw: String,
    body: String,
    raw_string: String,
}
impl Request {
    pub fn new(request_string: String) -> Request {
        let (first_line, headers, headers_str, body) =
            parse_headers(request_string.to_string()).unwrap();
        return Request {
            method: first_line.0,
            path: first_line.1,
            version: first_line.2,
            headers: headers,
            headers_raw: headers_str,
            body: body,
            raw_string: request_string,
        };
    }
    pub fn pack_response(response: &mut Response) {
        let headers_str = response
            .headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<String>>()
            .join("\r\n");

        let response_str = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            response.status_code,
            match response.status_code {
                200 => "OK",
                404 => "Not Found",
                _ => "Unknown",
            },
            headers_str,
            response.body
        );
        response.raw_string = response_str;
    }
}

pub fn parse_headers(
    request_string: String,
) -> Result<
    (
        (String, String, String),
        Vec<(String, String)>,
        String,
        String,
    ),
    String,
> {
    let mut parts = request_string.splitn(2, "\r\n\r\n");
    let headers_str = parts.next().ok_or("")?;
    let body_str = parts.next().unwrap_or("");

    let mut headers: Vec<(String, String)> = vec![];
    let mut first_line = None;

    for (i, line) in headers_str.lines().enumerate() {
        if i == 0 {
            first_line = Some(line.to_owned());
            continue;
        }
        let mut parts = line.splitn(2, ": ");
        let key = parts
            .next()
            .ok_or(format!("Invalid header: {}", line))?
            .to_owned();
        let value = parts
            .next()
            .ok_or(format!("Invalid header: {}", line))?
            .to_owned();
        headers.push((key, value));
    }
    let first_line = first_line.ok_or("Invalid request: no first line found")?;
    let mut first_line_parts = first_line.split_whitespace();

    let method = first_line_parts
        .next()
        .ok_or("Invalid request: no method found")?
        .to_owned();
    let path = first_line_parts
        .next()
        .ok_or("Invalid request: no path found")?
        .to_owned();
    let http_version = first_line_parts
        .next()
        .ok_or("Invalid request: no HTTP version found")?
        .to_owned();

    println!("first_line: {}", first_line);
    println!("headers: {:?}", headers);
    println!("body: {}", body_str);

    println!(
        "http_method: {}, path:{}, version:{}",
        method, path, http_version
    );
    return Ok((
        (method.to_string(), path.to_string(), http_version),
        headers,
        headers_str.to_string(),
        body_str.to_string(),
    ));
}

#[derive(Debug)]
enum ContentType {
    Html,
    Json,
    Text,
    Unknown,
}
impl ContentType {
    fn get(content_type: ContentType) -> String {
        match content_type {
            ContentType::Html => "text/html".to_string(),
            ContentType::Json => "application/json".to_string(),
            ContentType::Text => "text/plain".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

#[derive(Debug)]
enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
}
impl StatusCode {
    fn get(status_code: StatusCode) -> u16 {
        match status_code {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::BadRequest => 401,
            _ => 200,
        }
    }
}

#[derive(Debug)]
pub struct Response {
    status_code: u16,
    content_type: ContentType,
    headers: Vec<(String, String)>,
    body: String,
    raw_string: String,
}
impl Response {
    pub fn new() -> Response {
        return Response {
            status_code: 0u16,
            content_type: ContentType::Unknown,
            headers: vec![],
            body: String::new(),
            raw_string: String::new(),
        };
    }
    pub fn pack_response(&mut self) {
        let headers_str = self
            .headers
            .iter()
            .map(|(name, value)| format!("{}: {}", name, value))
            .collect::<Vec<String>>()
            .join("\r\n");

        let response_str = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status_code,
            match self.status_code {
                200 => "OK",
                404 => "Not Found",
                _ => "Unknown",
            },
            headers_str,
            self.body
        );
        self.raw_string = response_str;
    }
    pub fn send_text(&mut self, text: String) {
        self.set_content_type(ContentType::Text);
        self.set_status_code(StatusCode::Ok);
        self.body = text;
    }

    pub fn set_content_type(&mut self, content_type: ContentType) {
        let content_type_string = ContentType::get(content_type).to_string();
        self.headers
            .push(("Content-Type".to_string(), content_type_string))
    }
    pub fn set_status_code(&mut self, status_code: StatusCode) {
        self.status_code = StatusCode::get(status_code);
    }
    pub fn set_status_code_raw(&mut self, status_code: u16) {
        self.status_code = status_code;
    }
}

pub struct Route {
    pub path: String,
    pub method: String,
    pub response: Response,
}
impl Route {
    // pub fn new(path: &str, method: &str, request: Request, response: Response) -> Self {}
    pub fn start_handling(&self) {}
}
