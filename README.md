# Maria.rs, a simple Rust framework like ExpressJs

## Why?

- Maria is developed for web developers who switched to Rust from Nodejs, you can easily adapt Maria.rs code.

## what it includes?

- [x] Router
- [x] Json handle
- [x] Handler
- [x] Async
- [x] Traits
- [ ? ] Multithreading

## Examples
- Hello world example
```rust
use maria::{Router,Request,Response};

#[tokio::main]
fn main(){
    let mut router = Router::new();
    
    fn home(req_base: Arc<Mutex<Request>>, res_base: Arc<Mutex<Response>>) -> HandlerPtr {
        Box::new(async move {
            let mut res = res_base.lock().await;
            res.send_html("<h1>I fuckin did it</h1>");
        })
    }
    router.get("/",vec![home]);
    
    router.listen(1002).await;
    //that's it!
}
```

### /examples will be avaible in the future.
