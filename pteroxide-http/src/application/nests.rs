use pteroxide_models::{
    application::Nest,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetNests<'a> {
    app: &'a Application,
}

impl<'a> GetNests<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    /// Asynchronously executes the request and returns a list of [`Nest`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Nest>, Error> {
        let res = self
            .app
            .request::<FractalList<Nest>>(Builder::new(Route::GetNests.into()))
            .await?;

        Ok(res.data.iter().map(|n| n.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetNest<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> GetNest<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    /// Asynchronously executes the request and returns a [`Nest`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the nest is not found.
    pub async fn exec(&self) -> Result<Nest, Error> {
        let res = self
            .app
            .request::<FractalItem<Nest>>(Builder::new(Route::GetNest { id: self.id }.into()))
            .await?;

        Ok(res.attributes)
    }
}
