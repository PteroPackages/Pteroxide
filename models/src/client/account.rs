use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub language: String,
    pub admin: bool,
}

impl Account {
    pub fn fullname(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // TODO: use md5
    // pub fn avatar(&self) -> String {}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiKey {
    pub identifier: String,
    pub description: String,
    pub allowed_ips: Vec<String>,
    pub created_at: String,
    pub last_used_at: Option<String>,
}
