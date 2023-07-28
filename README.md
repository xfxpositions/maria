# Maria.rs, a simple Rust framework like ExpressJs

## Why?

- Maria is developed for web developers who switched to Rust from Nodejs, you can easily adapt Maria.rs code.

## what it includes?

- [x] Router
- [x] Json handle
- [x] Handler
- [ ] Traits
- [ ] Multithreading

## Examples

- Hello world example

```rust
use maria::{Router, Response, Request, HandlerFn, Mutex, Arc};

#[tokio::main]
async fn main(){

    //define first handler
    let home: HandlerFn = Arc::new(move |req: Arc<Mutex<Request>>, res: Arc<Mutex<Response>>|{
        Box::new(async move{
            let mut res = res.lock().await;
            res.send_html("Hello from maria.rs!");
        })
    });

    //create a new router for our app
    let mut router = Router::new();

    router.get("/", vec![home]);

    //that's it!
    router.listen(8080).await;
}
```

### /examples will be avaible in the future.
