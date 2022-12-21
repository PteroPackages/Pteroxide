use serde::{Deserialize, Serialize};

#[cfg(feature = "time")]
use crate::util::{self, Time};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Location {
    pub id: i32,
    pub short: String,
    pub long: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[cfg(feature = "time")]
impl Location {
    /// Parses the string created at time string into a [`Time`] object.
    pub fn parse_created_at(&self) -> Time {
        util::parse(self.created_at.clone())
    }

    /// Attempts to parse the created at time string into a [`Time`] object, returning an
    /// option.
    pub fn try_parse_created_at(&self) -> Option<Time> {
        match util::try_parse(self.created_at.clone()) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    /// Parses the updated at time string into a [`Time`] object, returning an option if the field
    /// has a value.
    pub fn parse_updated_at(&self) -> Option<Time> {
        match &self.updated_at {
            Some(s) => match util::try_parse(s.clone()) {
                Ok(t) => Some(t),
                Err(_) => None,
            },
            None => None,
        }
    }
}
