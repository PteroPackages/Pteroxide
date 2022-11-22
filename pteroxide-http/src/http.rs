use hyper::{
    body::{self, Buf},
    Client as HClient,
    client::HttpConnector, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Request, StatusCode,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use crate::{error::Error, request::builder::Builder};
use pteroxide_models::FractalError;

/// The HTTP client for interacting with the application and client API.
#[derive(Debug)]
pub struct Http {
    client: HClient<HttpsConnector<HttpConnector>>,
    url: String,
    key: String,
    // default_headers
}

impl Http {
    pub fn new(url: String, mut key: String) -> Self {
        let conn = HttpsConnector::new();
        key.insert_str(0, "Bearer ");

        Self {
            client: HClient::builder().build(conn),
            url,
            key,
        }
    }

    pub async fn request<T>(self, builder: Builder) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>
    {
        let uri = format!("{}{}", self.url, builder.route);
        let req = Request::builder()
            .uri(uri)
            .method(builder.method)
            .header(USER_AGENT, "Pteroxide HTTP Client")
            .header(AUTHORIZATION, self.key)
            .header(CONTENT_TYPE, builder.content_type)
            .header(ACCEPT, builder.accept_type)
            .body(builder.body)?;

        let res = self.client.request(req).await?;
        match res.status() {
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::ACCEPTED
            | StatusCode::NO_CONTENT => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader(buf.reader())
                    .expect("failed to deserialize into model");

                Ok(data)
            },
            _ => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader::<_, FractalError>(buf.reader())
                    .expect("failed to deserialize into model");

                Err(Error::from(data))
            },
        }
    }
}
