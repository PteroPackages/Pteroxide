use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{application::Server, fractal::FractalList};

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
