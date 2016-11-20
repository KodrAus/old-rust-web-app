//! # App model
//! 
//! This is the implementation of our simple application model.
//! 

use std::convert::{TryInto, TryFrom};
use std::result::Result as StdResult;
use serde::{de, Deserialize, Deserializer};
use errors::*;

#[derive(Debug, PartialEq, Serialize)]
pub struct Id(String);

// We use the `TryFrom` trait to maybe convert a string
// into an `Id`. If the string is empty, then we return
// a failure instead of an id value.
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
                value.try_into().map_err(|e| {
                    E::custom(format!("Failed to parse id: {}", e))
                })
            }
        }

        deserializer.deserialize_str(IdVisitor)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::result::Result as StdResult;
    use std::convert::TryFrom;
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
            name: "Some Name".to_string()
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
            name: "Some Name".to_string()
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