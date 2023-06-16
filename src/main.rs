extern crate maria;

use maria::{Router,Request,Response};

fn main(){
    let mut router = Router::new();
    fn amk_routeu(req: &mut Request, res: &mut Response){
        res.send_text("sa naber qwe");
    }
    router.get("/",vec![amk_routeu]);
    
    router.listen(1002)
}