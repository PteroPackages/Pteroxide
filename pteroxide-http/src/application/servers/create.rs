use pteroxide_models::{application::Server, fractal::FractalItem, FeatureLimits, Limits};
use serde::Serialize;
use std::collections::HashMap;

use crate::routing::Application as Route;
use crate::{Application, Builder, Error, Value};

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

#[derive(Debug, Default, Serialize)]
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
            fields: Default::default(),
        }
    }

    /// Sets the name of the server.
    #[must_use = "a server must have a name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields.name = name;

        self
    }

    /// Sets the description for a server, taking an [`Option`] which defaults to [`None`] to
    /// leave it unset.
    pub fn description(mut self, desc: Option<&'a str>) -> Self {
        self.fields.description = desc;

        self
    }

    /// Sets the external identifier for the server. Default is [`None`] - unset.
    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    /// Sets the owner for the server.
    #[must_use = "a server must have an owner"]
    pub fn owner(mut self, id: i32) -> Self {
        self.fields.user = id;

        self
    }

    /// Sets the egg to use for the server.
    #[must_use = "a server is bound to an egg"]
    pub fn egg(mut self, id: i32) -> Self {
        self.fields.egg = id;

        self
    }

    /// Sets the docker image for the server.
    #[must_use = "a docker image is required for a server"]
    pub fn docker_image(mut self, image: &'a str) -> Self {
        self.fields.docker_image = image;

        self
    }

    /// Sets the startup command for the server.
    #[must_use = "a startup command is required for a server"]
    pub fn startup(mut self, command: &'a str) -> Self {
        self.fields.startup = command;

        self
    }

    /// Sets an environment variable for the server. This is required with certain eggs.
    pub fn env_variable(mut self, name: &'a str, value: Value) -> Self {
        self.fields.environment.insert(name, value);

        self
    }

    /// Whether the server should skip the egg installation script during the installation process.
    /// Defaults to `false`.
    pub fn skip_scripts(mut self, value: bool) -> Self {
        self.fields.skip_scripts = value;

        self
    }

    /// Sets the status of the OOM killer for the server. Default is `false` (enabled).
    pub fn oom_disabled(mut self, value: bool) -> Self {
        self.fields.oom_disabled = value;

        self
    }

    /// Sets the limits for the server, including the memory, disk, and other docker configuration
    /// options.
    #[must_use = "limits are required for a server"]
    pub fn limits(mut self, data: Limits) -> Self {
        self.fields.limits = data;

        self
    }

    /// Sets the feature limits for the server.
    #[must_use = "feature limits are required for a server"]
    pub fn feature_limits(mut self, data: FeatureLimits) -> Self {
        self.fields.feature_limits = data;

        self
    }

    /// Sets the allocation data for the server, including the default allocation and any
    /// additional allocations. Defaults to [`None`] - unset. You must set either the allocation
    /// data or the deployment data for the server to be created.
    pub fn allocation(mut self, default: i32, additional: &'a [i32]) -> Self {
        self.fields.allocation = Some(AllocationData {
            default,
            additional: additional.to_vec(),
        });

        self
    }

    /// Sets the deployment options for the server. This will override the allocation data if set.
    /// Defaults to [`None`] - unset.
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

    /// Whether the server should start once the install process has been completed. Defaults to
    /// `false`.
    pub fn start_on_completion(mut self, value: bool) -> Self {
        self.fields.start_on_completion = value;

        self
    }

    /// Asynchronously executes the request and returns the new [`Server`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(mut self) -> Result<Server, Error> {
        if self.fields.allocation.is_some() && self.fields.deploy.is_some() {
            self.fields.allocation = None;
        }

        let builder = Builder::new(Route::CreateServer.into()).json(self.fields);
        let res = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(res.attributes)
    }
}
