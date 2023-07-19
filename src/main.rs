extern crate maria;

use std::sync::{Arc, Mutex};

use maria::{Router,Request,Response, HandlerPtr};

#[tokio::main]
async fn main(){
    let mut router = Router::new();

    fn query(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>)-> HandlerPtr{
        Box::new(async move{
            let req = req_base.lock().unwrap();
            let mut res = res_base.lock().unwrap();
    
            println!("{:?}",&req.path);
            let queries = &req.queries;
            println!("{:?}", queries);
            res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
        })
    }

    fn deneme2(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>)-> HandlerPtr {
        Box::new(
            async move{
                let req = req_base.lock().unwrap();
                let mut res = res_base.lock().unwrap();
        
                println!("{:?}",&req.path);
                let queries = &req.queries;
                println!("{:?}", queries);
                res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
            }
        )
      
    }
    router.get("/test/:id",vec![query, deneme2]);


    
    async_std::task::block_on(router.listen(1002));
}