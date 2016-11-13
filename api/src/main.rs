//! # A Rust Webapp
//!
//! This application is an example web API written in the [Rust](https://www.rust-lang.org)
//! language.
//! If I was going to write a web app in Rust today,
//! it would probably look something like this.

#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate crossbeam;

extern crate iron;

/// Error types.
pub mod errors;

/// Background worker infrastructure.
pub mod worker;

use futures::Future;
use iron::prelude::*;

type Message = futures::Complete<String>;

fn main() {
    // Create a queue with a sender and receiver and max length of 1.
    let (tx, rx) = worker::queue::QueueBuilder::new()
        .with_max_len(500)
        .build();

    // Spawn a background worker that just completes requests with a message.
    let worker =
        worker::unit::Worker::spawn(rx, String::from("Hello World"), |ctx, msg: Message| {
            msg.complete(ctx.to_owned());
        });

    // Check our queue before each request to see if it's over capacity.
    let backpressure = worker::Backpressure::new().add_queue(tx.clone());

    // Create a HTTP handler that will push a message and return 200.
    let mut chain = Chain::new(move |_: &mut Request| {
        // Create a future, give the complete end to the worker and wait.
        // This isn't a very useful example, but lets me check how this
        // is going to work in practice.
        let (c, p) = futures::oneshot();
        tx.push(c);
        let result = p.wait().unwrap();

        Ok(Response::with((iron::status::Ok, result)))
    });

    // Wire up our backpressure handler.
    chain.link_before(backpressure);

    // Run the web server.
    // NOTE: This will start throwing 503's after 1 request right now.
    Iron::new(chain).http("localhost:3000").unwrap();
    worker.join().unwrap();
}
