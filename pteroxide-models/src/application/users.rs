use serde::{Deserialize, Serialize};
#[cfg(feature = "time")]
use time::Time;

use super::Server;
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UserRelations {
    pub servers: Option<Vec<Server>>,
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
