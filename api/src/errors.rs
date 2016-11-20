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
            e @ Error(ErrorKind::PersonNotFound, _) => IronError::new(e, Status::NotFound),
            e => IronError::new(e, Status::InternalServerError),
        }
    }
}
