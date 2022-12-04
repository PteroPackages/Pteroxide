use hyper::Method;

/// Route members implementation for the application API.
pub enum Route {
    GetUsers,
    GetUser { id: i32 },
}

impl Route {
    /// Returns the corresponding method for the current route.
    pub const fn method(&self) -> Method {
        match self {
            Route::GetUsers | Route::GetUser { .. } => Method::GET,
        }
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::GetUsers => String::from("/api/application/users"),
            Route::GetUser { id } => format!("/api/application/users/{}", id),
        }
    }
}
