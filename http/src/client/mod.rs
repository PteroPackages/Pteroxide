//! Implementations for requests to the Client API.

pub mod account;
pub mod server;

use bytes::Buf;
use hyper::{
    Body,
    client::{Client as HttpClient, HttpConnector},
    Request, StatusCode, Uri,
};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::de::Deserialize;

use crate::{
    errors::Error,
    requests::RequestBuilder,
};
use self::{
    account::{
        CreateApiKey, DeleteApiKey, GetApiKeys, GetAccount, GetTwoFactorCode, UpdateAccount,
        UpdateTwoFactor,
    },
    server::{
        GetServers, GetServerResources, GetServerWebSocket, SendServerCommand, SetPowerState,
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
    pub http: HttpClient<HttpsConnector<HttpConnector>>,
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

    /// Returns a request builder for getting account servers.
    pub fn get_servers(&self) -> GetServers {
        GetServers::new(self)
    }

    /// Returns a request builder for getting a server's websocket details.
    pub fn get_server_ws(&self, id: String) -> GetServerWebSocket {
        GetServerWebSocket::new(self, id)
    }

    /// Returns a request builder for getting a server's resource utilization.
    pub fn get_server_resources(&self, id: String) -> GetServerResources {
        GetServerResources::new(self, id)
    }

    /// Returns a request builder for sending a command to a server's console.
    pub fn send_server_command(&self, id: String) -> SendServerCommand {
        SendServerCommand::new(self, id)
    }

    /// Returns a request builder for setting the power state of a server.
    pub fn set_power_state(&self, id: String) -> SetPowerState {
        SetPowerState::new(self, id)
    }
}
