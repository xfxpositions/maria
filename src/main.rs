extern crate maria;

use maria::{Router,Request,Response};

fn main(){
    let mut router = Router::new();
    fn hello(req: &mut Request, res: &mut Response){
        println!("{:?}",&req.path);
        let queries = &req.queries;
        println!("{:?}", req.params);

        res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
    }
    router.get("/deneme/:id",vec![hello]);
    
    router.listen(1002);
