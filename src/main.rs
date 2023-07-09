extern crate maria;

use maria::{Router,Request,Response};

fn main(){
    let mut router = Router::new();

    fn query(req: &mut Request, res: &mut Response){
        println!("{:?}",&req.path);
        let queries = &req.queries;
        println!("{:?}", queries);
        res.send_text(format!("given id is => {:?}", req.params.get("id")).as_str());
    }
    router.get("/test/:id",vec![query]);

    fn home(req: &mut Request, res: &mut Response){
        println!("{}", req.body);
        res.render("index.html");
    }
    router.get("/", vec![home]);
    router.listen(1002);
}