//! # Request handlers
//!
//! Iron's [`router`]() will accept a function closure that takes
//! a mutable `Request` and returns an `IronResult<Response>`.
//! The application specifies two handlers:
//!
//! - `get_person` handles `GET /person/:id`, and will get a `Person`
//! from Redis and return them as json.
//! - `post_person` handles `POST /person/:id`, and will update a
//! `Person` in Redis with a new name.

use serde_json;
use redis::{self, Commands};
use iron::prelude::*;
use iron::status;
use router::Router;

use errors::*;
use model::*;

/// Get a person by id.
///
/// This handler takes an id from the query parameters and gets
/// the corresponding person, or returns a `HTTP 404`.
pub fn get_person(req: &mut Request) -> IronResult<Response> {
    let id = get_id(&req)?;
    let conn = get_conn()?;

    let person_data = get_person_data(conn, &id)?;

    Ok(Response::with((status::Ok, person_data)))
}

#[derive(Deserialize)]
struct PostPersonCommand {
    pub name: String,
}

/// Post a new person value for an id.
///
/// This handler takes an id and `PostPersonCommand` and adds or updates
/// that person's data.
///
/// The body of the request should look something like:
///
/// ```json
/// { "name": "Some Name" }
/// ```
pub fn post_person(req: &mut Request) -> IronResult<Response> {
    let id = get_id(&req)?;
    let conn = get_conn()?;

    let person = make_person(req, id)?;

    set_person_data(conn, person)?;

    Ok(Response::with(status::Ok))
}

/// Get an `Id` from the request url params.
fn get_id(req: &Request) -> Result<Id> {
    req.extensions
        .get::<Router>()
        .unwrap()
        .find("id")
        .unwrap_or("")
        .try_into()
}

/// Get a new Redis connection.
fn get_conn() -> Result<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection().map_err(|e| e.into())
}

/// Get the data for a `Person` from Redis.
fn get_person_data(conn: redis::Connection, id: &Id) -> Result<String> {
    let person_data: Option<String> = conn.get(id.as_ref())?;
    person_data.ok_or(Error::from(ErrorKind::PersonNotFound))
}

/// Set the data for a `Person` in Redis.
fn set_person_data(conn: redis::Connection, person: Person) -> Result<()> {
    let person_data = serde_json::to_string(&person)?;

    conn.set(person.id.as_ref(), person_data)?;

    Ok(())
}

/// Get a person from the request body with an id.
fn make_person(req: &mut Request, id: Id) -> Result<Person> {
    let cmd: PostPersonCommand = serde_json::from_reader(&mut req.body)?;

    Ok(Person {
        id: id,
        name: cmd.name,
    })
}
