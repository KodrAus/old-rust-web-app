//! # Errors
//! 
//! Rust doesn't have exceptions, so errors are expressed as a variant on
//! the standard library [`Result`]() type that implements the [`Error`]()
//! trait.
//! The problem with the `Error` trait is that it can be cumbersome to implement
//! manually, and leads to a lot of conversion boiletplate.
//! Luckily we have crates like [`error-chain`]() that make it really easy to
//! declare error types.
//! 
//! Errors are treated a bit differently in our web server, because HTTP has a
//! particular way of notifying the client that something went wrong.
//! So when we encounter an application error we unwrap it to a `Result::Ok`
//! variant, but with an error status code, like `StatusCode::NotFound`.
//! That way the client is properly notified that something went wrong.

use tokio_timer;

error_chain! {
    foreign_links {
        tokio_timer::TimerError, Timer;
    }

	errors {
        NoRouteMatch(route: String) {
            description("the route could not be matched to a handler")
            display("the route: '{}' could not be matched to a handler", route)
        }
        MethodNotSupported {
            description("the http method is not supported")
            display("the http method is not supported")
        }
    }
}

use host::{ Response, StatusCode };

impl From<Error> for Response {
    fn from(err: Error) -> Response {
        let kind = err.0;

        kind.into()
    }
}

impl From<ErrorKind> for Response {
    fn from(err: ErrorKind) -> Response {
        match err {
            ErrorKind::NoRouteMatch(_) => Response::new().status(StatusCode::NotFound),
            ErrorKind::MethodNotSupported => Response::new().status(StatusCode::MethodNotAllowed),
            // Catch all for any other errors, which get expressed as a 500
            _ => Response::new().status(StatusCode::InternalServerError),
        }
    }
}
