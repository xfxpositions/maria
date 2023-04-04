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
```
mod parse_route;
mod request;
mod response;
mod router;
mod types;
use request::{Request};
use router::{Response, Router};
use serde::{Serialize, Deserialize};
//importing
let mut router = Router::new();
let hello_world(_req:&mut Request, res:&mut Response){
  res.send_html("<h1>Hello world from Maria.rs!</h1>");
}
router.get("/",vec![hello_world]); // our first route
```
 

### /examples will be avaible in the future.
