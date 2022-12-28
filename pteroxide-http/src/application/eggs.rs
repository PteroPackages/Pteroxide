use pteroxide_models::{
    application::Egg,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetEggs<'a> {
    app: &'a Application,
    nest: i32,
    with_config: bool,
    with_nest: bool,
    with_script: bool,
    with_servers: bool,
}

impl<'a> GetEggs<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, nest: i32) -> Self {
        Self {
            app,
            nest,
            with_config: false,
            with_nest: false,
            with_script: false,
            with_servers: false,
        }
    }

    /// Include the main [`config`] of the egg in the egg [`relationships`].
    ///
    /// [`config`]: pteroxide_models::application::EggConfig
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_config(mut self, value: bool) -> Self {
        self.with_config = value;

        self
    }

    /// Include the [`nest`] the egg is part of in the egg [`relationships`].
    ///
    /// [`nest`]: pteroxide_models::application::Nest
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_nest(mut self, value: bool) -> Self {
        self.with_nest = value;

        self
    }

    /// Include the [`script`] configuration of the egg in the egg [`relationships`].
    ///
    /// [`script`]: pteroxide_models::application::EggScript
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_script(mut self, value: bool) -> Self {
        self.with_script = value;

        self
    }

    /// Include the [`servers`] using the egg in the egg [`relationships`].
    ///
    /// [`servers`]: pteroxide_models::application::Server
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`Egg`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Egg>, Error> {
        let mut builder = Builder::new(Route::GetEggs { nest: self.nest }.into());

        if self.with_config {
            builder = builder.include("config");
        }
        if self.with_nest {
            builder = builder.include("nest");
        }
        if self.with_script {
            builder = builder.include("script");
        }
        if self.with_servers {
            builder = builder.include("servers");
        }

        let res = self.app.request::<FractalList<Egg>>(builder).await?;

        Ok(res.data.iter().map(|e| e.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetEgg<'a> {
    app: &'a Application,
    nest: i32,
    id: i32,
    with_config: bool,
    with_nest: bool,
    with_script: bool,
    with_servers: bool,
}

impl<'a> GetEgg<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, nest: i32, id: i32) -> Self {
        Self {
            app,
            nest,
            id,
            with_config: false,
            with_nest: false,
            with_script: false,
            with_servers: false,
        }
    }

    /// Include the main [`config`] of the egg in the egg [`relationships`].
    ///
    /// [`config`]: pteroxide_models::application::EggConfig
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_config(mut self, value: bool) -> Self {
        self.with_config = value;

        self
    }

    /// Include the [`nest`] the egg is part of in the egg [`relationships`].
    ///
    /// [`nest`]: pteroxide_models::application::Nest
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_nest(mut self, value: bool) -> Self {
        self.with_nest = value;

        self
    }

    /// Include the [`script`] configuration of the egg in the egg [`relationships`].
    ///
    /// [`script`]: pteroxide_models::application::EggScript
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_script(mut self, value: bool) -> Self {
        self.with_script = value;

        self
    }

    /// Include the [`servers`] using the egg in the egg [`relationships`].
    ///
    /// [`servers`]: pteroxide_models::application::Server
    /// [`relationships`]: pteroxide_models::application::EggRelations
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a [`Egg`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the egg is not found.
    pub async fn exec(&self) -> Result<Egg, Error> {
        let mut builder = Builder::new(
            Route::GetEgg {
                nest: self.nest,
                id: self.id,
            }
            .into(),
        );

        if self.with_config {
            builder = builder.include("config");
        }
        if self.with_nest {
            builder = builder.include("nest");
        }
        if self.with_script {
            builder = builder.include("script");
        }
        if self.with_servers {
            builder = builder.include("servers");
        }

        let res = self.app.request::<FractalItem<Egg>>(builder).await?;

        Ok(res.attributes)
    }
}
