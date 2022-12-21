use pteroxide_models::{application::Allocation, fractal::FractalList};
use serde::Serialize;

use crate::{error::*, routing::Application as Route, Application, Builder};

#[derive(Debug)]
pub struct GetAllocations<'a> {
    app: &'a Application,
    node: i32,
}

impl<'a> GetAllocations<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, node: i32) -> Self {
        Self { app, node }
    }

    /// Asynchronously executes the request and returns a list of [`Allocation`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Allocation>, Error> {
        let res = self
            .app
            .request::<FractalList<Allocation>>(Builder::new(
                Route::GetAllocations { node: self.node }.into(),
            ))
            .await?;

        Ok(res.data.iter().map(|a| a.attributes.clone()).collect())
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateAllocationsFields<'a> {
    pub ip: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    pub ports: Vec<String>,
}

#[derive(Debug)]
pub struct CreateAllocations<'a> {
    app: &'a Application,
    node: i32,
    fields: CreateAllocationsFields<'a>,
}

impl<'a> CreateAllocations<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application, node: i32) -> Self {
        Self {
            app,
            node,
            fields: Default::default(),
        }
    }

    /// Sets the IP address for the allocation.
    #[must_use = "an allocation is bound to an ip address"]
    pub fn ip(mut self, ip: &'a str) -> Self {
        self.fields.ip = ip;

        self
    }

    /// Sets an alias for the allocation so that it can be accessible via the alias instead of the
    /// port if configured properly. Defaults to [`None`] - unset.
    pub fn alias(mut self, alias: Option<&'a str>) -> Self {
        self.fields.alias = alias;

        self
    }

    /// Sets a port to include under the allocation IP.
    pub fn port(mut self, port: i32) -> Self {
        self.fields.ports.push(port.to_string());

        self
    }

    /// Sets a range of ports to include under the allocation IP.
    pub fn port_range(mut self, from: i32, to: i32) -> Self {
        self.fields.ports.push(format!("{from}-{to}"));

        self
    }

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(self) -> Result<(), Error> {
        let builder =
            Builder::new(Route::CreateAllocations { node: self.node }.into()).json(self.fields);

        self.app.request::<()>(builder).await
    }
}

#[derive(Debug)]
pub struct DeleteAllocation<'a> {
    app: &'a Application,
    node: i32,
    id: i32,
}

impl<'a> DeleteAllocation<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, node: i32, id: i32) -> Self {
        Self { app, node, id }
    }

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the allocation is not found.
    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(
                Route::DeleteAllocation {
                    node: self.node,
                    id: self.id,
                }
                .into(),
            ))
            .await
    }
}
