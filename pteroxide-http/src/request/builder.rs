use hyper::{Body, header::HeaderValue, Method};

use super::routing::application;

pub struct Builder {
    pub(crate) method: Method,
    pub(crate) route: String,
    pub(crate) body: Body,
    pub(crate) content_type: HeaderValue,
}

impl Builder {
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;

        self
    }

    pub fn route(mut self, route: application::Route) -> Self {
        self.route = route.to_string();

        self
    }

    pub fn body<T>(mut self, body: T) -> Self
    where Body: From<T>
    {
        self.body = Body::from(body);

        self
    }

    pub fn content_type(mut self, value: &str) -> Self {
        self.content_type = HeaderValue::from_str(value).unwrap();

        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            method: Method::GET,
            route: Default::default(),
            body: Body::empty(),
            content_type: HeaderValue::from_str("application/json").unwrap(),
        }
    }
}
