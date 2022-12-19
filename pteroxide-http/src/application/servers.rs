use pteroxide_models::{application::Server, fractal::FractalList};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetServers<'a> {
    app: &'a Application,
    with_owner: bool,
    with_subusers: bool,
}

impl<'a> GetServers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self {
            app,
            with_owner: false,
            with_subusers: false,
        }
    }

    /// Include the server [`owner`] in the server [`relationships`].
    ///
    /// [`owner`]: pteroxide_models::application::User
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_owner(mut self, value: bool) -> Self {
        self.with_owner = value;

        self
    }

    /// Include the server [`SubUser`]s in the server [`relationships`].
    ///
    /// [`SubUser`]: pteroxide_models::application::SubUser
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_subusers(mut self, value: bool) -> Self {
        self.with_subusers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`Server`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Server>, Error> {
        let mut builder = Builder::new(Route::GetServers.into());

        if self.with_owner {
            builder = builder.include("user");
        }
        if self.with_subusers {
            builder = builder.include("subusers");
        }

        let res = self.app.request::<FractalList<Server>>(builder).await?;

        Ok(res.data.iter().map(|s| s.attributes.clone()).collect())
    }
}
