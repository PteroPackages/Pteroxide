use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorData {
    pub code: String,
    pub status: String,
    pub detail: String,
}

impl Display for ErrorData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "code: {}; status: {}; detail: {}", self.code, self.status, self.detail)
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

impl std::error::Error for ErrorData {}
impl std::error::Error for FractalError {}

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
