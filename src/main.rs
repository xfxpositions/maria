extern crate maria;

use std::collections::HashMap;

use maria::{Router,Request,Response};

fn main(){
    let mut router = Router::new();
    fn hello(req: &mut Request, res: &mut Response){
        println!("{:?}",&req.headers);
        res.send_text("Hello from maria.rs");
    }
    router.get("/",vec![hello]);
    
    router.listen(1002);
}