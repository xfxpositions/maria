pub mod parse_route;
pub mod request;
pub mod response;
pub mod router;
pub mod types;

use request::Request;
use response::Response;
use serde::{Serialize, Deserialize};


mod prelude {
    pub use crate::prelude::parse_route;
    pub use crate::prelude::request;
    pub use crate::prelude::response;
    pub use crate::prelude::router;
    pub use crate::prelude::types::*;
}
