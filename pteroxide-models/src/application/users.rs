use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use super::Server;
use crate::fractal::FractalList;
#[cfg(feature = "time")]
use crate::util::{self, Time};

/// Represents a user object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub id: i32,
    pub external_id: Option<String>,
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub language: String,
    pub root_admin: bool,
    #[serde(rename = "2fa")]
    pub two_factor: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
    #[cfg(feature = "app-relations")]
    #[serde(default)]
    #[serde(skip_serializing)]
    pub relationships: Option<UserRelations>,
}

#[cfg(feature = "time")]
impl User {
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

#[cfg(feature = "app-relations")]
#[derive(Deserialize)]
#[doc(hidden)]
struct RawUserRelations {
    pub servers: Option<FractalList<Server>>,
}

#[allow(clippy::from_over_into)]
#[cfg(feature = "app-relations")]
impl Into<UserRelations> for RawUserRelations {
    fn into(self) -> UserRelations {
        UserRelations {
            servers: match self.servers {
                Some(v) => Some(v.data.iter().map(|s| s.attributes.clone()).collect()),
                None => None,
            },
        }
    }
}

#[cfg(feature = "app-relations")]
#[doc(hidden)]
struct RelationVisitor;

#[cfg(feature = "app-relations")]
impl<'de> Visitor<'de> for RelationVisitor {
    type Value = UserRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of user relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawUserRelations::deserialize(des)?;

        Ok(rel.into())
    }
}

/// Represents the relationship objects for a user.
#[cfg(feature = "app-relations")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRelations {
    pub servers: Option<Vec<Server>>,
}

#[cfg(feature = "app-relations")]
impl<'de> Deserialize<'de> for UserRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationVisitor)
    }
}

/// Represents a server subuser object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubUser {
    pub id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}
