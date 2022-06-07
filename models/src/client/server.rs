use serde::{Deserialize, Serialize};

use crate::global::{FeatureLimits, Limits};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resources {
    pub memory_bytes: i64,
    pub cpu_absolute: i64,
    pub disk_bytes: i64,
    pub network_rx_bytes: i64,
    pub network_tx_bytes: i64,
    pub uptime: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub server_owner: bool,
    pub identifier: String,
    pub uuid: String,
    pub internal_id: u32,
    pub name: String,
    pub description: String,
    pub node: String,
    pub sftp_details: SftpDetails,
    pub limits: Limits,
    pub feature_limits: FeatureLimits,
    pub invocation: Option<String>,
    pub docker_image: String,
    pub egg_features: Option<Vec<String>>,
    pub status: Option<String>,
    pub is_suspended: bool,
    pub is_installing: bool,
    pub is_transferring: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ServerState {
    OFFLINE,
    RUNNING,
    STARTING,
    STOPPING,
    STOPPED,
    UNKNOWN,
}

// doesn't work...
impl Serialize for ServerState {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        match *self {
            Self::OFFLINE => ser.serialize_unit_variant("Resources", 0, "OFFLINE"),
            Self::RUNNING => ser.serialize_unit_variant("Resources", 1, "RUNNING"),
            Self::STARTING => ser.serialize_unit_variant("Resources", 2, "STARTING"),
            Self::STOPPING => ser.serialize_unit_variant("Resources", 3, "STOPPING"),
            Self::STOPPED => ser.serialize_unit_variant("Resources", 4, "STOPPED"),
            _ => ser.serialize_unit_variant("Resources", 5, "UNKNOWN"),
        }
    }
}

impl From<&str> for ServerState {
    fn from(s: &str) -> Self {
        match s {
            "offline" => Self::OFFLINE,
            "running" => Self::RUNNING,
            "starting" => Self::STARTING,
            "stopping" => Self::STOPPING,
            "stopped" => Self::STOPPED,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerStatistics {
    pub current_state: String,
    pub is_suspended: bool,
    pub resources: Resources,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SftpDetails {
    pub ip: String,
    pub port: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebSocketAuth {
    pub socket: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct WebSocketWrapper {
    pub data: WebSocketAuth,
}
