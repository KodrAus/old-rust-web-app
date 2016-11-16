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
        Exception {
            description("something unexpected happened")
            display("something unexpected happened")
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
            _ => Response::new().status(StatusCode::InternalServerError),
        }
    }
}
