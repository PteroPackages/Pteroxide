use serde::{Deserialize, Serialize};

use crate::global::{FeatureLimits, Limits};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SftpDetails {
    pub ip: String,
    pub port: u64,
}
