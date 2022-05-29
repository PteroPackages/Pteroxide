use crate::client::Client;
use crate::errors::Error;
use crate::requests::RequestBuilder;

use pteroxide_models::{
    client::account::Account,
    fractal::FractalData,
};

pub struct GetAccount<'a> {
    http: &'a Client,
}

impl<'a> GetAccount<'a> {
    pub fn new(http: &'a Client) -> Self {
        Self { http }
    }

    pub async fn exec(&self) -> Result<Account, Error> {
        let req = RequestBuilder::new("/api/client/account");

        match self.http.request::<FractalData<Account>>(req).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(Error::from(e)),
        }
    }
}
