use futures::Future;
use hyper::Error;
use hyper::server::{Request, Response};

pub use route_recognizer::Params;

pub type HttpFuture = Box<Future<Item = Response, Error = Error>>;

pub trait Route {
    const ROUTE: &'static str;
}

pub trait Get
    where Self: Send + Sync
{
    fn call(&self, params: Params, req: Request) -> HttpFuture;
}

pub trait Post
    where Self: Send + Sync
{
    fn call(&self, params: Params, req: Request) -> HttpFuture;
}

mod router;

pub use self::router::*;

pub mod quick_router;