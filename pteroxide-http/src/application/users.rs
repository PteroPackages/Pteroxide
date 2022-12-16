use pteroxide_models::{
    application::User,
    fractal::{FractalItem, FractalList},
};
use serde::Serialize;

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetUsers<'a> {
    app: &'a Application,
    with_servers: bool,
}

impl<'a> GetUsers<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self {
            app,
            with_servers: false,
        }
    }

    /// Include the [`Server`]s the user has access to in the user [`relationships`].
    ///
    /// [`relationships`]: pteroxide_models::application::UserRelations
    /// [`Server`]: pteroxide_models::application::Server
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`User`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<User>, Error> {
        let mut builder = Builder::default().route(Route::GetUsers.into());
        if self.with_servers {
            builder = builder.param("include", "servers");
        }

        let res = self.app.request::<FractalList<User>>(builder).await?;

        Ok(res.data.iter().map(|u| u.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetUser<'a> {
    app: &'a Application,
    id: i32,
    with_servers: bool,
}

impl<'a> GetUser<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            with_servers: false,
        }
    }

    /// Include the [`Server`]s the user has access to in the user [`relationships`].
    ///
    /// [`relationships`]: pteroxide_models::application::UserRelations
    /// [`Server`]: pteroxide_models::application::Server
    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    /// Asynchronously executes the request and returns a list of [`User`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the user is not found.
    pub async fn exec(&self) -> Result<User, Error> {
        let mut builder = Builder::default().route(Route::GetUser { id: self.id }.into());
        if self.with_servers {
            builder = builder.param("include", "servers");
        }

        let res = self.app.request::<FractalItem<User>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateUserFields<'a> {
    pub username: &'a str,
    pub email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<&'a str>,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub root_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<&'a str>,
}

#[derive(Debug)]
pub struct CreateUser<'a> {
    app: &'a Application,
    fields: CreateUserFields<'a>,
}

impl<'a> CreateUser<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application) -> Self {
        Self {
            app,
            fields: Default::default(),
        }
    }

    pub fn username(mut self, username: &'a str) -> Self {
        self.fields.username = username;

        self
    }

    pub fn email(mut self, email: &'a str) -> Self {
        self.fields.email = email;

        self
    }

    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    pub fn first_name(mut self, name: &'a str) -> Self {
        self.fields.first_name = name;

        self
    }

    pub fn last_name(mut self, name: &'a str) -> Self {
        self.fields.last_name = name;

        self
    }

    pub fn root_admin(mut self, value: bool) -> Self {
        self.fields.root_admin = value;

        self
    }

    pub fn password(mut self, password: Option<&'a str>) -> Self {
        self.fields.password = password;

        self
    }

    pub async fn exec(self) -> Result<User, Error> {
        let builder = Builder::default()
            .route(Route::CreateUser.into())
            .json(self.fields);

        let res = self.app.request::<FractalItem<User>>(builder).await?;

        Ok(res.attributes)
    }
}
