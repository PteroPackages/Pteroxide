use pteroxide_models::{
    application::Egg,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetEggs<'a> {
    app: &'a Application,
    nest: i32,
}

impl<'a> GetEggs<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, nest: i32) -> Self {
        Self { app, nest }
    }

    /// Asynchronously executes the request and returns a list of [`Egg`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Egg>, Error> {
        let res = self
            .app
            .request::<FractalList<Egg>>(Builder::new(Route::GetEggs { nest: self.nest }.into()))
            .await?;

        Ok(res.data.iter().map(|e| e.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetEgg<'a> {
    app: &'a Application,
    nest: i32,
    id: i32,
}

impl<'a> GetEgg<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, nest: i32, id: i32) -> Self {
        Self { app, nest, id }
    }

    /// Asynchronously executes the request and returns a [`Egg`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the egg is not found.
    pub async fn exec(&self) -> Result<Egg, Error> {
        let res = self
            .app
            .request::<FractalItem<Egg>>(Builder::new(
                Route::GetEgg {
                    nest: self.nest,
                    id: self.id,
                }
                .into(),
            ))
            .await?;

        Ok(res.attributes)
    }
}
