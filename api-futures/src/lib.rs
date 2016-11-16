//! # A Rust Web API
//! 
//! This is an example of a typical HTTP API written in Rust.
//! 
//! ## The state of the world
//! 
//! Rust already has an ecosystem of frameworks and libraries for building
//! web applications, but there's previously been one major thing missing:
//! _asynchronous io_.
//! The main HTTP library used in Rust applications, [`hyper`](), currently
//! provides non-blocking io for managing connections, but the request
//! handlers themselves are synchronous.
//! That means only a single request can be processed by one of our precious
//! request threads at a time.
//! Despite being a fast, concurrent programming language, Rust hasn't
//! performed particularly well in [microbenchmarks](), seemingly because
//! of this constraint.
//! It's an unfortunate state of affairs, because Rust has some very nicely
//! designed and supported web frameworks, like [`iron`]().
//! 
//! Do not despair though!
//! Rust has been working on an asynchronous io stack starting from the
//! bottom layer and working its way up.
//! It looks a little something like this:
//! 
//! - [`mio`]() provides a cross-platform abstraction over OS async io
//! - [`futures`]() provides primitives for promises
//! - [`tokio`]() the love child of `mio` and `futures`. Provides an
//! event loop and service for working with io asynchronously
//! - [`hyper`]() provides a `tokio` service for running a HTTP server or
//! client.
//! 
//! This stack is now just about ready for web frameworks like `iron` to adopt,
//! but at the time of writing `tokio` and `hyper` are still unreleased.
//! 
//! ## The stack
//! 
//! This web app uses the `master` branches of `futures`, `tokio` and `hyper`
//! to demonstrate what an asynchronous web server could look like in the near
//! future.

#![feature(box_syntax, associated_consts)]

extern crate futures;
extern crate hyper;
extern crate route_recognizer;
extern crate tokio_timer;

#[macro_use]
extern crate error_chain;

/// Web hosting infrastructure.
pub mod host;

/// Hosting errors.
pub mod errors;