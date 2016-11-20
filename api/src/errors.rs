//! # Errors
//!
//! Rust doesn't have exceptions, so errors are expressed as a variant on
//! the standard library [`Result`]() type that implements the [`Error`]()
//! trait.
//! The problem with the `Error` trait is that it can be cumbersome to implement
//! manually, and leads to a lot of conversion boiletplate.
//! Luckily we have crates like [`error-chain`]() that make it really easy to
//! declare error types.

use redis;
use serde_json;

error_chain! {
    foreign_links {
        redis::RedisError, RedisError;
        serde_json::Error, JsonError;
    }
	errors {
        NotAnId {
            description("the given value isn't a valid id")
            display("the given value isn't a valid id")
        }
        PersonNotFound {
            description("the requested person doesn't exist")
            display("the requested person doesn't exist")
        }
    }
}

use iron::IronError;
use iron::status::Status;

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            e @ Error { kind: ErrorKind::PersonNotFound, state: _ } => IronError::new(e, Status::NotFound),
            e => IronError::new(e, Status::InternalServerError),
        }
    }
}
