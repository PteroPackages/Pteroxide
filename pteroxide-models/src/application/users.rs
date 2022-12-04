use serde::{
    de::{Error, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};
#[cfg(feature = "time")]
use time::Time;

use super::Server;
use crate::fractal::FractalList;
#[cfg(feature = "time")]
use crate::time as util;

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
    pub fn parse_created_at(&self) -> Time {
        util::parse(self.created_at.clone())
    }

    pub fn try_parse_created_at(&self) -> Option<Time> {
        match util::try_parse(self.created_at.clone()) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRelations {
    pub servers: Option<Vec<Server>>,
}

impl<'de> Deserialize<'de> for UserRelations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawUserRelations {
            servers: Option<FractalList<Server>>,
        }

        struct RelationVisitor;

        impl<'de> Visitor<'de> for RelationVisitor {
            type Value = RawUserRelations;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("struct RawUserRelations")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some(key) = map.next_key::<&str>()? {
                    if key == "servers" {
                        return map.next_value();
                    }
                }

                Err(Error::missing_field("servers"))
            }
        }

        let res =
            deserializer.deserialize_struct("RawUserRelations", &["servers"], RelationVisitor)?;

        Ok(Self {
            servers: match res.servers {
                Some(v) => Some(v.data.iter().map(|s| s.attributes.clone()).collect()),
                None => None,
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubUser {
    pub id: i32,
    pub user_id: i32,
    pub server_id: i32,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}
