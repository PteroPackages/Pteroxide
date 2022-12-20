use serde::{Deserialize, Serialize};

#[cfg(feature = "time")]
use crate::util::{self, Time};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location_id: i32,
    pub public: bool,
    pub fqdn: String,
    pub scheme: String,
    pub behind_proxy: bool,
    pub memory: i64,
    pub memory_overallocate: i64,
    pub disk: i64,
    pub disk_overallocate: i64,
    pub daemon_base: String,
    pub daemon_sftp: i64,
    pub daemon_listen: i64,
    pub maintenance_mode: bool,
    pub upload_size: i64,
    pub allocated_resources: NodeResources,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeResources {
    pub memory: i64,
    pub disk: i64,
}

#[cfg(feature = "time")]
impl Node {
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