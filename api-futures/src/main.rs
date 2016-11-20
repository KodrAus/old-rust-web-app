#![feature(box_syntax, associated_consts)]

extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_timer;
extern crate hyper;
extern crate route_recognizer;
extern crate webapp_demo;

use std::time::Duration;
use futures::{Future, finished, lazy};
use futures_cpupool::CpuPool;
use tokio_core::reactor::Core;
use tokio_timer::Timer;
use hyper::header::ContentLength;
use webapp_demo::host::*;

/// A handler for a HTTP request.
/// 
/// This handler implements `Route + Get + Post`, so can
/// be used to handle either of those verbs.
struct MyHandler {
    cpu_pool: CpuPool,
}

impl Route for MyHandler {
    const ROUTE: &'static str = "/";
}

// 'GET /'
impl Get for MyHandler {
    fn call(&self, _: Params, _: Request) -> HttpFuture {
        let response = Response::new()
            .header(ContentLength(11u64))
            .body("Hello world".as_bytes());

        box finished(response)
    }
}

// 'POST /'
impl Post for MyHandler {
    fn call(&self, _: Params, _: Request) -> HttpFuture {
        // Do some 'expensive work' on a background thread
        let work = self.cpu_pool
            .spawn(lazy(|| {
                Timer::default()
                    .sleep(Duration::from_millis(1000))
                    .and_then(|_| finished("Hello world".as_bytes()))
            }));

        // When the work is finished, build a HTTP response
        let respond = work.and_then(|msg| {
            let response = Response::new()
                .header(ContentLength(msg.len() as u64))
                .body(msg);

            finished(response)
        });

        respond.into_http_future()
    }
}

fn main() {
    // Create a background worker pool.
    let cpu_pool = CpuPool::new(4);

    // Create a request router with our handlers.
    let router = RouterBuilder::new()
        .get(MyHandler { cpu_pool: cpu_pool.clone() })
        .post(MyHandler { cpu_pool: cpu_pool.clone() })
        .build();

    // Create a `tokio` reactor.
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Set up our server to run on the reactor.
    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Server::http(&addr).unwrap();
    let lst = server.handle(move || Ok(router.clone()), &handle).unwrap();

    println!("listening on {}", lst);

    // Run the server 4eva
    core.run(futures::empty::<(), ()>()).unwrap();
}
