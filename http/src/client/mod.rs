use bytes::Buf;
use hyper::{
    Body,
    client::{Client as HttpClient, HttpConnector},
    Request,
    Uri,
    StatusCode,
};
use pteroxide_models::fractal::FractalError;
use serde::de::Deserialize;

use crate::errors::Error;
use crate::requests::RequestBuilder;
use crate::requests::account::GetAccount;

#[derive(Debug)]
pub struct Client {
    pub url: String,
    pub key: String,
    pub http: HttpClient<HttpConnector>,
}

impl Client {
    pub fn new(url: String, key: String) -> Self {
        Self {
            url,
            key,
            http: HttpClient::new(),
        }
    }

    pub async fn request<T>(&self, builder: RequestBuilder) -> Result<Option<T>, Error>
    where for<'de> T: Deserialize<'de>,
    {
        let uri = format!("{}{}", self.url, builder.path).parse::<Uri>().unwrap();

        let req = Request::builder()
            .method(builder.method)
            .uri(uri)
            .header("User-Agent", "Pteroxide Client")
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/json,text/plain")
            .body(Body::from(builder.body))
            .unwrap();

        match self.http.request(req).await {
            Ok(v) => match v.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    let buf = hyper::body::aggregate(v).await?;
                    let data = serde_json::from_reader::<_, T>(buf.reader()).unwrap();
                    Ok(Some(data))
                },
                StatusCode::ACCEPTED | StatusCode::NO_CONTENT => Ok(None),
                StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN |
                StatusCode::NOT_FOUND | StatusCode::METHOD_NOT_ALLOWED | StatusCode::CONFLICT |
                StatusCode::UNPROCESSABLE_ENTITY | StatusCode::TOO_MANY_REQUESTS => {
                    let buf = hyper::body::aggregate(v).await?;
                    let data = serde_json::from_reader::<_, FractalError>(buf.reader()).unwrap();
                    Err(Error::from(data))
                },
                // indeterminable
                _ => Err(Error::default())
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn get_account(&self) -> GetAccount {
        GetAccount::new(self)
    }
}
