use pteroxide_models::{
    application::Nest,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetNests<'a> {
    app: &'a Application,
    with_eggs: bool,
    with_servers: bool,
}

impl<'a> GetNests<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self {
            app,
            with_eggs: false,
            with_servers: false,
        }
    }

    /// Include the [`eggs`] the nest contains the the nest [`relationships`].
    ///
    /// [`eggs`]: pteroxide_models::application::Egg
    /// [`relationships`]: pteroxide_models::application::NestRelations
    pub fn with_eggs(mut self, value: bool) -> Self {
        self.with_eggs = value;

        self
    }

    /// Include the [`servers`] using eggs the nest contains the the nest [`relationships`].
    ///
    /// [`servers`]: pteroxide_models::application::Server
    /// [`relationships`]: pteroxide_models::application::NestRelations
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`Nest`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Nest>, Error> {
        let mut builder = Builder::new(Route::GetNests.into());

        if self.with_eggs {
            builder = builder.include("eggs");
        }
        if self.with_servers {
            builder = builder.include("servers");
        }

        let res = self.app.request::<FractalList<Nest>>(builder).await?;

        Ok(res.data.iter().map(|n| n.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetNest<'a> {
    app: &'a Application,
    id: i32,
    with_eggs: bool,
    with_servers: bool,
}

impl<'a> GetNest<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            with_eggs: false,
            with_servers: false,
        }
    }

    /// Include the [`eggs`] the nest contains the the nest [`relationships`].
    ///
    /// [`eggs`]: pteroxide_models::application::Egg
    /// [`relationships`]: pteroxide_models::application::NestRelations
    pub fn with_eggs(mut self, value: bool) -> Self {
        self.with_eggs = value;

        self
    }

    /// Include the [`servers`] using eggs the nest contains the the nest [`relationships`].
    ///
    /// [`servers`]: pteroxide_models::application::Server
    /// [`relationships`]: pteroxide_models::application::NestRelations
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a [`Nest`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the nest is not found.
    pub async fn exec(&self) -> Result<Nest, Error> {
        let mut builder = Builder::new(Route::GetNest { id: self.id }.into());

        if self.with_eggs {
            builder = builder.include("eggs");
        }
        if self.with_servers {
            builder = builder.include("servers");
        }

        let res = self.app.request::<FractalItem<Nest>>(builder).await?;

        Ok(res.attributes)
    }
}
