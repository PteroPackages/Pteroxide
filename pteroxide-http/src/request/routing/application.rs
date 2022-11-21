use hyper::Method;

/// Route members implementation for the application API.
pub enum Route {
    ListUsers,
    GetUser { id: i32 },
}

impl Route {
    pub const fn method(&self) -> Method {
        match self {
            Route::ListUsers
            | Route::GetUser { .. } => Method::GET
        }
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::ListUsers => String::from("/api/application/users"),
            Route::GetUser { id } => String::from(format!("/api/application/users/{}", id)),
        }
    }
}
