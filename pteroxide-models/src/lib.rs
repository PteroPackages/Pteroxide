use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[cfg(feature = "app")]
pub mod application;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FeatureLimits {
    allocations: u32,
    backups: u32,
    databases: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ErrorData {
    pub code: String,
    pub status: String,
    pub detail: String,
}

impl Display for ErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "code: {}; status: {}; detail: {}",
            self.code, self.status, self.detail
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FractalError {
    pub errors: Vec<ErrorData>,
}

impl Display for FractalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for e in self.errors.iter() {
            match Display::fmt(&e, f) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

impl Error for FractalError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FractalItem<T> {
    pub object: String,
    pub attributes: T,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FractalList<T> {
    pub object: String,
    pub data: Vec<FractalItem<T>>,
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
