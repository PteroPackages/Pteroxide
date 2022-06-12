use hyper::{Body, header::HeaderValue, Method};
use serde_json::Value;

use crate::errors::Error;

/// The base builder for creating requests to Pterodactyl.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::request::Builder;
/// use serde_json::json;
/// 
/// let body = json!({
///     "short": "gb",
///     "long": "great britain"
/// });
/// 
/// let req = Builder::new("/api/application/locations")
///     .method("POST")?
///     .body(body);
/// 
/// // This can be used directly in the application or client instance's `request()` method without
/// // needing any extra `build()` or `compile()` methods.
/// ```
#[derive(Debug)]
pub struct Builder {
    pub method: Method,
    pub path: String,
    pub body: Body,
    pub ctype: HeaderValue,
}

impl Builder {
    pub fn new(path: &str) -> Self {
        Self {
            method: Method::GET,
            path: String::from(path),
            body: Body::empty(),
            ctype: HeaderValue::from_str("application/json").unwrap(),
        }
    }

    pub fn method(mut self, method: &'static str) -> Result<Self, Error> {
        match method {
            "GET" => self.method = Method::GET,
            "POST" => self.method = Method::POST,
            "PATCH" => self.method = Method::PATCH,
            "PUT" => self.method = Method::PUT,
            "DELETE" => self.method = Method::DELETE,
            _ => return Err(Error::from("invalid http method")),
        }

        Ok(self)
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = String::from(path);

        self
    }

    pub fn body(mut self, value: Value) -> Self {
        self.body = Body::from(value.to_string());

        self
    }

    pub fn content_type(mut self, value: &str) -> Self {
        self.ctype = HeaderValue::from_str(value).unwrap();

        self
    }
}
