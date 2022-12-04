use pteroxide_models::{application::User, fractal::{FractalItem, FractalList}};

use crate::{routing::application::Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetUsers<'a> {
    app: &'a Application,
    with_servers: bool,
}

impl<'a> GetUsers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app, with_servers: false }
    }

    /// Include the [`Server`]s the user has access to in the user [`relationships`].
    /// 
    /// [`relationships`]: pteroxide_models::application::UserRelations
    /// [`Server`]: pteroxide_models::application::Server
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`User`] objects.
    /// 
    /// ## Errors
    /// 
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<User>, Error> {
        let mut builder = Builder::default().route(Route::GetUsers);
        if self.with_servers {
            builder = builder.param("include", "servers");
        }

        let res = self
            .app
            .request::<FractalList<User>>(builder)
            .await?;

        Ok(res.data.iter().map(|u| u.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetUser<'a> {
    app: &'a Application,
    id: i32,
    with_servers: bool,
}

impl<'a> GetUser<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id, with_servers: false }
    }

    /// Include the [`Server`]s the user has access to in the user [`relationships`].
    /// 
    /// [`relationships`]: pteroxide_models::application::UserRelations
    /// [`Server`]: pteroxide_models::application::Server
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`User`] objects.
    /// 
    /// ## Errors
    /// 
    /// Returns an [`Error`] if the request fails or if the user is not found.
    pub async fn exec(&self) -> Result<User, Error> {
        let mut builder = Builder::default().route(Route::GetUser { id: self.id });
        if self.with_servers {
            builder = builder.param("include", "servers");
        }

        let res = self
            .app
            .request::<FractalItem<User>>(builder)
            .await?;

        Ok(res.attributes)
    }
}
