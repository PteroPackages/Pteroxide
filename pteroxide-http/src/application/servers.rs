use pteroxide_models::{application::Server, fractal::FractalList};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetServers<'a> {
    app: &'a Application,
    with_owner: bool,
}

impl<'a> GetServers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self {
            app,
            with_owner: false,
        }
    }

    /// Include the server [`owner`] in the server [`relationships`].
    ///
    /// [`owner`]: pteroxide_models::application::User
    pub fn with_owner(mut self, value: bool) -> Self {
        self.with_owner = value;

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
            builder = builder.param("include", "user")
        }

        let res = self.app.request::<FractalList<Server>>(builder).await?;

        Ok(res.data.iter().map(|s| s.attributes.clone()).collect())
    }
}
