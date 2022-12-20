use pteroxide_models::{
    application::Node,
    fractal::{FractalItem, FractalList},
};
use serde::Serialize;

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetNodes<'a> {
    app: &'a Application,
}

impl<'a> GetNodes<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    pub async fn exec(&self) -> Result<Vec<Node>, Error> {
        let res = self
            .app
            .request::<FractalList<Node>>(Builder::new(Route::GetNodes.into()))
            .await?;

        Ok(res.data.iter().map(|n| n.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetNode<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> GetNode<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<Node, Error> {
        let res = self
            .app
            .request::<FractalItem<Node>>(Builder::new(Route::GetNode { id: self.id }.into()))
            .await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateNodeFields<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub location_id: i32,
    pub public: bool,
    pub fqdn: &'a str,
    pub scheme: &'a str,
    pub behind_proxy: bool,
    pub memory: i64,
    pub memory_overallocate: i64,
    pub disk: i64,
    pub disk_overallocate: i64,
    pub daemon_base: &'a str,
    pub daemon_sftp: i64,
    pub daemon_listen: i64,
    pub maintenance_mode: bool,
    pub upload_size: i64,
}

#[derive(Debug)]
pub struct CreateNode<'a> {
    app: &'a Application,
    fields: CreateNodeFields<'a>,
}

impl<'a> CreateNode<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application) -> Self {
        let fields = CreateNodeFields {
            daemon_base: "/var/lib/pterodactyl/volumes",
            daemon_sftp: 2022,
            daemon_listen: 8080,
            ..Default::default()
        };

        Self { app, fields }
    }

    /// Sets the name for the node.
    #[must_use = "a node must have a name"]
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields.name = name;

        self
    }

    /// Sets the description for the node, taking an [`Option`] which defaults to [`None`] to
    /// leave it unset.
    pub fn description(mut self, desc: Option<&'a str>) -> Self {
        self.fields.description = desc;

        self
    }

    /// Sets the location ID for the node.
    #[must_use = "a node is bound to a location"]
    pub fn location(mut self, id: i32) -> Self {
        self.fields.location_id = id;

        self
    }

    /// Whether the node should be publicly accessible. See
    /// [Pterodactyl documentation](https://pterodactyl.io) for more information.
    #[must_use]
    pub fn public(mut self, value: bool) -> Self {
        self.fields.public = value;

        self
    }

    /// Sets the Fully Qualified Domain Name (FQDN) for the node.
    #[must_use = "a node is bound to a fqdn"]
    pub fn fqdn(mut self, fqdn: &'a str) -> Self {
        self.fields.fqdn = fqdn;

        self
    }

    /// Sets the HTTP scheme for the node to use.
    #[must_use = "a node must have a http scheme"]
    pub fn scheme(mut self, scheme: &'a str) -> Self {
        self.fields.scheme = scheme;

        self
    }

    /// Whether the node is (or should be) behind a proxy.
    #[must_use]
    pub fn behind_proxy(mut self, value: bool) -> Self {
        self.fields.behind_proxy = value;

        self
    }

    /// Sets the memory limit and memory overallocation limit for the node.
    #[must_use = "a node must have a memory limit"]
    pub fn memory(mut self, limit: i64, overallocate: i64) -> Self {
        self.fields.memory = limit;
        self.fields.memory_overallocate = overallocate;

        self
    }

    /// Sets the disk limit and disk overallocation limit for the node.
    #[must_use = "a node must have a disk limit"]
    pub fn disk(mut self, limit: i64, overallocate: i64) -> Self {
        self.fields.disk = limit;
        self.fields.disk_overallocate = overallocate;

        self
    }

    /// Sets the daemon base for the node. Defaults to `/var/lib/pterodactyl/volumes`.
    pub fn daemon_base(mut self, base: &'a str) -> Self {
        self.fields.daemon_base = base;

        self
    }

    /// Sets the daemon SFTP port. Defaults to `2022`.
    pub fn daemon_sftp(mut self, port: i64) -> Self {
        self.fields.daemon_sftp = port;

        self
    }

    /// Sets the daemon listen port. Defaults to `8080`.
    pub fn daemon_listen(mut self, port: i64) -> Self {
        self.fields.daemon_listen = port;

        self
    }

    /// Whether the node should be set to maintenance mode on creation.
    #[must_use]
    pub fn maintenance_mode(mut self, value: bool) -> Self {
        self.fields.maintenance_mode = value;

        self
    }

    /// Sets the upload size limit for the node.
    #[must_use = "a node must have an upload size limit"]
    pub fn upload_size(mut self, limit: i64) -> Self {
        self.fields.upload_size = limit;

        self
    }

    pub async fn exec(self) -> Result<Node, Error> {
        let builder = Builder::new(Route::CreateNode.into()).json(self.fields);

        let res = self.app.request::<FractalItem<Node>>(builder).await?;

        Ok(res.attributes)
    }
}
