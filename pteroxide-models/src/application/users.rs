use serde::{Deserialize, Serialize};

use super::Server;

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
