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

    //adding route to router
    fn handler1(_req:&mut Request,res:&mut Response){
        res.send_file("index.html");
    }
    fn set_header(_req:&mut Request,res:&mut Response){
        res.add_header("deneme", "zibidi")
    }
    router.get("/",vec![set_header,handler1]);

    //add static serve path
    router.add_static_path("/src/static");

    //add another route to router   
    fn handler2(_req:&mut Request, res:&mut Response){
        res.send_text("deneme");
    }
    router.get("/test", vec![handler2]);
    router.get("/test2", vec![handler2.clone()]);

    //post handler example
    fn post_handler(req:&mut Request,res:&mut Response){
        res.set_status_code_raw(200);
        println!("request is => {:?}",req);
        res.send_text("post example");
    }
    router.post("/examplepost",vec![post_handler]);


    router.listen(8080);
}
