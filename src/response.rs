use crate::types::{content_type::ContentType, status_code::StatusCode};
#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub content_type: ContentType,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub raw_string: String,
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
                201 => "Created",
                202 => "Accepted",
                203..=230 => "Success",
                401 => "Unauthorized",
                402 => "Payment Required",
                403 => "Forbidden",
                404 => "Not Found",
                405..=440 => "Client Error",
                500 => "Internal Server Error",
                503 => "Unavaible",
                _ => "Unknown",
            },
            headers_str,
            self.body
        );
        self.raw_string = response_str;
    }
    pub fn send_text(&mut self, text: String) {
        if(self.status_code == 0){
            self.set_status_code(StatusCode::Ok);
        }
        self.set_content_type(ContentType::Text);
        self.body = text;
        self.pack_response();
    }
    pub fn send_json(&mut self, json: String) {
        if(self.status_code == 0){
            self.set_status_code(StatusCode::Ok);
        }
        self.set_content_type(ContentType::Json);
        self.body = json;
        self.pack_response();
    }
    pub fn send_html(&mut self, html: String) {
        if(self.status_code == 0){
            self.set_status_code(StatusCode::Ok);
        }
        self.set_content_type(ContentType::Html);
        self.body = html;
        self.pack_response();
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
