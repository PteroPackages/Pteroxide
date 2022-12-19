use pteroxide_models::{
    application::Server,
    fractal::{FractalItem, FractalList},
    FeatureLimits, Limits,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::{routing::Application as Route, Application, Builder, Error, Value};

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

#[derive(Debug)]
pub struct GetServer<'a> {
    app: &'a Application,
    id: i32,
    with_owner: bool,
    with_subusers: bool,
}

impl<'a> GetServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
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

    /// Asynchronously executes the request and returns a [`Server`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is not found.
    pub async fn exec(&self) -> Result<Server, Error> {
        let mut builder = Builder::new(Route::GetServer { id: self.id }.into());

        if self.with_owner {
            builder = builder.include("user");
        }
        if self.with_subusers {
            builder = builder.include("subusers");
        }

        let res = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct AllocationData {
    default: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    additional: Vec<i32>,
}

#[derive(Debug, Default, Serialize)]
struct DeployData<'a> {
    locations: Vec<i32>,
    port_range: Vec<&'a str>,
    dedicated_ip: bool,
}

#[derive(Debug, Serialize)]
struct CreateServerFields<'a> {
    name: &'a str,
    description: Option<&'a str>,
    external_id: Option<&'a str>,
    user: i32,
    egg: i32,
    docker_image: &'a str,
    startup: &'a str,
    environment: HashMap<&'a str, Value>,
    skip_scripts: bool,
    oom_disabled: bool,
    limits: Limits,
    feature_limits: FeatureLimits,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation: Option<AllocationData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deploy: Option<DeployData<'a>>,
    start_on_completion: bool,
}

#[derive(Debug)]
pub struct CreateServer<'a> {
    app: &'a Application,
    fields: CreateServerFields<'a>,
}

impl<'a> CreateServer<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application) -> Self {
        Self {
            app,
            fields: CreateServerFields {
                name: "",
                description: None,
                external_id: None,
                user: 0,
                egg: 0,
                docker_image: "",
                startup: "",
                environment: HashMap::new(),
                skip_scripts: false,
                oom_disabled: false,
                limits: Limits {
                    memory: 0,
                    swap: 0,
                    disk: 0,
                    io: None,
                    cpu: 0,
                    threads: None,
                    oom_disabled: None,
                },
                feature_limits: FeatureLimits {
                    allocations: 0,
                    backups: 0,
                    databases: 0,
                },
                allocation: None,
                deploy: None,
                start_on_completion: false,
            },
        }
    }

    pub fn name(mut self, name: &'a str) -> Self {
        self.fields.name = name;

        self
    }

    pub fn description(mut self, desc: Option<&'a str>) -> Self {
        self.fields.description = desc;

        self
    }

    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    pub fn owner(mut self, id: i32) -> Self {
        self.fields.user = id;

        self
    }

    pub fn egg(mut self, id: i32) -> Self {
        self.fields.egg = id;

        self
    }

    pub fn docker_image(mut self, image: &'a str) -> Self {
        self.fields.docker_image = image;

        self
    }

    pub fn startup(mut self, command: &'a str) -> Self {
        self.fields.startup = command;

        self
    }

    pub fn env_variable(mut self, name: &'a str, value: Value) -> Self {
        self.fields.environment.insert(name, value);

        self
    }

    pub fn skip_scripts(mut self, value: bool) -> Self {
        self.fields.skip_scripts = value;

        self
    }

    pub fn oom_disabled(mut self, value: bool) -> Self {
        self.fields.oom_disabled = value;

        self
    }

    pub fn limits(mut self, data: Limits) -> Self {
        self.fields.limits = data;

        self
    }

    pub fn feature_limits(mut self, data: FeatureLimits) -> Self {
        self.fields.feature_limits = data;

        self
    }

    pub fn allocation(mut self, default: i32, additional: &'a [i32]) -> Self {
        self.fields.allocation = Some(AllocationData {
            default,
            additional: additional.to_vec(),
        });

        self
    }

    pub fn deploy(
        mut self,
        locations: &'a [i32],
        port_range: &'a [&'a str],
        dedicated_ip: bool,
    ) -> Self {
        self.fields.deploy = Some(DeployData {
            locations: locations.to_vec(),
            port_range: port_range.to_vec(),
            dedicated_ip,
        });

        self
    }

    pub fn start_on_completion(mut self, value: bool) -> Self {
        self.fields.start_on_completion = value;

        self
    }

    pub async fn exec(mut self) -> Result<Server, Error> {
        if self.fields.allocation.is_some() && self.fields.deploy.is_some() {
            self.fields.allocation = None;
        }

        let builder = Builder::new(Route::CreateServer.into()).json(self.fields);

        let res = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug)]
pub struct SuspendServer<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> SuspendServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(Route::SuspendServer { id: self.id }.into()))
            .await
    }
}

#[derive(Debug)]
pub struct UnsuspendServer<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> UnsuspendServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(Route::UnsuspendServer { id: self.id }.into()))
            .await
    }
}

#[derive(Debug)]
pub struct ReinstallServer<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> ReinstallServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(Route::ReinstallServer { id: self.id }.into()))
            .await
    }
}

#[derive(Debug)]
pub struct DeleteServer<'a> {
    app: &'a Application,
    id: i32,
    force: bool,
}

impl<'a> DeleteServer<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            force: false,
        }
    }

    pub fn force(mut self, value: bool) -> Self {
        self.force = value;

        self
    }

    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(
                Route::DeleteServer {
                    id: self.id,
                    force: self.force,
                }
                .into(),
            ))
            .await
    }
}
