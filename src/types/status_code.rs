#[derive(Debug)]
pub enum StatusCode {
    Ok,
    NotFound,
    BadRequest,
}
impl StatusCode {
    pub fn get(status_code: StatusCode) -> u16 {
        match status_code {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::BadRequest => 401,
            _ => 200,
        }
    }
}
