error_chain! {
	errors {
        QueueIsFull {
            description("a worker queue is full")
            display("a worker queue is full")
        }
    }
}

use iron::IronError;
use iron::status::Status;

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            e @ Error(ErrorKind::QueueIsFull, _) => IronError::new(e, Status::ServiceUnavailable),
            e => IronError::new(e, Status::InternalServerError),
        }
    }
}
