#![feature(proc_macro, try_from)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate json_str;

extern crate iron;
extern crate router;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

/// Error types.
pub mod errors;

/// App model.
pub mod model;

use std::convert::TryInto;
use iron::prelude::*;
use iron::status;
use router::Router;

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

fn main() {
    let mut router = Router::new();
    router.get("/person/:id", get_person, "get_person");
    router.post("/person/:id", post_person, "post_person");

    Iron::new(router).http("localhost:3000").unwrap();

    fn get_person(req: &mut Request) -> IronResult<Response> {
        let id: Id = get_id!(req);

        Ok(Response::with((status::Ok, id.as_ref())))
    }

    fn post_person(req: &mut Request) -> IronResult<Response> {
        let id: Id = get_id!(req);

        Ok(Response::with((status::Ok, id.as_ref())))
    }
}
