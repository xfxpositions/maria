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
use maria::{Router,Request,Response};

fn main(){
    let mut router = Router::new();
    
    fn hello(req: &mut Request, res: &mut Response){
        res.send_text("Hello from maria.rs");
    }
    router.get("/",vec![hello]);
    
    router.listen(1002);
    //that's it!
}
```

### /examples will be avaible in the future.
