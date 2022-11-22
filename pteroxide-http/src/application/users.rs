use pteroxide_models::{application::User, FractalList};

use crate::{routing::application::Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetUsers<'a> {
    app: &'a Application,
}

impl<'a> GetUsers<'a> {
    pub const fn new(app: &'a Application) -> Self {
        Self { app }
    }

    pub async fn exec(&self) -> Result<Vec<User>, Error> {
        let res = self
            .app
            .request::<FractalList<User>>(Builder::default().route(Route::GetUsers))
            .await?;

        Ok(res.data.iter().map(|u| u.attributes.clone()).collect())
    }
}
