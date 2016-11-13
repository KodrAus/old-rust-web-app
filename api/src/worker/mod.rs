//! # Background workers
//!
//! This module provides some bits and pieces for offloading
//! work from a request thread to some other one,
//! and maybe wait for some results to come back.
//!
//! Rust gives us a bunch of primitives and language features for
//! dealing with [concurrency](https://doc.rust-lang.org/book/concurrency.html).
//!
//! We have to be careful when throwing work onto a background queue that isn't
//! completed in the lifetime of the request because it can cause the queue to
//! chew up all our memory.
//! To get around this we apply
//! [_backpressure_](http://www.reactivemanifesto.org/glossary#Back-Pressure)
//! in the form of a high-watermark on our queue lengths.
//! When the queue goes above that watermark we start rejecting requests with a `HTTP 503` code.
//!
//! # So why have background workers?
//!
//! Rust is a language that aims to be viable for just about any platform.
//! That means it has a minimal runtime for dealing with green threading like C#'s `TaskScheduler`.
//! So we have to manages our own OS threads, which you don't want to just go and spin
//! up willy-nilly.
//!
//! That doesn't really answer the question though: _why have background workers?_
//! There are some objects that we want to keep around for longer than the lifetime of a
//! single request, like database connections.
//! We could just [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html) these things 
//! on the heap, but there are also units of work that we want to run asynchronously, 
//! like waiting for IO on said database connections.
//! Threads are great for this because they allow us to run an independent unit of work
//! as well as decouple the lifetime of stuff from a single request.
//! 
//! So we don't want everything crammed into a single request lifetime,
//! but we also don't want to spin up lots of threads.
//! The solution is a background worker; a thread that accepts a message to handle,
//! that's accessible by any number of active requests.
//! 
//! The two pieces of this puzzle are:
//! 
//! - The `queue` that's used to send a message to a worker
//! - The `unit` that's run for each message.

/// A _multi-producer, multi-consumer_ queue.
pub mod queue;

/// A _unit of work_ run in the background.
pub mod unit;