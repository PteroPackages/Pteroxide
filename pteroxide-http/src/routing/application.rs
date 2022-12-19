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
}

impl Application {
    /// Returns the corresponding method for the current route.
    pub fn method(&self) -> Method {
        match self {
            Application::GetUsers
            | Application::GetUser { .. }
            | Application::GetServers
            | Application::GetServer { .. } => Method::GET,
            Application::CreateUser | Application::CreateServer => Method::POST,
            Application::UpdateUser { .. } => Method::PATCH,
            Application::DeleteUser { .. } => Method::DELETE,
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
            Application::GetServers | Application::CreateServer => String::from("/api/application/servers"),
            Application::GetServer { id } => format!("/api/application/servers/{}", id),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Route> for Application {
    fn into(self) -> Route {
        Route::Application(self)
    }
}
