#![feature(proc_macro, try_from)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate json_str;

extern crate iron;
extern crate router;

extern crate redis;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

/// Error types.
pub mod errors;

/// App model.
pub mod model;

use std::convert::TryInto;
use redis::Commands;
use iron::prelude::*;
use iron::status;
use router::Router;

use errors::*;
use model::*;

macro_rules! get_id {
    ($req:ident) => (
        $req.extensions.get::<Router>()
            .unwrap()
            .find("id")
            .unwrap_or("")
            .try_into()?;
    )
}

fn get_conn() -> Result<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection().map_err(|e| e.into())
}

fn main() {
    let mut router = Router::new();
    router.get("/person/:id", get_person, "get_person");
    router.post("/person/:id", post_person, "post_person");

    Iron::new(router).http("localhost:3000").unwrap();

    fn get_person(req: &mut Request) -> IronResult<Response> {
        let id: Id = get_id!(req);

        let conn = get_conn()?;

        // Maybe get the serialised person from Redis
        let person_data: Option<String> = conn
            .get(id.as_ref())
            .map_err(|e| Error::from(e))?;

        // Unwrap the serialised data, or return a `PersonNotFound` error
        let person_data = person_data.ok_or(Error::from(ErrorKind::PersonNotFound))?;

        // Deserialise the person
        let person: Person = serde_json::from_str(&person_data)
            .map_err(|e| Error::from(e))?;

        Ok(Response::with((status::Ok, person_data)))
    }

    fn post_person(req: &mut Request) -> IronResult<Response> {
        #[derive(Deserialize)]
        struct PostPersonCommand {
            pub name: String
        }

        let id: Id = get_id!(req);

        let conn = get_conn()?;

        let person: PostPersonCommand = serde_json::from_reader(&mut req.body)
            .map_err(|e| Error::from(e))?;

        let person = Person {
            id: id,
            name: person.name
        };

        let person_data = serde_json::to_string(&person)
            .map_err(|e| Error::from(e))?;

        conn.set(person.id.as_ref(), person_data)
            .map_err(|e| Error::from(e))?;

        Ok(Response::with(status::Ok))
    }
}
