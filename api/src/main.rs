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

use iron::prelude::*;

fn main() {
    // Create a queue with a sender and receiver and max length of 1.
    let (tx, rx) = worker::queue::QueueBuilder::new()
        .with_max_len(1)
        .build();

    // Check our queue before each request to see if it's over capacity.
    let backpressure = worker::Backpressure { to_check: vec![Box::new(tx.clone())] };

    // Create a HTTP handler that will push a message and return 200.
    let mut chain = Chain::new(move |_: &mut Request| {
        tx.push(());

        Ok(Response::with((iron::status::Ok, "Hello World")))
    });

    // Wire up our backpressure handler.
    chain.link_before(backpressure);

    // Run the web server.
    // NOTE: This will start throwing 503's after 1 request right now.
    Iron::new(chain).http("localhost:3000").unwrap();
}
