//! # Worker queue
//!
//! This is a simple wrapper around a thread-safe _Multi Producer, Multi Consumer_ queue.
//! The actual queue implementation is provided by [`crossbeam`]().
//!
//! If no maximum length is set then this queue is completely non-blocking.
//! If a maximum length is set then each push and pop operation will lock
//! a counter with the current length.
//!
//! This queue provides a simple [back-pressure]() mechanism by exposing a max length parameter.
//! The length isn't enforced when pushing messages to keep things simple, but it's expected that
//! a caller will check the length and decide what to do about it before pushing a message.
//! It turns out this is a good enough approach when you're dealing with a closed system,
//! like our API where it's easy to just write an Iron [middleware]()
//! to check queues on each request.
//!
//! # Examples
//!
//! Build a queue with a recommended max length of `500`:
//!
//! ```
//! # use ::worker::queue::*;
//! let (tx, rx) = QueueBuilder::new().with_max_len(500).build();
//! ```
//!
//! Build a queue with no recommended max length:
//!
//! ```
//! # use ::worker::queue::*;
//! let (tx, rx) = QueueBuilder::new().build();
//! ```
//!
//! Some things that would be nice to support here would be resilient queues.
//! So that way messages are also written to disk, and a message isn't
//! considered _pushed_ until it's sitting on disk.
//! In that case, you'd also have to make your message handlers _idempotent_,
//! so they can deal with a system failure that causes them to handle the
//! same message multiple times.

use std::sync::{Arc, Mutex};
use crossbeam::sync::SegQueue;

/// The producing end of a queue.
///
/// This structure can be safely cloned and shared amongst threads.
#[derive(Debug, Clone)]
pub struct Producer<T>(Arc<SegQueue<T>>, Option<(usize, Arc<Mutex<usize>>)>);

/// The consuming end of a queue.
///
/// This structure can be safely cloned and shared amongst threads.
#[derive(Debug, Clone)]
pub struct Consumer<T>(Arc<SegQueue<T>>, Option<Arc<Mutex<usize>>>);

/// Builder for a new queue.
pub struct QueueBuilder<T> {
    max_len: Option<usize>,
    _t: ::std::marker::PhantomData<T>,
}

impl<T> QueueBuilder<T> {
    /// Create a new queue builder.
    pub fn new() -> Self {
        QueueBuilder {
            max_len: None,
            _t: ::std::marker::PhantomData,
        }
    }

    /// Set a recommended maximum length for this queue.
    pub fn with_max_len(mut self, max_len: usize) -> Self {
        self.max_len = Some(max_len);

        self
    }

    /// Convert this builder into a `Producer`, `Consumer` pair.
    pub fn build(self) -> (Producer<T>, Consumer<T>) {
        let queue = Arc::new(SegQueue::new());

        let len_ctr = self.max_len.map(|max_len| (max_len, Arc::new(Mutex::new(0))));

        let tx = Producer(queue.clone(), len_ctr.clone());
        let rx = Consumer(queue, len_ctr.map(|(_, len)| len));

        (tx, rx)
    }
}

impl<T> Producer<T> {
    /// Push a message onto the queue.
    ///
    /// This doesn't enforce the maximum length before pushing, so if you don't check `is_full`
    /// then the queue length can grow indefinitely.
    pub fn push(&self, msg: T) {
        if let Some((_, ref len)) = self.1 {
            let mut len = len.lock().unwrap();
            *len += 1;
        }

        self.0.push(msg)
    }

    /// Check whether the queue is 'full'.
    ///
    /// The idea here is to keep the queues as simple as possible.
    /// So at the start of a request, you can poll the worker queues and see if any are full.
    /// At that stage you can decide what to do about it, which could be one of a number of things:
    ///
    /// 1. Ignore it and keep pushing messages
    /// 1. Block until the length goes down
    /// 1. Bork the request
    pub fn is_full(&self) -> bool {
        if let Some((max_len, ref len)) = self.1 {
            let len = len.lock().unwrap();
            *len >= max_len
        } else {
            false
        }
    }
}

impl<T> Consumer<T> {
    /// Try pop a message from the queue.
    pub fn try_pop(&self) -> Option<T> {
        if let Some(msg) = self.0.try_pop() {
            if let Some(ref len) = self.1 {
                let mut len = len.lock().unwrap();
                *len -= 1;
            }

            Some(msg)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mpmc_push_pop_max_len() {
        let (tx, rx) = QueueBuilder::new().with_max_len(1).build();
        let (tx2, rx2) = (tx.clone(), rx.clone());

        tx.push(());
        tx2.push(());

        assert_eq!(true, tx.is_full());
        assert_eq!(true, tx2.is_full());

        let _ = rx.try_pop().unwrap();
        let _ = rx2.try_pop().unwrap();

        assert_eq!(false, tx.is_full());
        assert_eq!(false, tx2.is_full());
    }

    #[test]
    fn mpmc_push_pop() {
        let (tx, rx) = QueueBuilder::new().build();
        let (tx2, rx2) = (tx.clone(), rx.clone());

        tx.push(());
        tx2.push(());

        let pop1 = rx.try_pop();
        let pop2 = rx2.try_pop();

        assert!(pop1.is_some() && pop2.is_some());
    }
}
