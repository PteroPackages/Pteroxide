use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    application::{Allocation, Location, Server},
    fractal::{FractalItem, FractalList},
};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawNodeRelations {
    allocations: Option<FractalList<Allocation>>,
    location: Option<FractalItem<Location>>,
    servers: Option<FractalList<Server>>,
}

#[doc(hidden)]
struct NodeRelationsVisitor;

impl<'de> Visitor<'de> for NodeRelationsVisitor {
    type Value = NodeRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of node relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawNodeRelations::deserialize(des)?;

        Ok(NodeRelations {
            allocations: match rel.allocations {
                Some(v) => Some(v.data.iter().map(|a| a.attributes.clone()).collect()),
                None => None,
            },
            location: match rel.location {
                Some(l) => Some(l.attributes),
                None => None,
            },
            servers: match rel.servers {
                Some(v) => Some(v.data.iter().map(|a| a.attributes.clone()).collect()),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeRelations {
    pub allocations: Option<Vec<Allocation>>,
    pub location: Option<Location>,
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for NodeRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(NodeRelationsVisitor)
    }
}
