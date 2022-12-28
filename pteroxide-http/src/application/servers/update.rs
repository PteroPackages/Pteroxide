use pteroxide_models::{application::Server, fractal::FractalItem, FeatureLimits, Limits};
use serde::Serialize;

use crate::{routing::Application as Route, Application, Builder, Error};
use super::GetServer;

#[derive(Debug, Default, Serialize)]
struct UpdateServerBuildFields {
    pub allocation_id: i32,
    pub oom_disabled: bool,
    pub limits: Limits,
    pub feature_limits: FeatureLimits,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub add_allocations: Vec<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub remove_allocations: Vec<i32>,
}

#[derive(Debug)]
pub struct UpdateServerBuild<'a> {
    app: &'a Application,
    id: i32,
    fields: UpdateServerBuildFields,
}

impl<'a> UpdateServerBuild<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            fields: Default::default(),
        }
    }

    /// Sets the allocation ID for the server.
    #[must_use]
    pub fn allocation_id(mut self, id: i32) -> Self {
        self.fields.allocation_id = id;

        self
    }

    /// Sets the status of the OOM killer for the server.
    #[must_use]
    pub fn oom_disabled(mut self, value: bool) -> Self {
        self.fields.oom_disabled = value;

        self
    }

    /// Sets the limits for the server, including the memory, disk, and other docker configuration
    /// options.
    #[must_use]
    pub fn limits(mut self, limits: Limits) -> Self {
        self.fields.limits = limits;

        self
    }

    /// Sets the feature limits for the server.
    #[must_use]
    pub fn feature_limits(mut self, feature_limits: FeatureLimits) -> Self {
        self.fields.feature_limits = feature_limits;

        self
    }

    /// Sets a specified allocation to be added to the server.
    pub fn add_allocation(mut self, id: i32) -> Self {
        self.fields.add_allocations.push(id);

        self
    }

    /// Sets a list of allocations to be added to the server.
    pub fn add_allocations(mut self, ids: Vec<i32>) -> Self {
        self.fields.add_allocations = ids;

        self
    }

    /// Sets a specified allocation to be removed from the server.
    pub fn remove_allocation(mut self, id: i32) -> Self {
        self.fields.remove_allocations.push(id);

        self
    }

    /// Sets a list of allocations to be removed from the server.
    pub fn remove_allocations(mut self, ids: Vec<i32>) -> Self {
        self.fields.remove_allocations = ids;

        self
    }

    /// Asynchronously executes the request and returns the updated [`Server`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(self) -> Result<Server, Error> {
        let builder =
            Builder::new(Route::UpdateServerBuild { id: self.id }.into()).json(self.fields);
        let res = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct UpdateServerDetailsFields<'a> {
    pub external_id: Option<&'a str>,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub user: i32,
}

#[derive(Debug)]
pub struct UpdateServerDetails<'a> {
    app: &'a Application,
    id: i32,
    fields: UpdateServerDetailsFields<'a>,
}

impl<'a> UpdateServerDetails<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id, fields: Default::default() }
    }

    /// Sets the external identifier for the server, otherwise defaults to the existing one. Make
    /// sure to also update any third-party services using this external identifier if changed.
    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    /// Sets the name for the server, otherwise defaults to the current one.
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields.name = name;

        self
    }

    /// Sets the description for the server, taking an [`Option`] which defaults to [`None`] to
    /// leave it unset.
    pub fn description(mut self, desc: Option<&'a str>) -> Self {
        self.fields.description = desc;

        self
    }

    /// Sets the owner of the server, otherwise defaults to the current one.
    pub fn owner(mut self, id: i32) -> Self {
        self.fields.user = id;

        self
    }

    pub async fn exec(mut self) -> Result<Server, Error> {
        let server = GetServer::new(self.app, self.id).exec().await?;

        self.fields.external_id = self.fields.external_id.or(server.external_id.as_deref());
        self.fields.description = self.fields.description.or(server.description.as_deref());

        if self.fields.name.is_empty() {
            self.fields.name = server.name.as_str();
        }
        if self.fields.user == 0 {
            self.fields.user = server.user;
        }

        let builder = Builder::new(Route::UpdateServerDetails { id: self.id }.into()).json(self.fields);
        let new = self.app.request::<FractalItem<Server>>(builder).await?;

        Ok(new.attributes)
    }
}
