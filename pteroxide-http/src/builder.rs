use hyper::{header::HeaderValue, Body, Method};
use urlencoding::encode;

use super::routing::application;

/// Builder utility for creating HTTP requests, abstracting from the default HTTP request struct.
pub struct Builder<'a> {
    pub(crate) method: Method,
    pub(crate) route: String,
    pub(crate) params: Vec<(&'a str, &'a str)>,
    pub(crate) body: Body,
    pub(crate) content_type: HeaderValue,
    pub(crate) accept_type: HeaderValue,
}

impl<'a> Builder<'a> {
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;

        self
    }

    pub fn uri(&self, domain: String) -> String {
        let url = format!("{}{}", domain, self.route);

        if self.params.is_empty() {
            return url;
        }

        let mut query = format!("?{}={}", self.params[0].0, self.params[0].1);

        if self.params.len() > 1 {
            let parts : Vec<String> = self.params
                .iter()
                .skip(1)
                .map(|(k, v)| format!("&{}={}", k, encode(v)))
                .collect();

            query.extend(parts);
        }

        format!("{}/{}", url, query)
    }

    pub fn route(mut self, route: application::Route) -> Self {
        self.method = route.method();
        self.route = route.to_string();

        self
    }

    pub fn param(mut self, key: &'a str, value: &'a str) -> Self {
        self.params.push((key, value));

        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where
        Body: From<T>,
    {
        self.body = Body::from(body);

        self
    }

    pub fn content_type(mut self, value: &str) -> Self {
        self.content_type = HeaderValue::from_str(value).unwrap();

        self
    }

    pub fn accept_type(mut self, value: &str) -> Self {
        self.accept_type = HeaderValue::from_str(value).unwrap();

        self
    }
}

impl Default for Builder<'_> {
    fn default() -> Self {
        Self {
            method: Default::default(),
            route: Default::default(),
            params: Default::default(),
            body: Body::empty(),
            content_type: HeaderValue::from_str("application/json").unwrap(),
            accept_type: HeaderValue::from_str("application/json").unwrap(),
        }
    }
}
