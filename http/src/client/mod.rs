pub mod account;
pub mod backup;
pub mod database;
pub mod file;
pub mod server;

use bytes::Buf;
use hyper::{
    body::{aggregate, Body},
    client::{Client as HttpClient, HttpConnector},
    Request, Uri,
};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::de::Deserialize;

use crate::{
    errors::Error,
    request::Builder,
};
use self::{
    account::{
        CreateApiKey, DeleteApiKey, GetApiKeys, GetAccount, GetTwoFactorCode, UpdateAccount,
        UpdateTwoFactor,
    },
    backup::{CreateBackup, GetBackups},
    database::{CreateDatabase, DeleteDatabase, GetDatabases, RotateDatabasePassword},
    file::GetFiles,
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
    pub async fn request<T>(&self, builder: Builder) -> Result<Option<T>, Error>
    where
        for<'de> T: Deserialize<'de>
    {
        let uri = &format!("{}{}", self.url, builder.path)
            .parse::<Uri>()
            .unwrap();

        let req = Request::builder()
            .method(builder.method)
            .uri(uri)
            .header("User-Agent", "Pteroxide Client")
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Content-Type", builder.ctype.clone())
            .header("Accept", builder.ctype.clone())
            .body(builder.body)
            .unwrap();

        let res = self.http.request(req).await;
        println!("{:#?}", res);

        match res {
            Ok(v) => match v.status().as_u16() {
                200 | 201 => {
                    let buf = aggregate(v).await?;
                    let data = serde_json::from_reader(buf.reader())
                        .expect("failed to serialize data");

                    Ok(Some(data))
                }
                202 | 204 => Ok(None),
                400 | 401 | 403 | 404 | 405 | 409 | 422 | 429 => {
                    let buf = aggregate(v).await?;
                    let data = serde_json::from_reader::<_, FractalError>(buf.reader())
                        .expect("failed to serialize error");

                    Err(Error::from(data))
                }
                _ => Err(Error::default()),
            }
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

    /// Returns a request builder for getting server databases.
    pub fn get_databases(&self, id: String) -> GetDatabases {
        GetDatabases::new(self, id)
    }

    /// Returns a request builder for creating a database.
    pub fn create_database(&self, id: String) -> CreateDatabase {
        CreateDatabase::new(self, id)
    }

    /// Returns a request builder for rotating the password of a database.
    pub fn rotate_db_password(&self, id: String) -> RotateDatabasePassword {
        RotateDatabasePassword::new(self, id)
    }

    /// Returns a request builder for deleting a database.
    pub fn delete_database(&self, id: String) -> DeleteDatabase {
        DeleteDatabase::new(self, id)
    }

    pub fn get_server_files(&self, id: String) -> GetFiles {
        GetFiles::new(self, id)
    }

    /// Returns a request builder for getting server backups.
    pub fn get_backups(&self, id: String) -> GetBackups {
        GetBackups::new(self, id)
    }

    /// Returns a request builder for creating a backup on a server.
    pub fn create_backup(&self, id: String) -> CreateBackup {
        CreateBackup::new(self, id)
    }
}
