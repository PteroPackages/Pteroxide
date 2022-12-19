use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use super::{Server, SubUser, User};
use crate::fractal::{FractalItem, FractalList};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawUserRelations {
    pub servers: Option<FractalList<Server>>,
}

#[doc(hidden)]
struct UserRelationVisitor;

impl<'de> Visitor<'de> for UserRelationVisitor {
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

        Ok(UserRelations {
            servers: match rel.servers {
                Some(v) => Some(v.data.iter().map(|s| s.attributes.clone()).collect()),
                None => None,
            },
        })
    }
}

/// Represents the relationship objects for a user.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRelations {
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for UserRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(UserRelationVisitor)
    }
}

#[derive(Deserialize)]
#[doc(hidden)]
struct RawServerRelations {
    user: Option<FractalItem<User>>,
    subusers: Option<FractalList<SubUser>>,
}

#[doc(hidden)]
struct ServerRelationVisitor;

impl<'de> Visitor<'de> for ServerRelationVisitor {
    type Value = ServerRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of server relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawServerRelations::deserialize(des)?;

        Ok(ServerRelations {
            user: match rel.user {
                Some(u) => Some(u.attributes),
                None => None,
            },
            subusers: match rel.subusers {
                Some(v) => Some(v.data.iter().map(|u| u.attributes.clone()).collect()),
                None => None,
            },
        })
    }
}

/// Represents the relationship objects for a server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerRelations {
    pub user: Option<User>,
    pub subusers: Option<Vec<SubUser>>,
}

impl<'de> Deserialize<'de> for ServerRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ServerRelationVisitor)
    }
}
