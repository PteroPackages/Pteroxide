use serde::{Deserialize, Serialize};

#[cfg(feature = "app-relations")]
use super::NodeRelations;

/// Represents the inner API configuration of a [`NodeConfiguration`] object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct APIConfiguration {
    pub host: String,
    pub port: i32,
    pub ssl: SSLConfiguration,
    pub upload_limit: i32,
}

/// Represents a node object. This contains general information about the node such as the
/// location ID, Fully Qualified Domain Name (FQDN), memory/disk resources and allocated resources.
/// Most fields of a [`NodeConfiguration`] object can be derived from this object.
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
    pub memory: i32,
    pub memory_overallocate: i32,
    pub disk: i32,
    pub disk_overallocate: i32,
    pub daemon_base: String,
    pub daemon_sftp: i32,
    pub daemon_listen: i32,
    pub maintenance_mode: bool,
    pub upload_size: i32,
    pub allocated_resources: NodeResources,
    pub created_at: String,
    pub updated_at: Option<String>,
    #[cfg(feature = "app-relations")]
    #[serde(default)]
    #[serde(skip_serializing)]
    pub relationships: Option<NodeRelations>,
}

#[cfg(feature = "time")]
crate::impl_time!(Node);

/// Represents the configuration of a node in the panel. This is also used by the Wings API when
/// being deployed or restarted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeConfiguration {
    pub debug: bool,
    pub uuid: String,
    pub token_id: String,
    pub token: String,
    pub api: APIConfiguration,
    pub system: SystemConfiguration,
    pub allowed_mounts: Vec<String>,
    pub remote: String,
}

/// Represents the allocated resources of a [`Node`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NodeResources {
    pub memory: i32,
    pub disk: i32,
}

/// Represents the inner SFTP configuration of a [`NodeConfiguration`] object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SftpConfiguration {
    pub bind_port: i32,
}

/// Represents the inner SSL configuration of a [`NodeConfiguration`] object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SSLConfiguration {
    pub enabled: bool,
    pub cert: String,
    pub key: String,
}

/// Represents the inner system configuration of a [`NodeConfiguration`] object.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SystemConfiguration {
    pub data: String,
    pub sftp: SftpConfiguration,
}
