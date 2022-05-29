use crate::client::Client;
use crate::errors::Error;
use crate::requests::RequestBuilder;

use pteroxide_models::{
    client::account::{Account, ApiKey},
    fractal::{FractalData, FractalList},
};

pub struct GetAccount<'a> {
    http: &'a Client,
    keys: bool,
}

impl<'a> GetAccount<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { http, keys: false }
    }

    pub fn api_keys(mut self, value: bool) -> Self {
        self.keys = value;

        self
    }

    pub async fn exec(self) -> Result<Account, Error> {
        let res = match self.http.request::<FractalData<Account>>(
            RequestBuilder::new("/api/client/account")
        ).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        };

        if res.is_ok() {
            let mut model = res.unwrap();
            if self.keys {
                match self.http.request::<FractalList<ApiKey>>(
                    RequestBuilder::new("/api/client/account/api-keys")
                ).await {
                    Ok(v) => {
                        model.api_keys = Some(v.unwrap()
                            .data
                            .iter()
                            .map(|k| k.attributes.clone())
                            .collect());
                    }
                    Err(e) => return Err(e),
                };
            }

            return Ok(model)
        }

        res
    }
}
