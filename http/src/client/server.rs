use serde_json::json;
use pteroxide_models::{
    fractal::{FractalList, FractalData},
    client::server::{
        PowerState, Server, ServerStatistics, WebSocketAuth, WebSocketWrapper,
    },
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

/// Gets the server's current resource utilization.
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
///     let res = client.get_server_resources("8d93a926".to_string())
///         .exec()
///         .await
///         .expect("couldn't get server resources");
/// 
///     println!("{:#?}", res);
/// }
/// ```
pub struct GetServerResources<'a> {
    http: &'a Client,
    id: String,
}

impl<'a> GetServerResources<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id }
    }

    /// Executes a request and returns the [`ServerStatistics`] if successful.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<ServerStatistics, Error> {
        match self.http.request::<FractalData<ServerStatistics>>(
            RequestBuilder::new(&format!("/api/client/servers/{}/resources", self.id))
        ).await {
            Ok(v) => Ok(v.unwrap().attributes),
            Err(e) => Err(e),
        }
    }
}

/// Sends a command to a specified server's console.
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
///     client.send_server_command("8d93a926".to_string())
///         .command("say hello rust!")
///         .exec()
///         .await
///         .expect("couldn't send server command");
/// }
/// ```
pub struct SendServerCommand<'a> {
    http: &'a Client,
    id: String,
    cmd: String,
}

impl<'a> SendServerCommand<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            cmd: Default::default(),
        }
    }

    /// Sets the command to be sent to the server.
    pub fn command(mut self, cmd: String) -> Self {
        self.cmd = cmd;

        self
    }

    /// Executes a request to send a command to the server's console.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`FieldError`] if no command was specified.
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`FieldError`]: crate::errors::ErrorKind::FieldError
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<(), Error> {
        if self.cmd.is_empty() {
            return Err(Error::from("no command specified"));
        }

        let mut req = RequestBuilder::new(&format!("/api/client/servers/{}/command", self.id));
        req.method("POST")?;
        req.json(json!({
            "command": self.cmd
        }));

        match self.http.request::<()>(req).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Sets the power state of a specified server. Defaults to `start` if `state()` is not specfiied.
/// 
/// ## Example
/// ```no_run
/// use pteroxide_http::client::Client;
/// use pteroxide_models::client::server::PowerState;
/// 
/// #[tokio::main]
/// async fn main() {
///     let client = Client::new(
///         "https://pterodactyl.domain".to_string(),
///         "client_api_key".to_string(),
///     );
/// 
///     client.set_power_state("8d93a926".to_string())
///         .state(PowerState::RESTART)
///         .exec()
///         .await
///         .expect("couldn't set server power state");
/// }
/// ```
pub struct SetPowerState<'a> {
    http: &'a Client,
    id: String,
    state: PowerState,
}

impl<'a> SetPowerState<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            state: Default::default(),
        }
    }

    /// The power state to set the server to.
    pub fn state(mut self, state: PowerState) -> Self {
        self.state = state;

        self
    }

    /// Executes a request to set the server's power state.
    /// 
    /// ## Errors
    /// Returns an [`Error`] with the kind [`RequestError`] if the request failed to execute.
    /// 
    /// [`RequestError`]: crate::errors::ErrorKind::RequestError
    pub async fn exec(self) -> Result<(), Error> {
        let mut req = RequestBuilder::new(&format!("/api/client/servers/{}/power", self.id));
        req.method("POST")?;
        req.json(json!({
            "signal": self.state.to_string()
        }));

        match self.http.request::<()>(req).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
