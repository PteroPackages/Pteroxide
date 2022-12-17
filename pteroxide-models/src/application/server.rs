use serde::{Deserialize, Serialize};

#[cfg(feature = "app-relations")]
use super::relations::ServerRelations;
#[cfg(feature = "time")]
use crate::util::{self, Time};
use crate::{FeatureLimits, Limits};

/// Represents the container details for the server, such as the startup command and docker image.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Container {
    pub startup_command: String,
    pub image: String,
    pub installed: i8,
    // TODO: handle this, maybe ContainerEnv?
    // pub environment
}

/// Represents a server object. This contains general information about the server such as the
/// UUID, (feature) limits, and the status in the panel. Note that this is NOT a representation of
/// the server/container on Wings, so it does not contain information like the current power state
/// or resource usage.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Server {
    pub id: i32,
    pub external_id: Option<String>,
    pub uuid: String,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub suspended: bool,
    pub limits: Limits,
    pub feature_limits: FeatureLimits,
    pub user: i32,
    pub node: i32,
    pub allocation: i32,
    pub nest: i32,
    pub egg: i32,
    pub container: Container,
    pub created_at: String,
    pub updated_at: Option<String>,
    #[cfg(feature = "app-relations")]
    #[serde(default)]
    #[serde(skip_serializing)]
    pub relationships: Option<ServerRelations>,
}

#[cfg(feature = "time")]
impl Server {
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

/// Represents the status of a [`Server`] in the panel.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    Installing,
    InstallFailed,
    Suspended,
    RestoringBackup,
    Unknown(String),
}

impl<'a> From<&'a str> for Status {
    fn from(value: &'a str) -> Self {
        match value {
            "installing" => Status::Installing,
            "install_failed" => Status::InstallFailed,
            "suspended" => Status::Suspended,
            "restoring_backup" => Status::RestoringBackup,
            v => Status::Unknown(String::from(v)),
        }
    }
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Installing => String::from("installing"),
            Status::InstallFailed => String::from("install_failed"),
            Status::Suspended => String::from("suspended"),
            Status::RestoringBackup => String::from("restoring_backup"),
            Status::Unknown(v) => String::from(v),
        }
    }
}
