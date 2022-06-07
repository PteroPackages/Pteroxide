//! Request builder implementations for the module.

use hyper::{Body, Method};
use serde_json::Value;

use crate::errors::Error;

/// The base builder for creating requests to Pterodactyl.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::requests::RequestBuilder;
/// use serde_json::json;
/// 
/// let body = json!({
///     "short": "gb",
///     "long": "great britain"
/// });
/// 
/// let req = RequestBuilder::new("/api/application/locations")
///     .method("POST")?
///     .json(body);
/// 
/// // This can be used directly in the application or client instance's `request()` method without
/// // needing any extra `build()` or `compile()` methods.
/// ```
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
    /// Creates a new request builder.
    pub fn new(path: &str) -> Self {
        let mut b = Self::default();
        b.path = String::from(path);

        b
    }

    /// Sets the request body's [`Value`] to a JSON object.
    pub fn json(&mut self, data: Value) {
        self.body = Body::from(data.to_string());
    }

    /// Sets the request body's [`Value`] to a raw string.
    pub fn text(&mut self, data: String) {
        self.body = Body::from(data.as_bytes().to_owned());
    }

    /// Sets the request [`Method`].
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`FieldError`] if the method is invalid.
    /// 
    /// [`FieldError`]: crate::errors::ErrorKind::FieldError
    pub fn method(&mut self, method: &str) -> Result<(), Error> {
        match Method::try_from(method) {
            Ok(m) => Ok(self.method = m),
            Err(_) => Err(Error::from("invalid http method")),
        }
    }
}
