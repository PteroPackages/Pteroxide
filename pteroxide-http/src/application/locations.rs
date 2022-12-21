use pteroxide_models::{
    application::Location,
    fractal::{FractalItem, FractalList},
};
use serde::Serialize;

use crate::{routing::Application as Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetLocations<'a> {
    app: &'a Application,
}

impl<'a> GetLocations<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    /// Asynchronously executes the request and returns a list of [`Location`] objects.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails.
    pub async fn exec(&self) -> Result<Vec<Location>, Error> {
        let res = self
            .app
            .request::<FractalList<Location>>(Builder::new(Route::GetLocations.into()))
            .await?;

        Ok(res.data.iter().map(|l| l.attributes.clone()).collect())
    }
}

#[derive(Debug)]
pub struct GetLocation<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> GetLocation<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    pub async fn exec(&self) -> Result<Location, Error> {
        let res = self
            .app
            .request::<FractalItem<Location>>(Builder::new(
                Route::GetLocation { id: self.id }.into(),
            ))
            .await?;

        Ok(res.attributes)
    }
}

#[derive(Debug, Default, Serialize)]
struct CreateLocationFields<'a> {
    pub short: &'a str,
    pub long: &'a str,
}

#[derive(Debug)]
pub struct CreateLocation<'a> {
    app: &'a Application,
    fields: CreateLocationFields<'a>,
}

impl<'a> CreateLocation<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application) -> Self {
        Self {
            app,
            fields: Default::default(),
        }
    }

    /// Sets the short code for the location. This is generally based on the
    /// [Alpha-2 Country Codes](https://www.iban.com/country-codes).
    #[must_use = "a location must have a short code"]
    pub fn short(mut self, short: &'a str) -> Self {
        self.fields.short = short;

        self
    }

    /// Sets the long code for the location.
    #[must_use = "a location must have a long code"]
    pub fn long(mut self, long: &'a str) -> Self {
        self.fields.long = long;

        self
    }

    /// Asynchronously executes the request and returns the new [`Location`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(self) -> Result<Location, Error> {
        let builder = Builder::new(Route::CreateLocation.into()).json(self.fields);
        let res = self.app.request::<FractalItem<Location>>(builder).await?;

        Ok(res.attributes)
    }
}

#[derive(Debug)]
pub struct UpdateLocation<'a> {
    app: &'a Application,
    id: i32,
    fields: CreateLocationFields<'a>,
}

impl<'a> UpdateLocation<'a> {
    #[doc(hidden)]
    pub fn new(app: &'a Application, id: i32) -> Self {
        Self {
            app,
            id,
            fields: Default::default(),
        }
    }

    /// Sets the short code for the location, otherwise defaults to the current on. This is
    /// generally based on the [Alpha-2 Country Codes](https://www.iban.com/country-codes).
    pub fn short(mut self, short: &'a str) -> Self {
        self.fields.short = short;

        self
    }

    /// Sets the long code for the location, otherwise defaults to the current one.
    pub fn long(mut self, long: &'a str) -> Self {
        self.fields.long = long;

        self
    }

    /// Asynchronously executes the request and returns the updated [`Location`] object.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or a field does not satisfy a validation rule.
    pub async fn exec(mut self) -> Result<Location, Error> {
        let loc = GetLocation::new(self.app, self.id).exec().await?;

        if self.fields.short.is_empty() {
            self.fields.short = loc.short.as_str();
        }
        if self.fields.long.is_empty() {
            self.fields.long = loc.long.as_str();
        }

        let builder = Builder::new(Route::UpdateLocation { id: self.id }.into()).json(self.fields);
        let new = self.app.request::<FractalItem<Location>>(builder).await?;

        Ok(new.attributes)
    }
}

#[derive(Debug)]
pub struct DeleteLocation<'a> {
    app: &'a Application,
    id: i32,
}

impl<'a> DeleteLocation<'a> {
    #[doc(hidden)]
    pub const fn new(app: &'a Application, id: i32) -> Self {
        Self { app, id }
    }

    /// Asynchronously executes the request and returns nothing.
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the location is not found.
    pub async fn exec(&self) -> Result<(), Error> {
        self.app
            .request::<()>(Builder::new(Route::DeleteLocation { id: self.id }.into()))
            .await
    }
}
