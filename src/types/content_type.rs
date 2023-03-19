#[derive(Debug)]
pub enum ContentType {
    Html,
    Json,
    Text,
    Unknown,
}
impl ContentType {
    pub fn get(content_type: ContentType) -> String {
        match content_type {
            ContentType::Html => "text/html".to_string(),
            ContentType::Json => "application/json".to_string(),
            ContentType::Text => "text/plain".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
