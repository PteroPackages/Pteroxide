use hyper::{header::HeaderValue, Body, Method};
use serde::Serialize;
use serde_json::json;
use urlencoding::encode;

use super::routing::Route;

/// Builder utility for creating HTTP requests, abstracting from the default HTTP request struct.
pub struct Builder {
    pub(crate) method: Method,
    pub(crate) route: String,
    pub(crate) params: Vec<(String, String)>,
    pub(crate) include: Vec<String>,
    pub(crate) body: Body,
    pub(crate) content_type: HeaderValue,
    pub(crate) accept_type: HeaderValue,
}

impl Builder {
    /// Sets the HTTP [`Method`] for the request and returns the builder. Defaults to [`GET`].
    ///
    /// [`GET`]: Method::GET
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;

        self
    }

    /// Builds a URI from the route and params set in the builder.
    pub fn uri(&mut self, domain: String) -> String {
        let url = format!("{}{}", domain, self.route);

        if !self.include.is_empty() {
            self.params
                .push(("include".to_string(), self.include.join(",")));
        }

        if self.params.is_empty() {
            return url;
        }

        let mut query = format!("?{}={}", self.params[0].0, self.params[0].1);

        if self.params.len() > 1 {
            let parts: Vec<String> = self
                .params
                .iter()
                .skip(1)
                .map(|(k, v)| format!("&{}={}", k, encode(v)))
                .collect();

            query.extend(parts);
        }

        format!("{}/{}", url, query)
    }

    /// Sets the HTTP [`Route`] for the request and returns the builder. This also sets the default
    /// request method from the route.
    ///
    /// [`Route`]: application::Route
    pub fn route(mut self, route: Route) -> Self {
        self.method = route.method();
        self.route = route.to_string();

        self
    }

    /// Sets a HTTP query parameter to include in the URI and returns the builder.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let builder = Builder::default()
    ///     .route(Route::GetUsers)
    ///     .param("include", "servers");
    ///
    /// println!("{}", builder.uri()); // "/api/application/users?include=servers"
    /// ```
    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.params.push((key.to_string(), value.to_string()));

        self
    }

    pub fn include(mut self, value: &str) -> Self {
        self.include.push(value.to_string());

        self
    }

    /// Sets the request [`Body`] to the given value and returns the builder. Defaults to empty.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let value = json!({
    ///     "username": "test",
    ///     "email": "test@example.com",
    ///     "first_name": "test",
    ///     "last_name": "example"
    /// });
    ///
    /// let builder = Builder::default()
    ///     .route(Route::CreateUser)
    ///     .body(value.to_string());
    /// ```
    pub fn body<T>(mut self, body: T) -> Self
    where
        Body: From<T>,
    {
        self.body = Body::from(body);

        self
    }

    /// Sets the request [`Body`] to the JSON representation of the value and returns the builder.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let fields = CreateUserFields {
    ///     username: "test",
    ///     email: "test@example.com",
    ///     first_name: "test",
    ///     last_name: "example",
    /// }
    ///
    /// let builder = Builder::default()
    ///     .route(Route::CreateUser)
    ///     .json(fields);
    /// ```
    pub fn json<T>(mut self, body: T) -> Self
    where
        T: Serialize,
    {
        let value = json!(body);
        self.body = Body::from(value.to_string());

        self
    }

    /// Sets the HTTP [`Content-Type`] header for the request and returns the builder. Defaults to
    /// `application/json`.
    ///
    /// [`Content-Type`]: hyper::http::header::CONTENT_TYPE
    pub fn content_type(mut self, value: &str) -> Self {
        self.content_type = HeaderValue::from_str(value).unwrap();

        self
    }

    /// Sets the HTTP [`Accept`] header for the request and returns the builder. Defaults to
    /// `application/json`.
    ///
    /// [`Accept`]: hyper::http::header::ACCEPT
    pub fn accept_type(mut self, value: &str) -> Self {
        self.accept_type = HeaderValue::from_str(value).unwrap();

        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            method: Default::default(),
            route: Default::default(),
            params: Default::default(),
            include: Default::default(),
            body: Default::default(),
            content_type: HeaderValue::from_str("application/json").unwrap(),
            accept_type: HeaderValue::from_str("application/json").unwrap(),
        }
    }
}
