use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    application::{Allocation, Egg, Location, Nest, Node, SubUser, User},
    fractal::{FractalItem, FractalList},
};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawServerRelations {
    allocations: Option<FractalList<Allocation>>,
    user: Option<FractalItem<User>>,
    subusers: Option<FractalList<SubUser>>,
    nest: Option<FractalItem<Nest>>,
    egg: Option<FractalItem<Egg>>,
    location: Option<FractalItem<Location>>,
    node: Option<FractalItem<Node>>,
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
            allocations: match rel.allocations {
                Some(v) => Some(v.data.iter().map(|a| a.attributes.clone()).collect()),
                None => None,
            },
            user: match rel.user {
                Some(u) => Some(u.attributes),
                None => None,
            },
            subusers: match rel.subusers {
                Some(v) => Some(v.data.iter().map(|u| u.attributes.clone()).collect()),
                None => None,
            },
            nest: match rel.nest {
                Some(n) => Some(n.attributes),
                None => None,
            },
            egg: match rel.egg {
                Some(e) => Some(e.attributes),
                None => None,
            },
            location: match rel.location {
                Some(l) => Some(l.attributes),
                None => None,
            },
            node: match rel.node {
                Some(n) => Some(n.attributes),
                None => None,
            },
        })
    }
}

/// Represents the relationship objects for a server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServerRelations {
    pub allocations: Option<Vec<Allocation>>,
    pub user: Option<User>,
    pub subusers: Option<Vec<SubUser>>,
    pub nest: Option<Nest>,
    pub egg: Option<Egg>,
    pub location: Option<Location>,
    pub node: Option<Node>,
}

impl<'de> Deserialize<'de> for ServerRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ServerRelationVisitor)
    }
}
