mod prelude;
use prelude::*;
use serde_json::json;
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
    fn middleware(req:&mut Request,_res:&mut Response){
        println!("Request method is {:?}",req.method.to_string());
    }
    router.top_level_handler(vec![middleware]);
    router.get("/",vec![set_header,handler1]);
    
    //json example
    fn json_handler(_req: &mut Request, res: &mut Response) {
        #[derive(Serialize, Debug, Deserialize)]
        struct Message {
            pub message: String,
        }
        let j = json!({
            "fingerprint": "0xF9BA143B95FF6D82",
            "location": "Menlo Park, CA"
        });
        res.send_json(&j);
        
        //struct
        let message = Message{message:"Test".to_string()};
        //res.send_json(&message); you can also do this.
    }
    router.get("/json",vec![json_handler]);

    //post handler example
    fn post_handler(req:&mut Request,res:&mut Response){
        res.set_status_code_raw(200);
        println!("request is => {:?}",req);
        res.send_text("post example");
    }

    //queries example
    fn query_test(req:&mut Request, res:&mut Response){
        let name = req.get_query("name").unwrap();

        if(name.is_empty()){
            res.send_html("<h1>No name provided in queries.</h1>")
        }else{
            res.send_html(format!("<h1>Hello, {name}</h1>").as_str());
        }
    }
    router.get("/query",vec![query_test]);

    router.post("/examplepost",vec![post_handler]);

    router.listen(443);
}
