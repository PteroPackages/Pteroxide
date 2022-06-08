use pteroxide_models::{
    fractal::FractalList,
    client::backups::Backup,
};

use crate::{
    client::Client,
    errors::Error,
    requests::RequestBuilder,
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
            RequestBuilder::new(&format!("/api/client/servers/{}/backups", self.id))
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
