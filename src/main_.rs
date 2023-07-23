extern crate maria;

use std::{sync::Arc, time::Duration};

use tokio::sync::Mutex;
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
            let html_text = "<h1>hey yo!</h1>";
            res.send_html(html_text);
        })
    }

    router.get("/", vec![home]);

    fn meryem(request: Arc<Mutex<Request>>, response: Arc<Mutex<Response>>) -> HandlerPtr{
        Box::new(async move{
            let mut res = response.lock().await;
            res.send_html("<h1>Meryem pasha</h1>");
        })
    }
    router.get("/meryem", vec![meryem]);

   router.listen(8081).await;
}
