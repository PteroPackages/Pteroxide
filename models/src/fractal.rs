use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct FractalError {
    pub code: String,
    pub status: String,
    pub detail: String,
}

impl Display for FractalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "code: {}; status: {}; detail: {}", self.code, self.status, self.detail)
    }
}

impl std::error::Error for FractalError {}

impl FractalError {
    pub fn new(code: &str, status: &str, detail: &str) -> Self {
        Self {
            code: code.to_string(),
            status: status.to_string(),
            detail: detail.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FractalData<T> {
    pub object: String,
    pub attributes: T,
}

impl<T> Display for FractalData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "object: {}" , self.object)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FractalList<T> {
    pub object: String,
    pub data: Vec<FractalData<T>>,
}

impl<T> Display for FractalList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "object: {}; items: {}", self.object, self.data.len())
    }
}
