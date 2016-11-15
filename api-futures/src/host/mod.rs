use futures::Future;
use hyper::Error;
use hyper::server::{ Request, Response };

pub use route_recognizer::Params;

pub type HttpFuture = Box<Future<Item = Response, Error = Error>>;

pub trait Get where
Self: Send + Sync {
	fn route(&self) -> &'static str;
	fn call(&self, params: Params, req: Request) -> HttpFuture;
}

pub trait Post where
Self: Send + Sync {
	fn route(&self) -> &'static str;
	fn call(&self, params: Params, req: Request) -> HttpFuture;
}

mod router;

pub use self::router::*;