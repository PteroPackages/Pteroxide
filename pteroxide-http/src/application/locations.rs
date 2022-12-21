use pteroxide_models::{
    application::Location,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetLocations<'a> {
    app: &'a Application,
}

impl<'a> GetLocations<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    /// Asynchronously executes the request and returns a list of [`Location`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Location>, Error> {
        let res = self
            .app
            .request::<FractalList<Location>>(Builder::new(Route::GetLocations.into()))
            .await?;

        Ok(res.data.iter().map(|l| l.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetLocation<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> GetLocation<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<Location, Error> {
        let res = self
            .app
            .request::<FractalItem<Location>>(Builder::new(
                Route::GetLocation { id: self.id }.into(),
            ))
            .await?;

        Ok(res.attributes)
    }
}
