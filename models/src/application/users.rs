use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub id: u32,
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
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubUser {
    pub id: u32,
    pub user_id: u32,
    pub server_id: u32,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}
