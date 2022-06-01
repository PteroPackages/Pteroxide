//! Implementations for requests to the Client API.

use bytes::Buf;
use hyper::{client::Client as HttpClient, Body, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::de::Deserialize;

use crate::{
    errors::Error,
    requests::{
        account::{
            CreateApiKey, DeleteApiKey, GetAccount, GetApiKeys, GetTwoFactorCode, UpdateAccount,
            UpdateTwoFactor,
        },
        RequestBuilder,
        server::GetServers,
    },
};

/// The manager for interacting with the Pterodactyl Client API.
/// 
/// ## Examples
/// ```no_run
/// use pteroxide_http::client::Client;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     let acc = app.get_account()
///         .exec()
///         .await
///         .expect("couldn't get the account");
/// 
///     println!("{:#?}", acc);
/// }
/// ```
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

    /// Performs a new request to the client API and returns the resulting [`Option<T>`].
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request fails.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
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
            .header("Content-Type", "application/json")
            .header("Accept", "application/json,text/plain")
            .body(builder.body)
            .unwrap_or_default();

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

    /// Returns a request builder for getting the account.
    pub fn get_account(&self) -> GetAccount {
        GetAccount::new(self)
    }

    /// Returns a request builder for getting API keys.
    pub fn get_api_keys(&self) -> GetApiKeys {
        GetApiKeys::new(self)
    }

    /// Returns a request builder for creating an API key.
    pub fn create_api_key(&self) -> CreateApiKey {
        CreateApiKey::new(self)
    }

    /// Returns a request builder for deleting an API key.
    pub fn delete_api_key(&self, id: String) -> DeleteApiKey {
        DeleteApiKey::new(self, id)
    }

    /// Returns a request builder for getting a two-factor authentication code.
    pub fn get_two_factor_code(&self) -> GetTwoFactorCode {
        GetTwoFactorCode::new(self)
    }

    /// Returns a request builder for updating the account.
    pub fn update_account(&self) -> UpdateAccount {
        UpdateAccount::new(self)
    }

    /// Returns a request builder for updating the two-factor status.
    pub fn update_two_factor(&self) -> UpdateTwoFactor {
        UpdateTwoFactor::new(self)
    }

    pub fn get_servers(&self) -> GetServers {
        GetServers::new(self)
    }
}
