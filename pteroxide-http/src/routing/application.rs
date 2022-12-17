use hyper::Method;

use super::Route;

#[derive(Debug)]
pub enum Application {
    GetUsers,
    GetUser { id: i32 },
    CreateUser,
    UpdateUser { id: i32 },
}

impl Application {
    /// Returns the corresponding method for the current route.
    pub fn method(&self) -> Method {
        match self {
            Application::GetUsers | Application::GetUser { .. } => Method::GET,
            Application::CreateUser => Method::POST,
            Application::UpdateUser { .. } => Method::PATCH,
        }
    }
}

impl ToString for Application {
    fn to_string(&self) -> String {
        match self {
            Application::GetUsers | Application::CreateUser => {
                String::from("/api/application/users")
            }
            Application::GetUser { id } | Application::UpdateUser { id } => {
                format!("/api/application/users/{}", id)
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
