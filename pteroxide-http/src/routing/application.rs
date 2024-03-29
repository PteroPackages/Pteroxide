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
    UpdateServerBuild { id: i32 },
    UpdateServerDetails { id: i32 },
    UpdateServerStartup { id: i32 },
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
    GetAllocations { node: i32 },
    CreateAllocations { node: i32 },
    DeleteAllocation { node: i32, id: i32 },
    GetNests,
    GetNest { id: i32 },
    GetEggs { nest: i32 },
    GetEgg { nest: i32, id: i32 },
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
            | Application::GetLocation { .. }
            | Application::GetAllocations { .. }
            | Application::GetNests
            | Application::GetNest { .. }
            | Application::GetEggs { .. }
            | Application::GetEgg { .. } => Method::GET,
            Application::CreateUser
            | Application::CreateServer
            | Application::SuspendServer { .. }
            | Application::UnsuspendServer { .. }
            | Application::ReinstallServer { .. }
            | Application::CreateNode
            | Application::CreateLocation
            | Application::CreateAllocations { .. } => Method::POST,
            Application::UpdateUser { .. }
            | Application::UpdateServerBuild { .. }
            | Application::UpdateServerDetails { .. }
            | Application::UpdateServerStartup { .. }
            | Application::UpdateNode { .. }
            | Application::UpdateLocation { .. } => Method::PATCH,
            Application::DeleteUser { .. }
            | Application::DeleteServer { .. }
            | Application::DeleteNode { .. }
            | Application::DeleteLocation { .. }
            | Application::DeleteAllocation { .. } => Method::DELETE,
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
            Application::UpdateServerBuild { id } => {
                format!("/api/application/servers/{}/build", id)
            }
            Application::UpdateServerDetails { id } => {
                format!("/api/application/servers/{}/details", id)
            }
            Application::UpdateServerStartup { id } => {
                format!("/api/application/servers/{}/startup", id)
            }
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
            Application::GetAllocations { node } | Application::CreateAllocations { node } => {
                format!("/api/application/nodes/{}/allocations", node)
            }
            Application::DeleteAllocation { node, id } => {
                format!("/api/application/nodes/{}/allocations/{}", node, id)
            }
            Application::GetNests => String::from("/api/application/nests"),
            Application::GetNest { id } => format!("/api/application/nests/{}", id),
            Application::GetEggs { nest } => format!("/api/application/nests/{}/eggs", nest),
            Application::GetEgg { nest, id } => {
                format!("/api/application/nests/{}/eggs/{}", nest, id)
            }
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Route> for Application {
    fn into(self) -> Route {
        Route::Application(self)
    }
}
