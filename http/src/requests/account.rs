use crate::client::Client;
use crate::errors::Error;
use crate::requests::RequestBuilder;

use pteroxide_models::{
    client::account::{Account, ApiKey},
    fractal::{FractalData, FractalList},
};
use serde_json::json;

pub struct GetAccount<'a> {
    http: &'a Client,
}

impl<'a> GetAccount<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    pub async fn exec(self) -> Result<Account, Error> {
        match self.http.request::<FractalData<Account>>(
            RequestBuilder::new("/api/client/account")
        ).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}

pub struct GetApiKeys<'a> {
    http: &'a Client,
}

impl<'a> GetApiKeys<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    pub async fn exec(self) -> Result<Vec<ApiKey>, Error> {
        match self.http.request::<FractalList<ApiKey>>(
            RequestBuilder::new("/api/client/account/api-keys")
        ).await {
            Ok(v) => Ok(v.unwrap()
                .data
                .iter()
                .map(|k| k.attributes.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }
}

pub struct CreateApiKey<'a> {
    http: &'a Client,
    description: String,
    allowed_ips: Vec<String>,
}

impl<'a> CreateApiKey<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self {
            http,
            description: Default::default(),
            allowed_ips: Default::default(),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = description;

        self
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.allowed_ips.push(ip);

        self
    }

    pub fn ips(mut self, ips: Vec<String>) -> Self {
        self.allowed_ips = ips;

        self
    }

    pub async fn exec(self) -> Result<ApiKey, Error> {
        if self.description.is_empty() {
            return Err(Error::from("api key description is required"));
        }

        let mut req = RequestBuilder::new("/api/client/account/api-keys");
        req.method("POST")?;
        req.json(json!({
            "description": self.description,
            "allowed_ips": self.allowed_ips
        }));

        match self.http.request::<FractalData<ApiKey>>(req).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}
