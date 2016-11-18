error_chain! {
	errors {
        NotAnId {
            description("the given value isn't a valid id")
            display("the given value isn't a valid id")
        }
    }
}

use iron::IronError;
use iron::status::Status;

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        match err {
            e => IronError::new(e, Status::InternalServerError),
        }
    }
}
