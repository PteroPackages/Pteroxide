use pteroxide_models::{application::Server, fractal::FractalList};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetServers<'a> {
    app: &'a Application,
}

impl<'a> GetServers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    /// Asynchronously executes the request and returns a list of [`Server`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Server>, Error> {
        let res = self
            .app
            .request::<FractalList<Server>>(Builder::default().route(Route::GetServers.into()))
            .await?;

        Ok(res.data.iter().map(|s| s.attributes.clone()).collect())
    }
}
