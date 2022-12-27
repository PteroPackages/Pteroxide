use pteroxide_models::{application::Server, fractal::FractalItem, FeatureLimits, Limits};
use serde::Serialize;

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug, Serialize)]
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
            fields: UpdateServerBuildFields {
                allocation_id: 0,
                oom_disabled: false,
                limits: Limits {
                    memory: 0,
                    disk: 0,
                    swap: 0,
                    cpu: 0,
                    io: None,
                    oom_disabled: None,
                    threads: None,
                },
                feature_limits: FeatureLimits {
                    allocations: 0,
                    backups: 0,
                    databases: 0,
                },
                add_allocations: Vec::new(),
                remove_allocations: Vec::new(),
            },
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
