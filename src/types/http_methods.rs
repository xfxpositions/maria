pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
impl HttpMethod {
    pub fn get(http_method: HttpMethod) -> String {
        match http_method {
            Self::GET => "GET".to_string(),
            Self::POST => "POST".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::DELETE => "DELETE".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
