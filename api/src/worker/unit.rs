//! # Unit of work
//!
//! This is a very simple wrapper around an OS thread that will pop
//! messages off a queue and run some function over them.
//! Right now it doesn't really make any assumptions.

use std::thread::{self, JoinHandle};
use std::marker::PhantomData;
use worker::queue::Consumer;

/// A background worker.
///
/// This type is generic over the _context_, _message_ and _unit function_.
/// The _context_ is some mutable state the worker has access to when handling a message.
/// The idea is to capture anything that needs to outlive a single message, like a
/// database connection, in the _context_.
/// The _message_ is the submitted input that's popped off a queue.
/// The _unit function_ is the function that's run for each message on the queue.
///
/// The worker doesn't deal with errors, _panicking_ is the root of all evil and will
/// bring an application to a halt because of backpressure.
/// So instead of _ever_ calling `.unwrap()` in a unit of work, either complete a future
/// in the message with an appropriate error, or log it and move on.
pub struct Worker<C, M, F> {
    _c: PhantomData<C>,
    _m: PhantomData<M>,
    _f: PhantomData<F>,
}

impl<C, M, F> Worker<C, M, F>
    where C: 'static + Send,
          M: 'static + Send,
          F: 'static + Send + FnMut(&mut C, M) -> ()
{
    /// Spawn a new worker that will start popping messages of the given queue consumer.
    pub fn spawn(rx: Consumer<M>, ctx: C, unit: F) -> JoinHandle<()> {
        let mut ctx = ctx;
        let mut unit = unit;

        thread::spawn(move || {
            loop {
                if let Some(msg) = rx.try_pop() {
                    unit(&mut ctx, msg);
                }
            }
        })
    }
}
