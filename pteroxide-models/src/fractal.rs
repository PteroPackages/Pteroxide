use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

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
    // pub meta: FractalMeta,
}

// #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
// pub struct FractalMeta {
//     pub count: i32,
//     pub total: i32,
//     pub current_page: i32,
//     pub per_page: i32,
//     pub total_pages: i32,
// }
