extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate hyper;
extern crate route_recognizer;

use std::sync::Arc;
use std::time::Duration;
use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;
use hyper::{Get, Post, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Server, Service, Request, Response};
use route_recognizer::Router;

type HttpRouter = Router<Arc<Box<HttpHandler>>>;
type HttpFuture = Box<Future<Item = Response, Error = hyper::Error>>;
type HttpHandler = Service<Request = Request, Response = Response, Error = hyper::Error, Future = HttpFuture> + Send + Sync;

#[derive(Clone)]
struct Base {
    router: Arc<HttpRouter>
}

impl Service for Base {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = HttpFuture;

    fn call(&self, req: Request) -> Self::Future {
        match self.router.recognize(req.path().unwrap()) {
            Ok(route) => route.handler.call(req),
            Err(_) => Box::new(futures::finished(Response::new().status(StatusCode::NotFound)))
        }
    }
}

#[derive(Clone)]
struct Echo {
    workers: CpuPool
}

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = HttpFuture;

    fn call(&self, req: Request) -> Self::Future {
        if req.method() != &Get {
            return Box::new(futures::finished(Response::new().status(StatusCode::MethodNotAllowed)));
        }

        Box::new(self.workers
            .spawn(futures::lazy(|| {
                let timer = Timer::default();

                timer.sleep(Duration::from_millis(1000))
                        .and_then(|_| futures::finished("Hello world".as_bytes()))
            }))
            .then(|msg|
                match msg {
                    Ok(msg) => futures::finished(Response::new()
                        .header(ContentLength(msg.len() as u64))
                        .body(msg)),
                    Err(_) => futures::finished(Response::new().status(StatusCode::InternalServerError))
                }))
    }
}

fn main() {
    let workers = CpuPool::new(4);

    let mut router: HttpRouter = Router::new();
    router.add("/", Arc::new(Box::new(Echo { workers: workers.clone() })));

    let router = Arc::new(router);

    let server = Server::http(&"127.0.0.1:1337".parse().unwrap()).unwrap();
    let (listening, server) = server.standalone(move || Ok(Base { router: router.clone() })).unwrap();
    println!("Listening on http://{}", listening);
    server.run();
}
