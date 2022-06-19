use hyper::Request;
use hyper::body::{self, Body};
use pteroxide_models::fractal::FractalData;
use pteroxide_models::{
    fractal::FractalList,
    client::file::File,
    global::UrlData,
};
use std::fs;
use std::path::Path;

use crate::{
    client::Client,
    errors::Error,
    request::Builder,
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
        let res = self.http.request::<FractalList<File>>(
            Builder::new(
                &format!("/api/client/servers/{}/files/list?directory={}", self.id, self.dir)
            )
        ).await?;

        Ok(res.unwrap()
            .data
            .iter()
            .map(|f| f.attributes.clone())
            .collect())
    }
}

pub struct GetFileContents<'a> {
    http: &'a Client,
    id: String,
    name: String,
}

impl<'a> GetFileContents<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            name: Default::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub async fn exec(self) -> Result<String, Error> {
        if self.name.len() == 0 {
            return Err(Error::from("file name is required"));
        }

        let req = Builder::new(
            &format!("/api/client/servers/{}/files/contents?file={}", self.id, self.name)
        ).content_type("text/plain");
        let res = self.http.request_raw(req).await?;

        Ok(res.unwrap())
    }
}

pub struct Downloader<'a> {
    http: &'a Client,
    path: String,
    url: String,
}

impl<'a> Downloader<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, url: String) -> Self {
        Self {
            http,
            path: Default::default(),
            url,
        }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn path(mut self, path: String) -> Result<Self, Error> {
        let p = Path::new(&path);
        if p.exists() {
            if p.is_file() {
                return Err(Error::from("file already exists at this path"));
            }
        }
        self.path = path;

        Ok(self)
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.path.is_empty() {
            return Err(Error::from("no file path set"));
        }

        fs::File::create(self.path.clone()).expect("failed to create file");
        let req = Request::builder()
            .uri(self.url)
            .body(Body::empty())
            .unwrap();

        let res = self.http.http.request(req).await?;
        if res.status().as_u16() != 200 {
            return Err(Error::from("failed to download the file"));
        }
        let data = body::to_bytes(res).await?;

        fs::write(self.path.clone(), data.to_vec())
            .expect("failed to write data to file");

        Ok(())
    }
}

pub struct DownloadFile<'a> {
    http: &'a Client,
    id: String,
    name: String,
}

impl<'a> DownloadFile<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            name: Default::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name.clone();

        self
    }

    pub async fn exec(self) -> Result<Downloader<'a>, Error> {
        if self.name.is_empty() {
            return Err(Error::from("file name is required"));
        }

        let req = Builder::new(
            &format!("/api/client/servers/{}/files/download?file={}", self.id, self.name)
        );
        let data = self.http.request::<FractalData<UrlData>>(req).await?;
        let dl = Downloader::new(self.http, data.unwrap().attributes.url);

        Ok(dl)
    }
}
