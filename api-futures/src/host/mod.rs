use futures::Future;
use hyper::Error;
use hyper::server::{ Request, Response };

pub use route_recognizer::Params;

pub type HttpFuture = Box<Future<Item = Response, Error = Error>>;

pub trait Service where 
Self: Send + Sync {
	fn route(&self) -> &'static str;

    fn call(&self, params: Params, req: Request) -> HttpFuture;
}

pub mod router;