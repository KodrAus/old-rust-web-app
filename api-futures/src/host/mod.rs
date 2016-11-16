//! # Web host
//!
//! This module contains the infrastructure for hosting our web application.
//! Right now, that basically just means a router for mapping request paths
//! to a particular handler.
//!
//! # Handlers
//!
//! The approach taken to routing demonstrates how Rust's [traits]()
//! can be used to compose functional requirements.
//! We provide a few traits for building a request handler:
//!
//! - `Route` for a url path pattern to match
//! - `Get` for handling a `GET` request
//! - `Post` for handling a `POST` request
//!
//! Requests can only be routed to a handler that implements `Route` and at least
//! one of `Get` and `Post`.
//!
//! ## Examples
//!
//! A simple `GET` handler that accepts an `id` from the url:
//!
//! ```
//! # #![feature(associated_consts)]
//! # extern crate webapp_demo;
//! # use webapp_demo::host::*;
//! struct MyHandler;
//!
//! impl Route for MyHandler {
//!     const ROUTE: &'static str = "/some/path/:id";
//! }
//!
//! impl Get for MyHandler {
//!     fn call(&self, params: Params, _: Request) -> HttpFuture {
//! 		let id = params.find("id");
//!
//!         // Handle the request
//! # unimplemented!()
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! A simple `GET` and `POST` handler:
//!
//! ```
//! # #![feature(associated_consts)]
//! # extern crate webapp_demo;
//! # use webapp_demo::host::*;
//! struct MyHandler;
//!
//! impl Route for MyHandler {
//!     const ROUTE: &'static str = "/some/path/:id";
//! }
//!
//! impl Get for MyHandler {
//!     fn call(&self, params: Params, _: Request) -> HttpFuture {
//! 		let id = params.find("id");
//!
//!         // Get the thing
//! # unimplemented!()
//!     }
//! }
//!
//! impl Post for MyHandler {
//!     fn call(&self, params: Params, _: Request) -> HttpFuture {
//! 		let id = params.find("id");
//!
//!         // Update the thing
//! # unimplemented!()
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Note that we can't use different routes for the same handler.
//! This is enforced by Rust's type system.
//!
//! # Router
//!
//! The router is a service that recognises
//! url paths and passes the reqeust to an appropriate handler.
//!
//! An important thing to notice with the router is that it's immutable,
//! so we pay an upfront cost when starting the server for allocating
//! handlers, but after that they're all ready to go.
//! That makes the router very cheap to use at runtime, it only needs
//! a single reference counted pointer per request.
//!
//! ## Examples
//!
//! The router is created with using a [builder]():
//!
//! ```
//! # #![feature(associated_consts)]
//! # extern crate webapp_demo;
//! # use webapp_demo::host::*;
//! # struct MyGetHandler;
//! # impl Route for MyGetHandler {
//! # const ROUTE: &'static str = "/";
//! # }
//! # impl Get for MyGetHandler {
//! # fn call(&self, _: Params, _: Request) -> HttpFuture {
//! # unimplemented!()
//! # }
//! # }
//! # struct MyPostHandler;
//! # impl Route for MyPostHandler {
//! # const ROUTE: &'static str = "/";
//! # }
//! # impl Post for MyPostHandler {
//! # fn call(&self, _: Params, _: Request) -> HttpFuture {
//! # unimplemented!()
//! # }
//! # }
//! # fn main() {
//! let router = RouterBuilder::new()
//!     .get(MyGetHandler)
//!     .post(MyPostHandler)
//!     .build();
//! # }
//! ```
//!
//! The `get` and `post` methods expect a `T: Get + Route` and
//! `T: Post + Route` respectively.

use futures::Future;
use hyper::Error;
pub use hyper::StatusCode;
pub use hyper::server::{Server, Request, Response};

/// A bucket of parameters matched in the url path.
pub use route_recognizer::Params;

/// A future representing a `Response`.
///
/// This type is _boxed_, which means it's a pointer to some value on
/// the heap, rather than a value on the stack.
/// This is because we need to be able to support different concrete
/// types for the `Future` but can only do that when we use an
/// allocator that can deal with data of varying size, like the heap.
/// Using boxes comes with a cost in both allocation and in use, but
/// greatly simplifies our implementation.
///
/// The heap allocation is also slightly different than reference types
/// in higher level languages because there isn't a garbage collector
/// that gets poked when we allocate.
/// Once the pointer to the box is dropped, its memory on the heap is
/// freed deterministically.
pub type HttpFuture = Box<Future<Item = Response, Error = Error>>;

/// A constant url pattern.
///
/// The `Route` trait uses the unstable [_associated constants_]() feature
/// to annotate a type with a single fixed route pattern it expects.
pub trait Route {
    const ROUTE: &'static str;
}

/// A handler for a `GET` request.
pub trait Get
    where Self: Send + Sync
{
    /// Call the handler with the given url parameters and request.
    fn call(&self, params: Params, req: Request) -> HttpFuture;
}

/// A handler for a `POST` request.
pub trait Post
    where Self: Send + Sync
{
    /// Call the handler with the given url parameters and request.
    ///
    /// For posts we could also pre-buffer the request body and
    /// pass it as a parameter to this method.
    fn call(&self, params: Params, req: Request) -> HttpFuture;
}

mod router;

pub use self::router::*;
