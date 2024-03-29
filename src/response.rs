use std::{fs, path::Path};

use serde::Serialize;

use crate::types::{ContentType, StatusCode};
#[derive(Debug)]
pub struct Response {
    pub status_code: u16,
    pub content_type: ContentType,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub raw_string: String,
    pub static_paths: Vec<String>,
    render_path: String,
    pub finish: bool,
}
impl Response {
    pub fn new(render_path: String, static_paths: Vec<String>) -> Response {
        return Response {
            status_code: 0u16,
            content_type: ContentType::Unknown,
            headers: vec![],
            body: String::new(),
            raw_string: String::new(),
            static_paths: static_paths,
            render_path: render_path,
            finish: false,
        };
    }
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()));
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
    fn send_setup(&mut self) {
        if self.status_code == 0 {
            self.set_status_code(StatusCode::Ok);
        }
        self.add_header("powered-by", "maria.rs")
    }
    pub fn send_text(&mut self, text: &str) {
        self.send_setup();
        self.add_header("Content-Length", text.len().to_string().as_str());
        self.set_content_type(ContentType::Text);
        self.body = text.to_string();
        self.pack_response();
    }
    pub fn send_json<T: Serialize>(&mut self, data: T) {
        match serde_json::to_string(&data) {
            Ok(json_str) => {
                self.send_setup();
                self.set_content_type(ContentType::Json);
                self.add_header("Content-Length", json_str.len().to_string().as_str());
                self.body = json_str;
                self.pack_response();
            }
            Err(e) => {
                self.set_status_code_raw(500);
                self.send_text(format!("Error serializing JSON data: {:?}", e).as_str());
            }
        }
    }
    pub fn send_html(&mut self, html: &str) {
        self.send_setup();
        self.add_header("Content-Length", html.len().to_string().as_str());
        self.set_content_type(ContentType::Html);
        self.body = html.to_string();
        self.pack_response();
    }
    pub fn send_file(&mut self, filename: &str) {
        let base_path = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            + self.render_path.as_str();
        let file = fs::read_to_string(base_path + filename);
        match file {
            Ok(file) => {
                self.send_setup();
                self.set_content_type(ContentType::Html);
                self.add_header("Content-Length", file.len().to_string().as_str());
                self.body = file;
                self.pack_response();
            }
            Err(e) => {
                self.send_text(format!("Err: can't find file {} err: {:?}", filename, e).as_str());
            }
        }
    }
    pub fn send_static_file(&mut self, path: &str) {
        let file = fs::read_to_string(path.to_string());
        match file {
            Ok(file) => {
                self.send_setup();
                self.set_content_type(ContentType::Unknown);
                self.add_header("Content-Length", file.len().to_string().as_str());
                self.body = file;
                self.pack_response();
            }
            Err(e) => {
                self.send_text(format!("Err: can't find file {} err: {:?}", path, e).as_str());
            }
        }
    }
    pub fn render(&mut self, file_name: &str) {
        let file_path = Path::new(&self.render_path).join(file_name);

        let file = fs::read_to_string(&file_path);
        match file {
            Ok(file) => {
                self.send_setup();
                self.set_content_type(ContentType::Html);
                self.add_header("Content-Length", file.len().to_string().as_str());
                self.body = file;
                self.pack_response();
            }
            Err(e) => {
                self.send_text(
                    format!(
                        "Err: an error occured to opening file {:?} in render path, err: {:?} ",
                        &file_path.to_str(),
                        e
                    )
                    .as_str(),
                );
            }
        }
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
