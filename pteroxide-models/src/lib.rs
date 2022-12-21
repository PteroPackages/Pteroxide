use serde::{Deserialize, Serialize};

#[cfg(feature = "app")]
pub mod application;
#[cfg(feature = "fractal")]
pub mod fractal;
#[cfg(feature = "time")]
pub mod util;

/// Represents the feature limits of a server.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeatureLimits {
    pub allocations: i32,
    pub backups: i32,
    pub databases: i32,
}

/// Represents the limits of a server.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Limits {
    pub memory: i32,
    pub swap: i32,
    pub disk: i32,
    pub io: Option<i32>,
    pub cpu: i32,
    pub threads: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_disabled: Option<bool>,
}
