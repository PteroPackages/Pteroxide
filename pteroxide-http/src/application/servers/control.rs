use crate::{routing::Application as Route, Application, Builder, Error};

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

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is in conflict.
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

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is in conflict.
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

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is in conflict.
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

    /// Whether the server should be deleted by force instead of a gracious deletion. Defaults to
    /// `false`.
    pub fn force(mut self, value: bool) -> Self {
        self.force = value;

        self
    }

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or the server is not found.
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
