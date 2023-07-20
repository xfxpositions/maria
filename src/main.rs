extern crate maria;

use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use maria::{HandlerPtr, Request, Response, Router};

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    fn wait(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
        })
    }
    fn query(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let req = req_base.lock().await;
            let mut res = res_base.lock().await;

            println!("{:?}", &req.path);
            let queries = &req.queries;
            println!("{:?}", queries);
            res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
        })
    }

    router.get("/test/:id", vec![query]);

   router.listen(1002).await;
}
