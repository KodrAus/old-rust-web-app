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

/// Web handler routes.
pub mod routes;

use iron::prelude::*;
use router::Router;

fn main() {
    // Create a new Iron router
    let mut router = Router::new();

    // Get a person by id
    router.get("/person/:id", routes::get_person, "get_person");

    // Post an updated person value
    router.post("/person/:id", routes::post_person, "post_person");

    // Create the Iron server with the router and start listening
    Iron::new(router).http("localhost:1337").unwrap();
}
