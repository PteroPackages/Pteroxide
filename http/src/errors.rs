use pteroxide_models::fractal::FractalError;
use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum ErrorKind {
    FieldError,
    FractalError,
    RequestError,
    UnknownError,
}

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn error::Error + Send + Sync>>,
    pub(super) kind: ErrorKind,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            kind: ErrorKind::UnknownError,
            source: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ErrorKind::FieldError => f.write_str("recieved a validation field error"),
            ErrorKind::FractalError => f.write_str("recieved an api error"),
            ErrorKind::RequestError => f.write_str("failed to perform request"),
            ErrorKind::UnknownError => f.write_str("unknown error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|s| &**s as &(dyn error::Error + 'static))
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Self {
            kind: ErrorKind::RequestError,
            source: Some(Box::new(error)),
        }
    }
}

impl From<FractalError> for Error {
    fn from(error: FractalError) -> Self {
        Self {
            kind: ErrorKind::FractalError,
            source: Some(Box::new(error)),
        }
    }
}

impl From<&str> for Error {
    fn from(_: &str) -> Self {
        Self {
            kind: ErrorKind::FieldError,
            source: None,
        }
    }
}

impl Error {
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
