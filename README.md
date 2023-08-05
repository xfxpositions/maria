# Maria.rs, a simple Rust framework like ExpressJs

## Why?

- Maria is developed for web developers who switched to Rust from Nodejs, you can easily adapt Maria.rs code.

## what it includes?

- [x] Router
- [x] Json handle
- [x] Handler
- [x] Async
- [x] Traits
- [x] Multithreading

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

# Release notes

## 0.7.6

### Path params wildcart added.

- path params /:\*param_key added.
- wildcart can take any of params
- for example: file/:\*path -> file/folderpath/anotherfolder/file.txt
- params: \*path: folderpath/anotherfolder/file.txt

## 0.7.5

### A quick bugfix?

- The problem is that it drains the routes when it goes into the handling function.
- But the drain is getting all inside the vec into another vec.
- So this problem causes 404 after getting a response in route.
- Also deleted a small debug log.

## 0.7.4

### What's new?

- New closure type handlers.
- Some quick deadlock fixes.
- Fully multithreded.
- Cargo.toml ready to be publish!
- Some warning fixes.

### /examples will be avaible in the future.
