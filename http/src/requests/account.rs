use crate::client::Client;
use crate::errors::Error;
use crate::requests::RequestBuilder;

use pteroxide_models::{
    client::account::{Account, ApiKey},
    fractal::{FractalData, FractalList},
};

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
