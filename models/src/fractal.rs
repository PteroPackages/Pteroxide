//! The wrapper objects that encapsule the inner value of a HTTP response from the Pterodactyl API
//! This can be either a response item [`FractalData`], a response list [`FractalList`], or a
//! response error [`FractalError`].

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// The error object (received on non-2xx HTTP responses). It implements the default
/// [`std::error::Error`] for compatibility.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

/// Wrapper object that holds one or more [`ErrorData`] object. It implements the default
/// [`std::error::Error`] for compatibility.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

/// The wrapper object for a single item from the Pterodactyl API.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FractalData<T> {
    pub object: String,
    pub attributes: T,
}

impl<T> Display for FractalData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "object: {}" , self.object)
    }
}

/// The wrapper object for an array of items from the Pterodactyl API. These items is often a list
/// of [`FractalData`] wrapped objects.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FractalList<T> {
    pub object: String,
    pub data: Vec<FractalData<T>>,
}

impl<T> Display for FractalList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "object: {}; items: {}", self.object, self.data.len())
    }
}
