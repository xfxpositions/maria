extern crate maria;

use std::{sync::Arc, time::Duration};

use futures::lock::Mutex;
use maria::{HandlerPtr, Request, Response, Router};

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    fn wait(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
        })
    }
    fn query(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {

            let req = req_base.lock().await;
            let mut res = res_base.lock().await;

            println!("{:?}", &req.path);
            let queries = &req.queries;
            println!("{:?}", queries);
            res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
        })
    }

    router.get("/test/:id", vec![wait, query]);

    fn home(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {
            let mut res = res_base.lock().await;
            let html_text = "<h1>I fuckin did it</h1>";
            res.send_html(html_text);
        })
    }

    router.get("/", vec![home]);

   router.listen(1002).await;
}
