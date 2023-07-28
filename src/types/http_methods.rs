#[derive(Debug, PartialEq)]

#[derive(Clone, Copy)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    UNKNOWN,
    ALL,
}
impl HttpMethod {
    pub fn from_string(string: &str) -> HttpMethod {
        match string {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "ALL" => HttpMethod::ALL,
            _ => HttpMethod::UNKNOWN,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::GET => "GET".to_string(),
            Self::POST => "POST".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::DELETE => "DELETE".to_string(),
            Self::ALL => "ALL".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
