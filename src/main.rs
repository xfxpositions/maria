use std::sync::Arc;

use maria::{Router, Response, Request, HandlerPtr};
use mongodb::Client;
use tokio::sync::Mutex;

extern crate maria;

#[tokio::main]
async fn main(){
    let db_uri = "";
    let client = mongodb::Client::with_uri_str(db_uri).await.unwrap(); 
    let db = Arc::new(client.database("test"));

    let get_db = move ||{
        db.clone()
    };
    let deneme2 = move |res: Arc<Mutex<Request>>, req: Arc<Mutex<Response>>|{
        Box::new(async move{
            let db = get_db();
        })
    };
    
    fn deneme()-> HandlerPtr{
        let db = get_db();
        Box::new(async move{
            
        })
    }

    let mut router = Router::new();
    
    router.get("/", vec![deneme2]);
    
    
    router.listen(8080).await;
}