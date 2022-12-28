use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    application::{Node, Server},
    fractal::FractalList,
};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawLocationRelations {
    nodes: Option<FractalList<Node>>,
    servers: Option<FractalList<Server>>,
}

#[doc(hidden)]
struct RelationsVisitor;

impl<'de> Visitor<'de> for RelationsVisitor {
    type Value = LocationRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of location relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawLocationRelations::deserialize(des)?;

        Ok(LocationRelations {
            nodes: match rel.nodes {
                Some(v) => Some(v.data.iter().map(|n| n.attributes.clone()).collect()),
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
pub struct LocationRelations {
    pub nodes: Option<Vec<Node>>,
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for LocationRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationsVisitor)
    }
}
