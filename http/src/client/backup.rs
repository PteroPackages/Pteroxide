use pteroxide_models::{
    fractal::{FractalList, FractalData},
    client::backups::Backup,
};
use serde_json::json;

use crate::{
    client::Client,
    errors::Error,
    request::Builder,
};

/// Gets a list of backups on a specific server.
/// 
/// ## Example
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
///     client.get_backups("8d93a926".to_string())
///         .exec()
///         .await
///         .expect("couldn't get server backups")
///         .iter()
///         .for_each(|s| println!("{}", s.name));
/// }
/// ```
pub struct GetBackups<'a> {
    http: &'a Client,
    id: String,
}

impl<'a> GetBackups<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id }
    }

    /// Executes a request and returns a list of [`Backup`]s from the server if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<Vec<Backup>, Error> {
        match self.http.request::<FractalList<Backup>>(
            Builder::new(&format!("/api/client/servers/{}/backups", self.id))
        ).await {
            Ok(v) => Ok(v.unwrap()
                .data
                .iter()
                .map(|b| b.attributes.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }
}

/// Creates a backup on a specified server.
/// 
/// ## Example
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
///     let backup = client.create_backup("8d93a926".to_string())
///         .name(Some("my new backup".to_string()))
///         .locked(Some(true))
///         .exec()
///         .await
///         .expect("couldn't create the backup");
/// 
///     println!("{}", backup.uuid);
/// }
/// ```
pub struct CreateBackup<'a> {
    http: &'a Client,
    id: String,
    name: Option<String>,
    is_locked: Option<bool>,
    ignored: Option<String>,
}

impl<'a> CreateBackup<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            name: None,
            is_locked: None,
            ignored: None,
        }
    }

    /// Sets the name for the backup.
    pub fn name(mut self, value: Option<String>) -> Self {
        self.name = value;

        self
    }

    /// Whether to lock the backup on the server.
    pub fn locked(mut self, value: Option<bool>) -> Self {
        self.is_locked = value;

        self
    }

    /// Sets the files to be ignored in the backup.
    pub fn ignore(mut self, value: Option<String>) -> Self {
        self.ignored = value;

        self
    }

    /// Executes a request and returns the new [`Backup`] if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<Backup, Error> {
        let mut req = Builder::new(&format!("/api/client/servers/{}/backups", self.id))
            .method("POST")?;

        if self.name.is_some() || self.is_locked.is_some() || self.ignored.is_some() {
            req = req.body(json!({
                "name": self.name,
                "is_locked": self.is_locked,
                "ignored": self.ignored
            }));
        }

        match self.http.request::<FractalData<Backup>>(req).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}
