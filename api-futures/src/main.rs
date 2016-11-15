#![feature(box_syntax, associated_consts)]

extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;
extern crate hyper;
extern crate route_recognizer;
extern crate webapp_demo;

use std::time::Duration;
use futures::{ Future, finished, lazy };
use futures_cpupool::CpuPool;
use tokio_timer::Timer;
use hyper::StatusCode;
use hyper::header::ContentLength;
use hyper::server::{ Server, Request, Response };
use webapp_demo::host::*;

struct Echo {
    cpu_pool: CpuPool
}

impl Route for Echo { 
    const ROUTE: &'static str = "/"; 
}

impl Get for Echo {
    fn call(&self, _: Params, _: Request) -> HttpFuture {
        // Do some 'expensive work' on a background thread
        let work = self.cpu_pool
            .spawn(lazy(|| {
                Timer::default()
                    .sleep(Duration::from_millis(1000))
                    .and_then(|_| finished("Hello world".as_bytes()))
            }));

        // When the work is finished, build a HTTP response
        let respond = work
            .then(|msg| {
                let response = match msg {
                    Ok(msg) => {
                        Response::new()
                            .header(ContentLength(msg.len() as u64))
                            .body(msg)
                    },
                    Err(_) => {
                        Response::new()
                            .status(StatusCode::InternalServerError)
                    }
                };

                finished(response)
            });

        box respond
    }
}

fn main() {
    let cpu_pool = CpuPool::new(4);

    let router = RouterBuilder::new()
        .get(Echo { cpu_pool: cpu_pool.clone() })
        .build();

    let addr = "127.0.0.1:1337".parse().unwrap();
    let server = Server::http(&addr).unwrap();
    let (lst, server) = server.standalone(move || Ok(router.clone())).unwrap();

    println!("listening on {}", lst);

    server.run();
}
