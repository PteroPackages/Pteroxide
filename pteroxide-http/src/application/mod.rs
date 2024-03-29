use hyper::{
    body::{self, Buf},
    client::HttpConnector,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client as HClient, Request, StatusCode,
};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::Deserialize;
use serde_json::{self, Value};

use self::{
    allocations::{CreateAllocations, DeleteAllocation, GetAllocations},
    eggs::{GetEgg, GetEggs},
    locations::{CreateLocation, DeleteLocation, GetLocation, GetLocations, UpdateLocation},
    nests::{GetNest, GetNests},
    nodes::{CreateNode, DeleteNode, GetNode, GetNodeConfiguration, GetNodes, UpdateNode},
    servers::{
        CreateServer, DeleteServer, GetServer, GetServers, ReinstallServer, SuspendServer,
        UnsuspendServer, UpdateServerBuild, UpdateServerDetails, UpdateServerStartup,
    },
    users::{CreateUser, DeleteUser, GetUser, GetUsers, UpdateUser},
};
use super::{error::*, Builder};

pub mod allocations;
pub mod eggs;
pub mod locations;
pub mod nests;
pub mod nodes;
pub mod servers;
pub mod users;

/// The main interface for interacting with the application API.
#[derive(Debug)]
pub struct Application {
    http: HClient<HttpsConnector<HttpConnector>>,
    url: String,
    key: String,
}

impl Application {
    /// Constructs a new [`Application`] with the given API credentials.
    pub fn new(url: String, mut key: String) -> Self {
        let conn = HttpsConnector::new();
        if !key.starts_with("Bearer ") {
            key.insert_str(0, "Bearer ");
        }

        Self {
            http: HClient::builder().build(conn),
            url,
            key,
        }
    }

    /// Performs an API request using the [`Builder`] with the set fields. Returns a result with
    /// the deserialized API response, if any.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let builder = Builder::new(Route::GetUser { id: 2 });
    /// let data = app.request::<FractalItem<User>>(builder).await?;
    /// println!("{:#?}", data.attributes);
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the response fails to be deserialized.
    pub async fn request<T>(&self, mut builder: Builder) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let req = Request::builder()
            .uri(builder.uri(self.url.clone())) // problematic
            .method(builder.method)
            .header(USER_AGENT, "Pteroxide HTTP Application")
            .header(AUTHORIZATION, self.key.clone())
            .header(CONTENT_TYPE, builder.content_type)
            .header(ACCEPT, builder.accept_type)
            .body(builder.body)?;

        let res = self.http.request(req).await?;
        match res.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader(buf.reader())
                    .expect("failed to deserialize into model");

