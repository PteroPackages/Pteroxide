use pteroxide_models::{
    fractal::FractalList,
    client::server::{Server, WebSocketAuth, WebSocketWrapper},
};

use crate::{
    client::Client,
    errors::Error,
    requests::RequestBuilder,
};

/// Gets a list of servers associated with the account.
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
///     client.get_servers()
///         .exec()
///         .await
///         .expect("couldn't get servers")
///         .iter()
///         .for_each(|s| println!("{}", s.name));
/// }
/// ```
pub struct GetServers<'a> {
    http: &'a Client,
    access: String,
}

impl<'a> GetServers<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client) -> Self {
        Self {
            http,
            access: String::from("admin"),
        }
    }

    pub fn access(mut self, access: &str) -> Result<Self, Error> {
        match access {
            "admin" | "admin-all" | "owner" => self.access = String::from(access),
            _ => return Err(Error::from("invalid access type")),
        }

        Ok(self)
    }

    /// Executes a request and returns a list of [`Server`]s if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<Vec<Server>, Error> {
        match self.http.request::<FractalList<Server>>(
            RequestBuilder::new(&format!("/api/client?type={}", self.access))
        ).await {
            Ok(v) => Ok(v.unwrap()
                .data
                .iter()
                .map(|k| k.attributes.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }
}

/// Gets the websocket authentication details for a specified server.
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
///     let auth = client.get_server_ws("8d93a926".to_string())
///         .exec()
///         .await
///         .expect("couldn't get server ws");
/// 
///     println!("{}\n{}", auth.socket, auth.token);
/// }
/// ```
pub struct GetServerWebSocket<'a> {
    http: &'a Client,
    id: String,
}

impl<'a> GetServerWebSocket<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id }
    }

    /// Executes a request and returns the server's [`WebSocketAuth`] if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<WebSocketAuth, Error> {
        match self.http.request::<WebSocketWrapper>(
            RequestBuilder::new(&format!("/api/client/servers/{}/websocket", self.id))
        ).await {
            Ok(v) => Ok(v.unwrap().data),
            Err(e) => Err(e),
        }
    }
}
