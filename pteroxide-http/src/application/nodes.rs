use pteroxide_models::{
    application::Node,
    fractal::{FractalItem, FractalList},
};

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
