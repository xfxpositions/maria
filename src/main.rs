use maria::{Router, Response, Request, HandlerFn, Mutex, Arc, handler};

#[tokio::main]
async fn main(){

    //define first handler
    let home: HandlerFn = handler!(req, res, {
        println!("{:?}", req.params);
            
        //res.send_html(format!("Hello from maria.rs!, {:?}",req.params.get("id")).as_str());
        res.send_html("<h2>OK</h2>");
 
    });

    //create a new router for our app
    let mut router = Router::new();

    router.r#use(vec![handler!(_req,_res,{
        println!("Something has came!");
    })]);
    
    router.get("/test/:*testparam", vec![home]);
    //that's it!
    router.listen(8080).await;
}