use crate::{Builder, Error, Http, routing::application::Route};

use pteroxide_models::{application::User, FractalList};

#[derive(Debug)]
pub struct GetUsers {
    http: Http,
}

impl GetUsers {
    pub const fn new(http: Http) -> Self {
        Self { http }
    }

    pub async fn exec(self) -> Result<Vec<User>, Error> {
        let res = self.http.request::<FractalList<User>>(
            Builder::default().route(Route::ListUsers)
        ).await?;

        Ok(res.data
            .iter()
            .map(|u| u.attributes.clone())
            .collect())
    }
}
