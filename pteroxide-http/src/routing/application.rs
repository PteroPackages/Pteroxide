use hyper::Method;

use super::Route;

#[derive(Debug)]
pub enum Application {
    GetUsers,
    GetUser { id: i32 },
    CreateUser,
    UpdateUser { id: i32 },
    DeleteUser { id: i32 },
    GetServers,
    GetServer { id: i32 },
    CreateServer,
    SuspendServer { id: i32 },
    UnsuspendServer { id: i32 },
    ReinstallServer { id: i32 },
    DeleteServer { id: i32, force: bool },
    GetNodes,
    GetNode { id: i32 },
    GetNodeConfig { id: i32 },
    CreateNode,
    UpdateNode { id: i32 },
    DeleteNode { id: i32 },
    GetLocations,
    GetLocation { id: i32 },
    CreateLocation,
    UpdateLocation { id: i32 },
    DeleteLocation { id: i32 },
}

impl Application {
    /// Returns the corresponding method for the current route.
    pub fn method(&self) -> Method {
        match self {
            Application::GetUsers
            | Application::GetUser { .. }
            | Application::GetServers
            | Application::GetServer { .. }
            | Application::GetNodes
            | Application::GetNode { .. }
            | Application::GetNodeConfig { .. }
            | Application::GetLocations
            | Application::GetLocation { .. } => Method::GET,
            Application::CreateUser
            | Application::CreateServer
            | Application::SuspendServer { .. }
            | Application::UnsuspendServer { .. }
            | Application::ReinstallServer { .. }
            | Application::CreateNode
            | Application::CreateLocation => Method::POST,
            Application::UpdateUser { .. }
            | Application::UpdateNode { .. }
            | Application::UpdateLocation { .. } => Method::PATCH,
            Application::DeleteUser { .. }
            | Application::DeleteServer { .. }
            | Application::DeleteNode { .. }
            | Application::DeleteLocation { .. } => Method::DELETE,
        }
    }
}

impl ToString for Application {
    fn to_string(&self) -> String {
        match self {
            Application::GetUsers | Application::CreateUser => {
                String::from("/api/application/users")
            }
            Application::GetUser { id }
            | Application::UpdateUser { id }
            | Application::DeleteUser { id } => {
                format!("/api/application/users/{}", id)
            }
            Application::GetServers | Application::CreateServer => {
                String::from("/api/application/servers")
            }
            Application::GetServer { id } => format!("/api/application/servers/{}", id),
            Application::SuspendServer { id } => format!("/api/application/servers/{}/suspend", id),
            Application::UnsuspendServer { id } => {
                format!("/api/application/servers/{}/unsuspend", id)
            }
            Application::ReinstallServer { id } => {
                format!("/api/application/servers/{}/reinstall", id)
            }
            Application::DeleteServer { id, force } => {
                if *force {
                    format!("/api/application/servers/{}/force", id)
                } else {
                    format!("/api/application/servers/{}", id)
                }
            }
            Application::GetNodes | Application::CreateNode => {
                String::from("/api/application/nodes")
            }
            Application::GetNode { id }
            | Application::UpdateNode { id }
            | Application::DeleteNode { id } => {
                format!("/api/application/nodes/{}", id)
            }
            Application::GetNodeConfig { id } => {
                format!("/api/application/nodes/{}/configuration", id)
            }
            Application::GetLocations | Application::CreateLocation => {
                String::from("/api/application/locations")
            }
            Application::GetLocation { id }
            | Application::UpdateLocation { id }
            | Application::DeleteLocation { id } => format!("/api/application/locations/{}", id),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Route> for Application {
    fn into(self) -> Route {
        Route::Application(self)
    }
}
