use pteroxide_models::{
    application::Server,
    fractal::{FractalItem, FractalList},
};

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetServers<'a> {
    app: &'a Application,
    with_allocations: bool,
    with_owner: bool,
    with_subusers: bool,
    with_nest: bool,
    with_egg: bool,
    // with_variables: bool,
    with_location: bool,
    with_node: bool,
    // with_databases: bool,
    // not doing that transfer bs
}

impl<'a> GetServers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self {
            app,
            with_allocations: false,
            with_owner: false,
            with_subusers: false,
            with_nest: false,
            with_egg: false,
            with_location: false,
            with_node: false,
        }
    }

    /// Include the [`allocations`] bound to the server in the server [`relationships`].
    ///
    /// [`allocations`]: pteroxide_models::application::Allocation
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_allocations(mut self, value: bool) -> Self {
        self.with_allocations = value;

        self
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

    /// Include the [`nest`] the server's egg is part of in the server [`relationships`].
    ///
    /// [`nest`]: pteroxide_models::application::Nest
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_nest(mut self, value: bool) -> Self {
        self.with_nest = value;

        self
    }

    /// Include the [`egg`] the server uses in the server [`relationships`].
    ///
    /// [`egg`]: pteroxide_models::application::Egg
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_egg(mut self, value: bool) -> Self {
        self.with_egg = value;

        self
    }

    /// Include the [`location`] the server's node is part of in the server [`relationships`].
    ///
    /// [`location`]: pteroxide_models::application::Location
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_location(mut self, value: bool) -> Self {
        self.with_location = value;

        self
    }

    /// Include the [`node`] the server is part of in the server [`relationships`].
    ///
    /// [`node`]: pteroxide_models::application::Node
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_node(mut self, value: bool) -> Self {
        self.with_node = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`Server`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Server>, Error> {
        let mut builder = Builder::new(Route::GetServers.into());

        if self.with_allocations {
            builder = builder.include("allocations");
        }
        if self.with_owner {
            builder = builder.include("user");
        }
        if self.with_subusers {
            builder = builder.include("subusers");
        }
        if self.with_nest {
            builder = builder.include("nest");
        }
        if self.with_egg {
            builder = builder.include("egg");
        }
        if self.with_location {
            builder = builder.include("location");
        }
        if self.with_node {
            builder = builder.include("node");
        }

        let res = self.app.request::<FractalList<Server>>(builder).await?;

        Ok(res.data.iter().map(|s| s.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetServer<'a> {
    app: &'a Application,
    id: i32,
    with_allocations: bool,
    with_owner: bool,
    with_subusers: bool,
    with_nest: bool,
    with_egg: bool,
    // with_variables: bool,
    with_location: bool,
    with_node: bool,
    // with_databases: bool,
}

impl<'a> GetServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            with_allocations: false,
            with_owner: false,
            with_subusers: false,
            with_nest: false,
            with_egg: false,
            with_location: false,
            with_node: false,
        }
    }

    /// Include the [`allocations`] bound to the server in the server [`relationships`].
    ///
    /// [`allocations`]: pteroxide_models::application::Allocation
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_allocations(mut self, value: bool) -> Self {
        self.with_allocations = value;

        self
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

    /// Include the [`nest`] the server's egg is part of in the server [`relationships`].
    ///
    /// [`nest`]: pteroxide_models::application::Nest
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_nest(mut self, value: bool) -> Self {
        self.with_nest = value;

        self
    }

    /// Include the [`egg`] the server uses in the server [`relationships`].
    ///
    /// [`egg`]: pteroxide_models::application::Egg
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_egg(mut self, value: bool) -> Self {
        self.with_egg = value;

        self
    }

    /// Include the [`location`] the server's node is part of in the server [`relationships`].
    ///
    /// [`location`]: pteroxide_models::application::Location
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_location(mut self, value: bool) -> Self {
        self.with_location = value;

        self
    }

    /// Include the [`node`] the server is part of in the server [`relationships`].
    ///
    /// [`node`]: pteroxide_models::application::Node
    /// [`relationships`]: pteroxide_models::application::ServerRelations
    pub fn with_node(mut self, value: bool) -> Self {
        self.with_node = value;

        self
    }

    /// Asynchronously executes the request and returns a [`Server`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is not found.
    pub async fn exec(&self) -> Result<Server, Error> {
        let mut builder = Builder::new(Route::GetServer { id: self.id }.into());

        if self.with_allocations {
            builder = builder.include("allocations");
        }
        if self.with_owner {
            builder = builder.include("user");
        }
        if self.with_subusers {
            builder = builder.include("subusers");
        }
        if self.with_nest {
            builder = builder.include("nest");
        }
        if self.with_egg {
            builder = builder.include("egg");
        }
        if self.with_location {
            builder = builder.include("location");
        }
        if self.with_node {
            builder = builder.include("node");
        }

        let res = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(res.attributes)
    }
}