                Ok(data)
            }
            StatusCode::NO_CONTENT => serde_json::from_value(Value::Null).map_err(|_| Error {
                kind: ErrorKind::DeserializeError,
                source: None,
            }),
            _ => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader::<_, FractalError>(buf.reader())
                    .expect("failed to deserialize into model");

                Err(Error::from(data))
            }
        }
    }

    /// Returns a request builder for getting a list of [`User`]s.
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn get_users(&self) -> GetUsers<'_> {
        GetUsers::new(self)
    }

    /// Returns a request builder for getting a specified [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn get_user(&self, id: i32) -> GetUser<'_> {
        GetUser::new(self, id)
    }

    /// Returns a request builder for creating a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub fn create_user(&self) -> CreateUser<'_> {
        CreateUser::new(self)
    }

    /// Returns a request builder for updating a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub fn update_user(&self, id: i32) -> UpdateUser<'_> {
        UpdateUser::new(self, id)
    }

    /// Returns a request builder for deleting a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn delete_user(&self, id: i32) -> DeleteUser<'_> {
        DeleteUser::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Server`]s.
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn get_servers(&self) -> GetServers<'_> {
        GetServers::new(self)
    }

    /// Returns a request builder for getting a specified [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn get_server(&self, id: i32) -> GetServer<'_> {
        GetServer::new(self, id)
    }

    /// Returns a request builder for creating a [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub fn create_server(&self) -> CreateServer<'_> {
        CreateServer::new(self)
    }

    /// Returns a request builder for updating a [`Server`]'s build configuration.
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub fn update_server_build(&self, id: i32) -> UpdateServerBuild<'_> {
        UpdateServerBuild::new(self, id)
    }

    /// Returns a request builder for updating a [`Server`]'s details.
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub fn update_server_details(&self, id: i32) -> UpdateServerDetails<'_> {
        UpdateServerDetails::new(self, id)
    }

    /// Returns a request builder for updating a [`Server`]'s startup configuration.
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub fn update_server_startup(&self, id: i32) -> UpdateServerStartup<'_> {
        UpdateServerStartup::new(self, id)
    }

    /// Returns a request builder for suspending a [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn suspend_server(&self, id: i32) -> SuspendServer<'_> {
        SuspendServer::new(self, id)
    }

    /// Returns a request builder for unsuspending a [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn unsuspend_server(&self, id: i32) -> UnsuspendServer<'_> {
        UnsuspendServer::new(self, id)
    }

    /// Returns a request builder for triggering the reinstall process of a [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn reinstall_server(&self, id: i32) -> ReinstallServer<'_> {
        ReinstallServer::new(self, id)
    }

    /// Returns a request builder for deleting a [`Server`].
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn delete_server(&self, id: i32) -> DeleteServer<'_> {
        DeleteServer::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Node`]s.
    ///
    /// [`Node`]: pteroxide_models::application::Node
    pub const fn get_nodes(&self) -> GetNodes<'_> {
        GetNodes::new(self)
    }

    /// Returns a request builder for getting a specified [`Node`].
    ///
    /// [`Node`]: pteroxide_models::application::Node
    pub const fn get_node(&self, id: i32) -> GetNode<'_> {
        GetNode::new(self, id)
    }

    /// Returns a request builder for getting the [`configuration`] of a specified node.
    ///
    /// [`configuration`]: pteroxide_models::application::NodeConfiguration
    pub const fn get_node_configuration(&self, id: i32) -> GetNodeConfiguration<'_> {
        GetNodeConfiguration::new(self, id)
    }

    /// Returns a request builder for creating a [`Node`].
    ///
    /// [`Node`]: pteroxide_models::application::Node
    pub fn create_node(&self) -> CreateNode<'_> {
        CreateNode::new(self)
    }

    /// Returns a request builder for updating a [`Node`].
    ///
    /// [`Node`]: pteroxide_models::application::Node
    pub fn update_node(&self, id: i32) -> UpdateNode<'_> {
        UpdateNode::new(self, id)
    }

    /// Returns a request builder for deleting a [`Node`].
    ///
    /// [`Node`]: pteroxide_models::application::Node
    pub const fn delete_node(&self, id: i32) -> DeleteNode<'_> {
        DeleteNode::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Location`]s.
    ///
    /// [`Location`]: pteroxide_models::application::Location
    pub const fn get_locations(&self) -> GetLocations<'_> {
        GetLocations::new(self)
    }

    /// Returns a request builder for getting a specified [`Location`].
    ///
    /// [`Location`]: pteroxide_models::application::Location
    pub const fn get_location(&self, id: i32) -> GetLocation<'_> {
        GetLocation::new(self, id)
    }

    /// Returns a request builder for creating a [`Location`].
    ///
    /// [`Location`]: pteroxide_models::application::Location
    pub fn create_location(&self) -> CreateLocation<'_> {
        CreateLocation::new(self)
    }

    /// Returns a request builder for updating a [`Location`].
    ///
    /// [`Location`]: pteroxide_models::application::Location
    pub fn update_location(&self, id: i32) -> UpdateLocation<'_> {
        UpdateLocation::new(self, id)
    }

    /// Returns a request builder for deleting a [`Location`].
    ///
    /// [`Location`]: pteroxide_models::application::Location
    pub const fn delete_location(&self, id: i32) -> DeleteLocation<'_> {
        DeleteLocation::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Allocation`]s.
    ///
    /// [`Allocation`]: pteroxide_models::application::Allocation
    pub const fn get_allocations(&self, node: i32) -> GetAllocations<'_> {
        GetAllocations::new(self, node)
    }

    /// Returns a request builder for creating [`Allocation`]s.
    ///
    /// [`Allocation`]: pteroxide_models::application::Allocation
    pub fn create_allocations(&self, node: i32) -> CreateAllocations<'_> {
        CreateAllocations::new(self, node)
    }

    /// Returns a request builder for deleting an [`Allocation`].
    ///
    /// [`Allocation`]: pteroxide_models::application::Allocation
    pub const fn delete_allocation(&self, node: i32, id: i32) -> DeleteAllocation<'_> {
        DeleteAllocation::new(self, node, id)
    }

    /// Returns a request builder for getting a list of [`Nest`]s.
    ///
    /// [`Nest`]: pteroxide_models::application::Nest
    pub const fn get_nests(&self) -> GetNests<'_> {
        GetNests::new(self)
    }

    /// Returns a request builder for getting a specified [`Nest`].
    ///
    /// [`Nest`]: pteroxide_models::application::Nest
    pub const fn get_nest(&self, id: i32) -> GetNest<'_> {
        GetNest::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Egg`]s.
    ///
    /// [`Egg`]: pteroxide_models::application::Egg
    pub const fn get_nest_eggs(&self, nest: i32) -> GetEggs<'_> {
        GetEggs::new(self, nest)
    }

    /// Returns a request builder for getting a specified [`Egg`].
    ///
    /// [`Egg`]: pteroxide_models::application::Egg
    pub const fn get_nest_egg(&self, nest: i32, id: i32) -> GetEgg<'_> {
        GetEgg::new(self, nest, id)
    }
}
