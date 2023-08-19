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

///macro for defining handlers
/// first param is ```Request```, second is ```Response```
/// # Example
/// ```rust
///  let home: HandlerFn = handler!(req, res, {
///     res.send_html("<h2>OK</h2>");
///  });
/// ```

#[macro_export]
macro_rules! handler {
    ($response:ident, $request:ident, $body:block) => {{
        Arc::new(move |req: Arc<Mutex<Request>>, res: Arc<Mutex<Response>>| {
            Box::new(async move {
                let mut $request = res.lock().await;
                let mut $response = req.lock().await;
                
                $body
            })
        })
    }};
}