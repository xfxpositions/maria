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
use maria::{ Request, Response, Router, handler, HandlerFn, Arc, Mutex};

#[tokio::main]
async fn main(){

    //defining first handler
    let home: HandlerFn = handler!(_req, res, {
        res.send_html("<h1>Hello from Maria!</h1>");
    });

    // init the router
    let mut router = Router::new();

    // add our handler to router
    router.get("/", vec![home]);

    // that's it!
    router.listen(8080).await;
}
```

# Release notes

## 0.8.1

### Router.r#use added.

- I can't name as Router.use because Rust already has a keyword as use.
- But you can use with Router.r#use();
- Same as expressjs Router.use method.
- You can define something for all methods and all paths
- Example:
- ```rs
    router.r#use(vec![handler!(_req,_res,{
        println!("Something has came!");
    })]);
  ```
- Also, readme updated with new handler! usage

## 0.8.0

### New handler! macro for defining handlers, a quick header changes

- now you can define a handler with using handler! macro
  example:

- ```rust
    let home: HandlerFn = handler!(_req, res, {
        res.send_text("Hello world!");
    })
  ```

- 2 Examples added
- Basic function documentation added
- and that's done. also, documentation will come to next versions.

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

### Ready to release!

- New closure type handlers.
- Some quick deadlock fixes.
- Fully multithreded.
- Cargo.toml ready to be publish!
- Some warning fixes.
