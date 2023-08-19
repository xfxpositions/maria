use maria::{ Request, Response, Router, handler, HandlerFn, Arc, Mutex};

#[tokio::main]
async fn main(){

    //defining first handler
    let home: HandlerFn = handler!(_req, res, {
        res.send_html("<h1>Hello from Maria!</h1>");
    });
    
    // init the router
    let mut router = Router::new();

    // add our handler to router
    router.get("/", vec![home]);

    // that's it!
    router.listen(8080).await;
}