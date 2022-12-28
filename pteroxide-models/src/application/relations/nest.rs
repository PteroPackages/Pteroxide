use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    application::{Egg, Server},
    fractal::FractalList,
};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawNestRelations {
    eggs: Option<FractalList<Egg>>,
    servers: Option<FractalList<Server>>,
}

#[doc(hidden)]
struct RelationsVisitor;

impl<'de> Visitor<'de> for RelationsVisitor {
    type Value = NestRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of nest relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawNestRelations::deserialize(des)?;

        Ok(NestRelations {
            eggs: match rel.eggs {
                Some(v) => Some(v.data.iter().map(|e| e.attributes.clone()).collect()),
                None => None,
            },
            servers: match rel.servers {
                Some(v) => Some(v.data.iter().map(|s| s.attributes.clone()).collect()),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NestRelations {
    pub eggs: Option<Vec<Egg>>,
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for NestRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationsVisitor)
    }
}
