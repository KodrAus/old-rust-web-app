//! # App model
//!
//! This is the implementation of our simple application model.
//! We have a very simple `Person`, which can be serialised or
//! deserialised.
//! Rust's `serde` serialisation framework requires a structure to
//! explicitely opt-in to serialisation, so it's possible to enforce
//! your application model can't be sent across application boundaries.
//! We don't bother with that in this case, because there isn't much
//! a `Person` can do.
//! For serious projects though, it's worth considering how Rust's strong
//! type system can be leveraged to enforce constraints.

use std::result::Result as StdResult;
use serde::{de, Deserialize, Deserializer};
use errors::*;

pub use std::convert::{TryInto, TryFrom};

/// A person's Id
///
/// The id is a non-blank string value.
/// We can potentially construct an id from a borrowed string, so
/// long as that string isn't empty.
/// If it is, we return an error.
/// This invariant is enforced wherever an id is used, including
/// deserialisation.
///
/// Rust's type system enforces an `Id` value can't be mutated, because
/// it doesn't expose any public data.
/// We can only access the underlying value through an immutable borrow.
///
/// # Creating `Id` values
///
/// We use the unstable `TryFrom` trait to maybe convert a borrowed
/// string into an `Id`.
/// If the string is `""`, then the result is an `ErrorKind::NotAnId`.
/// Otherwise we clone the string contents and store it as an id value.
///
/// Creating an id:
///
/// ```
/// # use model::*;
/// let id = Id::try_from("an id").unwrap();
/// ```
///
/// # Getting `Id` values
///
/// We can get an immutable reference to the underlying id value as
/// a string.
///
/// ```
/// # use model::*;
/// # let id = Id::try_from("an id").unwrap();
/// let id_value: &str = id.as_ref();
/// ```
///
/// We implement deserialisation for the `Id` manually, so it will
/// enforce our invariant that the value must not be empty.
/// This is an interesting point when it comes to data migration,
/// because it may well be the case one day that we want to enforce
/// new and incompatible invariants on our models.
/// As far as the domain is concerned, the data that hydrates it
/// will either need to be upgraded to the new rules or the model
/// will need to cope with new invalid state.
/// The generic deserialisation method gives us a good place to look
/// at upgrading data.
#[derive(Debug, PartialEq, Serialize)]
pub struct Id(String);

impl<'a> TryFrom<&'a str> for Id {
    type Err = Error;

    fn try_from(id: &'a str) -> StdResult<Id, Self::Err> {
        match id {
            "" => Err(ErrorKind::NotAnId.into()),
            _ => Ok(Id(id.to_string())),
        }
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deserialize for Id {
    fn deserialize<D>(deserializer: &mut D) -> StdResult<Id, D::Error>
        where D: Deserializer
    {
        struct IdVisitor;

        impl de::Visitor for IdVisitor {
            type Value = Id;

            fn visit_str<E>(&mut self, value: &str) -> StdResult<Id, E>
                where E: de::Error
            {
                value.try_into().map_err(|e| E::custom(format!("Failed to parse id: {}", e)))
            }
        }

        deserializer.deserialize_str(IdVisitor)
    }
}

/// A person.
///
/// The `Person` is our basic application model.
/// There isn't a whole lot to them here; a person has an `id` and
/// a `name`, which is just an owned `String`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: String,
}

#[cfg(test)]
mod tests {
    //! # Model tests
    //!
    //! This module isn't compiled in a typical build, because of
    //! that `#[cfg(test)]` attribute.
    //! In a test build though, this module is included and the
    //! tests are run.
    //! Having tests inline with the functionality they test means
    //! we don't have to make anything public just so the test
    //! method can access it.

    use serde_json;
    use std::result::Result as StdResult;
    use super::*;

    #[test]
    fn valid_id() {
        let id = Id::try_from("an id").unwrap();

        assert_eq!("an id", id.as_ref())
    }

    #[test]
    fn invalid_id() {
        let id = Id::try_from("");

        assert!(id.is_err());
    }

    #[test]
    fn serialise_person() {
        let person = Person {
            id: Id::try_from("an id").unwrap(),
            name: "Some Name".to_string(),
        };

        let expected = json_str!({
            "id": "an id",
            "name": "Some Name"
        });

        let result = serde_json::to_string(&person).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn deserialise_person_valid() {
        let ser = json_str!({
            "id": "an id",
            "name": "Some Name"
        });

        let expected = Person {
            id: Id::try_from("an id").unwrap(),
            name: "Some Name".to_string(),
        };

        let result: Person = serde_json::from_str(&ser).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn deserialise_person_invalid() {
        let ser = json_str!({
            "id": "",
            "name": "Some Name"
        });

        let result: StdResult<Person, _> = serde_json::from_str(&ser);

        assert!(result.is_err());
    }
}
