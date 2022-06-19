//! Global objects used in both the Application API and Client API.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FeatureLimits {
    pub allocations: i32,
    pub backups: i32,
    pub databases: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Limits {
    pub memory: i64,
    pub disk: i64,
    pub swap: i64,
    pub io: i64,
    pub cpu: i64,
    pub threads: Option<String>,
    pub oom_disabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UrlData {
    pub url: String,
}
