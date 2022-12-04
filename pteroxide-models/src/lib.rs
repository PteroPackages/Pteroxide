use serde::{Deserialize, Serialize};

#[cfg(feature = "app")]
pub mod application;
#[cfg(feature = "fractal")]
pub mod fractal;
#[cfg(any(feature = "time", feature = "util"))]
pub mod util;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeatureLimits {
    allocations: i32,
    backups: i32,
    databases: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Limits {
    pub memory: i64,
    pub swap: i64,
    pub disk: i64,
    pub io: Option<i64>,
    pub cpu: i64,
    pub threads: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_disabled: Option<bool>,
}
