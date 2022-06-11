use pteroxide_models::{
    fractal::FractalList,
    client::file::File,
};
// use serde_json::json;

use crate::{
    client::Client,
    errors::Error,
    requests::RequestBuilder,
};

pub struct GetFiles<'a> {
    http: &'a Client,
    id: String,
    dir: String,
}

impl<'a> GetFiles<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            dir: String::from("/"),
        }
    }

    pub fn directory(mut self, dir: String) -> Self {
        self.dir = dir;

        self
    }

    pub async fn exec(self) -> Result<Vec<File>, Error> {
        match self.http.request::<FractalList<File>>(
            RequestBuilder::new(
                &format!("/api/client/servers/{}/files/list?directory={}", self.id, self.dir)
            )
        ).await {
            Ok(v) => Ok(v.unwrap()
                .data
                .iter()
                .map(|f| f.attributes.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }
}

// pub struct GetFileContents<'a> {
//     http: &'a Client,
//     id: String,
//     name: String,
// }

// impl<'a> GetFileContents<'a> {
//     #[doc(hidden)]
//     pub fn new(http: &'a Client, id: String) -> Self {
//         Self {
//             http,
//             id,
//             name: Default::default(),
//         }
//     }

//     pub fn name(mut self, name: String) -> Self {
//         self.name = name;

//         self
//     }
// }
