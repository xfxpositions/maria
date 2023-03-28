mod parse_route;
mod request;
mod response;
mod router;
mod types;
use request::{Request};
use router::{Response, Router};

fn main() {
    println!("Hello, world!");
    //creating router
    let mut router = Router::new();
    fn a(req:&mut Request, res:&mut Response){
        println!("TOP LEVEL HANDLER {}",req.method.to_string());       
    }
    //adding route to router
    fn handler1(_req:&mut Request,res:&mut Response){
        res.send_file("index.html");
    }
    fn set_header(_req:&mut Request,res:&mut Response){
        res.add_header("deneme", "zibidi")
    }
    fn middleware(req:&mut Request,res:&mut Response){
        println!("Request method is: {}",req.method.to_string());
    }
    router.top_level_handler(vec![middleware]);
    router.get("/",vec![set_header,handler1]);
    //add static serve path
    router.add_static_path("/src/static");

    //post handler example
    fn post_handler(req:&mut Request,res:&mut Response){
        res.set_status_code_raw(200);
        println!("request is => {:?}",req);
        res.send_text("post example");
    }
    router.post("/examplepost",vec![post_handler]);


    router.listen(8080);
}
