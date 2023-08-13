use maria::{Router, Response, Request, HandlerFn, Mutex, Arc, handler};

#[tokio::main]
async fn main(){
    
    let home2: HandlerFn = handler!(res, _req, {
        res.add_header("Content-Type", "text/plain; charset=utf-8");
        res.send_text("tengri türük budun korundasın");
    });

    //define first handler
    let home: HandlerFn = handler!(res, req, {
        println!("{:?}", req.params);
            
        //res.send_html(format!("Hello from maria.rs!, {:?}",req.params.get("id")).as_str());
        res.send_html("<h2>OK</h2>");
 
    });

    //create a new router for our app
    let mut router = Router::new();
    
    router.get("/test/:*testparam", vec![home]);
    router.get("/tengri", vec![home2]);
    //that's it!
    router.listen(8080).await;
}