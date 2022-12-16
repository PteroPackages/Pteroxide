pub mod application;

pub use application::Application;

#[derive(Debug)]
pub enum Route {
    Application(Application),
}

impl Route {
    pub fn method(&self) -> hyper::Method {
        match self {
            Route::Application(r) => r.method(),
        }
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Route::Application(r) => r.to_string(),
        }
    }
}
