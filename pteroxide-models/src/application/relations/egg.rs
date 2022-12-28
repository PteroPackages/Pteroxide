use serde::{
    de::{value::MapAccessDeserializer, MapAccess, Visitor},
    Deserialize,
};
use std::fmt::{Formatter, Result as FmtResult};

use crate::{
    application::{EggConfig, EggScript, Nest, Server},
    fractal::{FractalItem, FractalList},
};

#[derive(Deserialize)]
#[doc(hidden)]
struct RawEggRelations {
    config: Option<FractalItem<EggConfig>>,
    nest: Option<FractalItem<Nest>>,
    script: Option<FractalItem<EggScript>>,
    servers: Option<FractalList<Server>>,
    // variables
}

#[doc(hidden)]
struct RelationsVisitor;

impl<'de> Visitor<'de> for RelationsVisitor {
    type Value = EggRelations;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a map of egg relationships")
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let des = MapAccessDeserializer::new(map);
        let rel = RawEggRelations::deserialize(des)?;

        Ok(EggRelations {
            config: match rel.config {
                Some(c) => Some(c.attributes),
                None => None,
            },
            nest: match rel.nest {
                Some(n) => Some(n.attributes),
                None => None,
            },
            script: match rel.script {
                Some(s) => Some(s.attributes),
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
pub struct EggRelations {
    pub config: Option<EggConfig>,
    pub nest: Option<Nest>,
    pub script: Option<EggScript>,
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for EggRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RelationsVisitor)
    }
}
