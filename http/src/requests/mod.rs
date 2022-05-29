use hyper::Method;

pub mod account;

#[derive(Debug)]
pub struct RequestBuilder {
    pub method: Method,
    pub path: String,
    pub body: Vec<u8>,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: Default::default(),
            body: Default::default(),
        }
    }
}

impl RequestBuilder {
    pub fn new(path: &str) -> Self {
        let mut r = Self::default();
        r.path = String::from(path);
        r
    }
}
