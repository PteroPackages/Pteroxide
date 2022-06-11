use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HostDetails {
    pub address: String,
    pub port: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub username: String,
    pub host: HostDetails,
    pub connections_from: String,
    pub max_connections: i64,
}
