use pteroxide_models::fractal::FractalList;
use serde::Deserialize;
use std::marker::PhantomData;

use crate::{routing::Route, Application, Builder, Error};

pub struct Response<'a, T> {
    phantom: PhantomData<T>,
    http: &'a Application,
    route: Route,
    params: Vec<(String, String)>,
    page: u32,
    per_page: u32,
}

impl<'a, T> Response<'a, T> {
    pub fn new(http: &'a Application, route: Route) -> Self {
        Self {
            phantom: PhantomData,
            http,
            route,
            params: Default::default(),
            page: 1,
            per_page: 50,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;

        self
    }

    pub fn per_page(mut self, per_page: u32) -> Self {
        self.per_page = per_page;

        self
    }

    pub async fn get_page(self) -> Result<Vec<T>, Error>
    where
        for<'de> T: Deserialize<'de> + Clone,
    {
        let mut builder = Builder::new(self.route)
            .param("page", &self.page.to_string())
            .param("per_page", &self.per_page.to_string());

        for (key, value) in self.params {
            builder = builder.param(&key, &value);
        }

        let res = self.http.request::<FractalList<T>>(builder).await?;

        Ok(res.data.iter().map(|v| v.attributes.clone()).collect())
    }

    pub async fn get_next_page(mut self) -> Result<Vec<T>, Error>
    where
        for<'de> T: Deserialize<'de> + Clone,
    {
        self.page += 1;

        self.get_page().await
    }

    // pub async fn collect(mut self) -> Result<Vec<T>, Error>
    // where
    //     for<'de> T: Deserialize<'de> + Clone,
    // {
    //     let mut res = self.get_page().await?;

    //     loop {
    //         self.page += 1;

    //         if let Ok(value) = self.get_page().await {
    //             if value.len() == 0 {
    //                 break;
    //             }
    //             res.extend(value);
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(res)
    // }
}
