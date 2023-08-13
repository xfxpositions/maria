mod request;
mod response;
mod router;
mod types;

pub use tokio::sync::Mutex;
pub use std::sync::Arc;

pub use request::Request;
pub use response::Response;
pub use types::{ContentType, HttpMethod, StatusCode};

pub use router::{pack_handler, Handler, HandlerFn, HandlerPtr, Router};

#[macro_export]
macro_rules! handler {
    ($request:ident, $response:ident, $body:block) => {{
        Arc::new(move |req: Arc<Mutex<Request>>, res: Arc<Mutex<Response>>| {
            Box::new(async move {
                let mut $request = res.lock().await;
                let mut $response = req.lock().await;
                
                $body
            })
        })
    }};
}