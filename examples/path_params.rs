use maria::{Router, Response, Request, HandlerFn, Mutex, Arc, handler};

#[tokio::main]
async fn main(){
    
    let path_params: HandlerFn = handler!(req, res, {
        println!("{:?}", req.params);
            
        //res.send_html(format!("Hello from maria.rs!, {:?}",req.params.get("id")).as_str());
        res.send_html("<h2>OK</h2>");
 
    });

    //create a new router for our app
    let mut router = Router::new();
    
    router.get("/test/:*testparam", vec![path_params]);
    
    router.listen(8080).await;
}