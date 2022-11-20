use serde::{Deserialize, Serialize};

pub mod application;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeatureLimits {
    allocations: u32,
    backups: u32,
    databases: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FractalItem<T> {
    pub object: String,
    pub attributes: T,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FractalList<T> {
    pub object: String,
    pub data: FractalItem<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Limits {
    pub memory: u64,
    pub swap: u64,
    pub disk: u64,
    pub io: Option<String>,
    pub cpu: u64,
    pub threads: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_disabled: Option<bool>,
}
