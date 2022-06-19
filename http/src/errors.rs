use pteroxide_models::fractal::FractalError;
use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// The kind of error that is being returned. This can vary depending on where in the module the
/// error originated from.
#[derive(Debug)]
pub enum ErrorKind {
    FieldError { msg: &'static str },
    FractalError,
    RequestError,
    UnknownError,
}

/// Represents an error in the module. Because this can have different origins, [`ErrorKind`] is
/// implemented to identify the kinds. It implements the default [`std::error::Error`] for
/// compatibility.
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
            ErrorKind::FieldError { msg } => {
                f.write_str("validation requirement failed: ")?;
                f.write_str(msg)
            }
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

impl From<FractalError> for Error {
    fn from(error: FractalError) -> Self {
        Self {
            kind: ErrorKind::FractalError,
            source: Some(Box::new(error)),
        }
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

impl From<hyper::http::Error> for Error {
    fn from(error: hyper::http::Error) -> Self {
        Self {
            kind: ErrorKind::RequestError,
            source: Some(Box::new(error)),
        }
    }
}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(error: hyper::http::uri::InvalidUri) -> Self {
        Self {
            kind: ErrorKind::RequestError,
            source: Some(Box::new(error)),
        }
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Self {
            kind: ErrorKind::FieldError { msg },
            source: None,
        }
    }
}

impl Error {
    /// Returns the [`ErrorKind`] of the error.
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
