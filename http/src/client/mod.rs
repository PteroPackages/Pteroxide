use bytes::Buf;
use hyper::{client::Client as HttpClient, Body, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::de::Deserialize;

use crate::{
    errors::Error,
    requests::{
        account::{GetAccount, GetApiKeys},
        RequestBuilder
    },
};

#[derive(Debug)]
pub struct Client {
    pub url: String,
    pub key: String,
    pub http: HttpClient<HttpsConnector<hyper::client::HttpConnector>>,
}

impl Client {
    /// Creates a new client with the given url and key.
    pub fn new(url: String, key: String) -> Self {
        let https = HttpsConnector::new();
        Self {
            url,
            key,
            http: HttpClient::builder().build::<_, Body>(https),
        }
    }

    pub async fn request<T>(&self, builder: RequestBuilder) -> Result<Option<T>, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let uri = format!("{}{}", self.url, builder.path)
            .parse::<Uri>()
            .unwrap();

        let req = Request::builder()
            .method(builder.method)
            .uri(uri)
            .header("User-Agent", "Pteroxide Client")
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/json,text/plain")
            .body(Body::from(builder.body))
            .unwrap();

        let res = self.http.request(req).await;
        println!("{:#?}", res);

        match res {
            Ok(v) => match v.status() {
                StatusCode::OK | StatusCode::CREATED => {
                    let buf = hyper::body::aggregate(v).await?;
                    let data = serde_json::from_reader::<_, T>(buf.reader()).unwrap();
                    Ok(Some(data))
                }
                StatusCode::ACCEPTED | StatusCode::NO_CONTENT => Ok(None),
                StatusCode::BAD_REQUEST
                | StatusCode::UNAUTHORIZED
                | StatusCode::FORBIDDEN
                | StatusCode::NOT_FOUND
                | StatusCode::METHOD_NOT_ALLOWED
                | StatusCode::CONFLICT
                | StatusCode::UNPROCESSABLE_ENTITY
                | StatusCode::TOO_MANY_REQUESTS => {
                    let buf = hyper::body::aggregate(v).await?;
                    let data = serde_json::from_reader::<_, FractalError>(buf.reader())
                        .expect("couldn't deserialize error");
                    Err(Error::from(data))
                }
                // indeterminable
                _ => Err(Error::default()),
            },
            Err(e) => Err(Error::from(e)),
        }
    }

    pub fn get_account(&self) -> GetAccount {
        GetAccount::new(self)
    }

    pub fn get_api_keys(&self) -> GetApiKeys {
        GetApiKeys::new(self)
    }
}
