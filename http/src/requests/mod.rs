use hyper::{Body, Method};
use serde_json::Value;

use crate::errors::Error;

pub mod account;

#[derive(Debug)]
pub struct RequestBuilder {
    pub method: Method,
    pub path: String,
    pub body: Body,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            method: Default::default(),
            path: Default::default(),
            body: Default::default(),
        }
    }
}

impl RequestBuilder {
    pub fn new(path: &str) -> Self {
        let mut b = Self::default();
        b.path = String::from(path);

        b
    }

    pub fn json(&mut self, data: Value) {
        let v = data.as_str().unwrap();
        self.body = Body::from(v.to_owned());
    }

    pub fn text(&mut self, data: String) {
        self.body = Body::from(data.as_bytes().to_owned());
    }

    pub fn method(&mut self, method: &str) -> Result<(), Error> {
        match Method::try_from(method) {
            Ok(m) => Ok(self.method = m),
            Err(_) => Err(Error::from("invalid http method")),
        }
    }
}
