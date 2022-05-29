use std::error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum ErrorKind {
    RequestError,
    FractalError,
}

#[derive(Debug)]
pub struct Error {
    pub(super) source: Option<Box<dyn error::Error + Send + Sync>>,
    pub(super) kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ErrorKind::RequestError => f.write_str("failed to perform request"),
            ErrorKind::FractalError => f.write_str("recieved an api error"),
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

impl Error {
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
