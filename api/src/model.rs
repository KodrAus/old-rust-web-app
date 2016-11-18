use std::convert::{TryInto, TryFrom};
use std::result::Result as StdResult;
use serde::{de, Deserialize, Deserializer};
use errors::*;

#[derive(Serialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: Id,
    pub name: String,
}
