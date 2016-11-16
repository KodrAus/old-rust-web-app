//! # Quick Router
//!
//! A simple router that takes an argument of type `T` and matches it to routes.
//! This router avoids the need to box response types, so can be more efficient if
//! allocations are a problem.
//! It does require some additional boilerplate though.
//!
//! This router is best used with an `enum` as the generic parameter:
//!
//! ```
//! # use webapp_demo::host::quick_router::*;
//! enum Handlers {
//!     HandlerA,
//!     HandlerB
//! }
//!
//! let router = RouterBuilder::new()
//!     .route("/a", Handlers::HandlerA)
//!     .route("/b", Handlers::HandlerB)
//!     .build();
//!
//! match router.match_path("/a") {
//!     Some((&Handlers::HandlerA, p)) => println!("handle request for a"),
//!     Some((&Handlers::HandlerB, p)) => println!("handler request for b"),
//!     None => println!("no match: return 404"),
//! }
//! ```

use std::sync::Arc;
use route_recognizer::{ Router as Recognizer, Params };

type HttpRouter<T> = Recognizer<T>;

#[derive(Clone)]
pub struct Router<T> {
    router: Arc<Box<HttpRouter<T>>>,
}

impl <T> Router<T> {
    pub fn match_path<I>(&self, path: I) -> Option<(&T, Params)>
        where I: AsRef<str> {
        match self.router.recognize(path.as_ref()) {
            Ok(route) => Some((route.handler, route.params)),
            Err(_) => None,
        }
    }
}

pub struct RouterBuilder<T> {
    router: HttpRouter<T>
}

impl <T> RouterBuilder<T> {
    pub fn new() -> Self {
        RouterBuilder {
            router: HttpRouter::new(),
        }
    }

    pub fn route<I>(mut self, route: I, handler: T) -> Self
        where I: AsRef<str>
    {
        self.router.add(route.as_ref(), handler);

        self
    }

    pub fn build(self) -> Router<T> {
        Router {
            router: Arc::new(Box::new(self.router))
        }
    }
}