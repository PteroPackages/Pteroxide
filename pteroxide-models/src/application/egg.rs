use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "app-relations")]
use super::EggRelations;

/// Represents an egg (service) object, containing all the necessary information about the service
/// including Docker images, startup scripts, and parser configurations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Egg {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub nest: i32,
    pub author: String,
    pub description: Option<String>,
    pub docker_image: String,
    pub docker_images: HashMap<String, String>,
    pub config: EggConfig,
    pub startup: String,
    pub script: EggScript,
    pub created_at: String,
    pub updated_at: Option<String>,
    #[cfg(feature = "app-relations")]
    #[serde(default)]
    #[serde(skip_serializing)]
    pub relationships: Option<EggRelations>,
}

#[cfg(feature = "time")]
crate::impl_time!(Egg);

/// Represents the inner configuration of an [`Egg`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EggConfig {
    // FIXME: might not be able to implement this due to complexity
    // pub files: HashMap<String, EggConfigDescriptor>,
    pub startup: HashMap<String, String>,
    pub stop: Option<String>,
    pub logs: Vec<String>,
    pub file_denylist: Option<Vec<String>>,
    pub extends: Option<String>,
}

// #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
// pub struct EggConfigDescriptor {
//     pub parser: String,
//     pub find: HashMap<String, String>,
// }

/// Represents the script information of an [`Egg`] for Docker containers.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EggScript {
    pub privileged: bool,
    pub install: String,
    pub entry: String,
    pub container: String,
    pub extends: Option<String>,
}
