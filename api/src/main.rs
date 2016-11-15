//! # A Rust Webapp
//!
//! This application is an example web API written in the [Rust](https://www.rust-lang.org)
//! language.
//! If I was going to write a web app in Rust today,
//! it would probably look something like this.
//!
//! # The stack
//!
//! - [`iron`](https://docs.rs/iron/*/iron/) as the web framework
//! - [`futures`](https://docs.rs/futures/*/futures/) for asynchronous promise primitives
//! - [`crossbeam`](https://docs.rs/crossbeam/*/crossbeam/) for lock-free queues
//! - [`error-chain`](https://docs.rs/error-chain/*/error_chain/) for better error types.
//!
//! # Design
//!
//! Iron follows a middleware-based request processing design, so it's easy to extend
//! and work with.
//! This app tries to avoid _middleware soup_ by not sharing state across middleware where
//! possible.
//!
//! This app offloads work to background workers in dedicated threads for concurrency
//! or for stuff that needs to be done out-of-band with an individual request.
//! Rust only has OS threads, so we have to be stingy about when threads are spawned.

#[macro_use]
extern crate error_chain;

extern crate futures;
extern crate crossbeam;

extern crate iron;

/// Error types.
pub mod errors;

/// Domain model.
pub mod model;

/// Background worker infrastructure.
pub mod worker;

use futures::Future;
use iron::prelude::*;

type Message = futures::Complete<String>;

fn main() {
    // Create a queue with a sender and receiver and max length.
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
    Iron::new(chain).http("localhost:3000").unwrap();
    worker.join().unwrap();
}
