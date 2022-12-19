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
        let mut builder = Builder::new(Route::GetUsers.into());
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
        let mut builder = Builder::new(Route::GetUser { id: self.id }.into());
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

    /// Sets the username for the user.
    #[must_use = "a user must have a username"]
    pub fn username(mut self, username: &'a str) -> Self {
        self.fields.username = username;

        self
    }

    /// Sets the email for the user. Note that this must be unique to the user; duplicate emails
    /// are not allowed by the panel.
    #[must_use = "a user must have an email"]
    pub fn email(mut self, email: &'a str) -> Self {
        self.fields.email = email;

        self
    }

    /// Sets the external identifier for the user. This is in place for third-party applications
    /// and is not required (default is [`None`] - unset).
    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    /// Sets the first name of the user. This is required alongside the `last_name`.
    #[must_use = "a user must have a first name"]
    pub fn first_name(mut self, name: &'a str) -> Self {
        self.fields.first_name = name;

        self
    }

    /// Sets the last name of the user. This is required alongside the `first_name`.
    #[must_use = "a user must have a last name"]
    pub fn last_name(mut self, name: &'a str) -> Self {
        self.fields.last_name = name;

        self
    }

    /// Sets the root admin status of the user. By default this is [`None`] - false.
    pub fn root_admin(mut self, value: bool) -> Self {
        self.fields.root_admin = value;

        self
    }

    /// Sets the password for the user. By default this is [`None`] - unset. If no password is set,
    /// the user will be prompted to set one upon logging in via a web interface. If a password is
    /// set, it cannot be accessed or viewed anywhere - be careful how you manage your passwords.
    pub fn password(mut self, password: Option<&'a str>) -> Self {
        self.fields.password = password;

        self
    }

    /// Asynchronously executes the request and returns the new [`User`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(self) -> Result<User, Error> {
        let builder = Builder::new(Route::CreateUser.into()).json(self.fields);

        let res = self.app.request::<FractalItem<User>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct UpdateUserFields<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub external_id: Option<&'a str>,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub root_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<&'a str>,
}

pub struct UpdateUser<'a> {
    app: &'a Application,
    id: i32,
    fields: UpdateUserFields<'a>,
}

impl<'a> UpdateUser<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            fields: Default::default(),
        }
    }

    /// Sets the username for the user, otherwise defaults to the existing one.
    pub fn username(mut self, username: &'a str) -> Self {
        self.fields.username = username;

        self
    }

    /// Sets the email for the user, otherwise defaults to the existing one.
    pub fn email(mut self, email: &'a str) -> Self {
        self.fields.email = email;

        self
    }

    /// Sets the external identifier for the user, otherwise defaults to the existing one. Make
    /// sure to also update any third-party services using this external identifier if changed.
    pub fn external_id(mut self, id: Option<&'a str>) -> Self {
        self.fields.external_id = id;

        self
    }

    /// Sets the first name for the user, otherwise defaults to the existing one.
    pub fn first_name(mut self, name: &'a str) -> Self {
        self.fields.first_name = name;

        self
    }

    /// Sets the last name for the user, otherwise defaults to the existing one.
    pub fn last_name(mut self, name: &'a str) -> Self {
        self.fields.last_name = name;

        self
    }

    /// Sets the root admin status of the user, otherwise defaults to the current status.
    pub fn root_admin(mut self, value: bool) -> Self {
        self.fields.root_admin = Some(value);

        self
    }

    /// Sets the password for the user. This will always override the existing password as there
    /// is no way to get the current one from the panel.
    pub fn password(mut self, password: Option<&'a str>) -> Self {
        self.fields.password = password;

        self
    }

    /// Asynchronously executes the request and returns the updated [`User`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(mut self) -> Result<User, Error> {
        let user = GetUser::new(self.app, self.id).exec().await?;

        if self.fields.username.is_empty() {
            self.fields.username = &user.username
        }
        if self.fields.email.is_empty() {
            self.fields.email = &user.email
        }
        if self.fields.first_name.is_empty() {
            self.fields.first_name = &user.first_name
        }
        if self.fields.last_name.is_empty() {
            self.fields.last_name = &user.last_name
        }
        if self.fields.root_admin.is_none() {
            self.fields.root_admin = Some(user.root_admin)
        }

        self.fields.external_id = self.fields.external_id.or(user.external_id.as_deref());

        let builder = Builder::new(Route::UpdateUser { id: self.id }.into()).json(self.fields);

        let new = self.app.request::<FractalItem<User>>(builder).await?;

        Ok(new.attributes)
    }
}

pub struct DeleteUser<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> DeleteUser<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the user is not found.
    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(Route::DeleteUser { id: self.id }.into()))
            .await
    }
}
