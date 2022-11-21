use std::{error, fmt::{Display, Formatter, Result as FmtResult}};

use pteroxide_models::FractalError;

/// Represents an interface for pteroxide-http errors, including errors received from the API.
#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn error::Error + Send + Sync>>,
    pub(super) kind: ErrorKind,
}

impl Error {
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn into_parts(&self) -> (&ErrorKind, Option<&Box<dyn error::Error + Send + Sync>>) {
        (&self.kind, self.source.as_ref())
    }

    pub fn into_source(&self) -> Option<&Box<dyn error::Error + Send + Sync>> {
        self.source.as_ref()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match &self.kind {
            ErrorKind::DeserializeError => f.write_str("Failed to deserialize body into model"),
            ErrorKind::RatelimitError => f.write_str("Received a ratelimit while processing request"),
            ErrorKind::RequestError => f.write_str("Request failed while processing"),
            ErrorKind::FractalError(e) => f.write_str(&format!("Received an error from the API ({})", e.errors[0].code)),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|s| &**s as &(dyn error::Error + 'static))
    }
}

impl From<FractalError> for Error {
    fn from(e: FractalError) -> Self {
        Self {
            kind: ErrorKind::FractalError(e.to_owned()),
            source: Some(Box::new(e.to_owned())),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self {
            kind: ErrorKind::RequestError,
            source: Some(Box::new(e)),
        }
    }
}

impl From<hyper::http::Error> for Error {
    fn from(e: hyper::http::Error) -> Self {
        Self {
            kind: ErrorKind::RequestError,
            source: Some(Box::new(e)),
        }
    }
}

/// The different kinds of errors that can be returned in pteroxide-http.
#[derive(Debug)]
pub enum ErrorKind {
    DeserializeError,
    RatelimitError,
    RequestError,
    FractalError(FractalError),
}
