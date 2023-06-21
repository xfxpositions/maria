use crate::types::HttpMethod;

use std::collections::HashMap;

fn parse_path(path_string: &String){
    let parts = path_string.split("/");
    
}

pub fn parse_headers(
    request_string: String,
) -> Result<
    (
        (String, String, String),
        HashMap<String, String>,
        String,
        String,
        Option<HashMap<String, String>>,

    ),
    String,
> {
    let mut parts = request_string.splitn(2, "\r\n\r\n");
    let headers_str = parts.next().ok_or("")?;
    let body_str = parts.next().unwrap_or("");

    let mut headers: HashMap<String, String> = HashMap::new();
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
        headers.insert(key, value);
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

    let mut query_params = None;
    if let Some(pos) = path.find('?') {
        let query_string = &path[pos + 1..];
        query_params = Some(parse_query_params(query_string));
        // remove the query string from the path
        let path_without_query = path[..pos].to_owned();
        return Ok((
            (method.to_string(), path_without_query, http_version),
            headers,
            headers_str.to_string(),
            body_str.to_string(),
            query_params,
        ));
    }

    let mut params = HashMap::new();
    if let Some(query_params) = query_params {
        for (key, value) in query_params.iter() {
            params.insert(key.to_string(), value.to_string());
        }
    }

    Ok((
        (method.to_string(), path.to_string(), http_version),
        headers,
        headers_str.to_string(),
        body_str.to_string(),
        Some(params),
    ))
}


fn parse_query_params(query_string: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for pair in query_string.split('&') {
        if let Some((name, value)) = parse_query_pair(pair) {
            result.insert(name, value);
        }
    }
    result
}

fn parse_query_pair(pair: &str) -> Option<(String, String)> {
    let mut parts = pair.split('=');
    let name = parts.next()?.to_string();
    let value = parts.next()?.to_string();
    Some((name, value))
}


#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: HttpMethod,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub headers_raw: String,
    pub body: String,
    pub raw_string: String,
    pub queries: Option<HashMap<String, String>>,
    pub params: HashMap<String, String>
}
impl Request {
    pub fn new(request_string: String) -> Request {
        let (first_line, headers, headers_str, body,queries) =
            parse_headers(request_string.to_string()).unwrap();
        return Request {
            method: HttpMethod::from_string(first_line.0.as_str()),
            path: first_line.1,
            version: first_line.2,
            headers: headers,
            headers_raw: headers_str,
            body: body,
            raw_string: request_string,
            queries:queries,
            params: HashMap::new()
        };
    }

    pub fn get_query(&self,query_name: &str) -> Option<String> {
        if let Some(query_params) = &self.queries {
            if let Some(query_value) = query_params.get(query_name) {
                return Some(query_value.to_owned());
            }
        }
        None
    }
}
