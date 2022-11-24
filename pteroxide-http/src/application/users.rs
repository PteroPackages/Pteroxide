use pteroxide_models::{application::User, fractal::FractalList};

use crate::{routing::application::Route, Application, Builder, Error};

#[derive(Debug)]
pub struct GetUsers<'a> {
    app: &'a Application,
    with_servers: bool,
}

impl<'a> GetUsers<'a> {
    pub const fn new(app: &'a Application) -> Self {
        Self { app, with_servers: false }
    }

    pub fn with_servers(mut self, value: bool) -> Self {
        self.with_servers = value;

        self
    }

    pub async fn exec(&self) -> Result<Vec<User>, Error> {
        let mut builder = Builder::default().route(Route::GetUsers);
        if self.with_servers {
            builder = builder.param("include", "servers");
        }

        let res = self
            .app
            .request::<FractalList<User>>(builder)
            .await?;

        Ok(res.data.iter().map(|u| u.attributes.clone()).collect())
    }
}
