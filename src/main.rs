use maria::{Router, Response, Request, HandlerFn, Mutex, Arc};

#[tokio::main]
async fn main(){

    //define first handler
    let home: HandlerFn = Arc::new(move |req: Arc<Mutex<Request>>, res: Arc<Mutex<Response>>|{
        Box::new(async move{
            let mut res = res.lock().await;
            let req = req.lock().await;

            println!("{:?}", req.params);
            
            //res.send_html(format!("Hello from maria.rs!, {:?}",req.params.get("id")).as_str());
            res.send_html("<h2>OK</h2>");
        })
    });
    
    //create a new router for our app
    let mut router = Router::new();
    
    router.get("/test/:123", vec![home]);

    //that's it!
    router.listen(8080).await;
}