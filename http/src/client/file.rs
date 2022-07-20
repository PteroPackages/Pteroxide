use hyper::Request;
use hyper::body::{self, Body};
use serde_json::json;
use pteroxide_models::fractal::FractalData;
use pteroxide_models::{
    fractal::FractalList,
    client::file::{File, RenameFileData},
    global::UrlData,
};
use std::fs::{self, File as FsFile};
use std::io::{BufReader, Read};
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

pub struct RenameFile<'a> {
    http: &'a Client,
    id: String,
    root: String,
    files: Vec<RenameFileData>,
}

impl<'a> RenameFile<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            root: String::from("/"),
            files: Default::default(),
        }
    }

    pub fn root(mut self, dir: String) -> Self {
        if dir.starts_with("/home/container") {
            self.root = dir.strip_prefix("/home/container").unwrap().to_string();
        } else {
            self.root = dir;
        }

        self
    }

    pub fn set(mut self, from: String, to: String) -> Self {
        self.files.push(RenameFileData { from, to });

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.files.len() == 0 {
            return Err(Error::from("at least one file must be set"));
        }

        let req = Builder::new(&format!("/api/client/servers/{}/files/rename", self.id))
            .method("PUT")?
            .body(json!({
                "root": self.root,
                "files": self.files
            }));

        self.http.request::<()>(req).await?;

        Ok(())
    }
}

pub struct CopyFile<'a> {
    http: &'a Client,
    id: String,
    location: String,
}

impl<'a> CopyFile<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id, location: Default::default() }
    }

    pub fn location(mut self, path: String) -> Self {
        self.location = path;

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.location.is_empty() {
            return Err(Error::from("a location is required"))
        }

        let req = Builder::new(&format!("/api/client/servers/{}/files/copy", self.id))
            .method("POST")?
            .body(json!({
                "location": self.location
            }));

        self.http.request::<()>(req).await?;

        Ok(())
    }
}

pub struct WriteFile<'a> {
    http: &'a Client,
    id: String,
    name: String,
    data: Vec<u8>,
}

impl<'a> WriteFile<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            name: Default::default(),
            data: Default::default(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn content(mut self, content: String) -> Self {
        self.data = content.as_bytes().to_vec();

        self
    }

    pub fn from_file(mut self, file: FsFile) -> Self {
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut self.data).expect("could not read from file");

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::from("a file name is required"))
        }

        let mut req = Builder::new(
            &format!("/api/client/servers/{}/files/write?file={}", self.id, self.name)
        ).method("POST")?
            .content_type("text/plain");

        req.body = Body::from(self.data);

        self.http.request::<()>(req).await?;

        Ok(())
    }
}

pub struct CompressFiles<'a> {
    http: &'a Client,
    id: String,
    root: String,
    files: Vec<String>,
}

impl<'a> CompressFiles<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            root: String::from("/"),
            files: Default::default(),
        }
    }

    pub fn root(mut self, dir: String) -> Self {
        if dir.starts_with("/home/container") {
            self.root = dir.strip_prefix("/home/container").unwrap().to_string();
        } else {
            self.root = dir;
        }

        self
    }

    pub fn set(mut self, file: String) -> Self {
        self.files.push(file);

        self
    }

    pub async fn exec(self) -> Result<File, Error> {
        if self.files.is_empty() {
            return Err(Error::from("at least one file must be specified"));
        }

        let req = Builder::new(&format!("/api/client/servers/{}/files/compress", self.id))
            .method("POST")?
            .body(json!({
                "root": self.root,
                "files": self.files
            }));


        let res = self.http.request::<FractalData<File>>(req).await?;

        Ok(res.unwrap().attributes)
    }
}

pub struct DecompressFile<'a> {
    http: &'a Client,
    id: String,
    root: String,
    file: String,
}

impl<'a> DecompressFile<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            root: String::from("/"),
            file: Default::default(),
        }
    }

    pub fn root(mut self, path: String) -> Self {
        self.root = path;

        self
    }

    pub fn file(mut self, name: String) -> Self {
        self.file = name;

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.file.is_empty() {
            return Err(Error::from("a file name is required"));
        }

        let req = Builder::new(&format!("/api/client/servers/{}/files/decompress", self.id))
            .method("POST")?
            .body(json!({
                "root": self.root,
                "file": self.file
            }));

        self.http.request::<()>(req).await?;

        Ok(())
    }
}

pub struct DeleteFiles<'a> {
    http: &'a Client,
    id: String,
    root: String,
    files: Vec<String>,
}

impl<'a> DeleteFiles<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            root: String::from("/"),
            files: Default::default(),
        }
    }

    pub fn root(mut self, path: String) -> Self {
        self.root = path;

        self
    }

    pub fn set(mut self, file: String) -> Self {
        self.files.push(file);

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.files.is_empty() {
            return Err(Error::from("at least one file must be specified"));
        }

        let req = Builder::new(&format!("/api/client/servers/{}/files/delete", self.id))
            .method("POST")?
            .body(json!({
                "root": self.root,
                "files": self.files
            }));

        self.http.request::<()>(req).await?;

        Ok(())
    }
}

pub struct CreateFolder<'a> {
    http: &'a Client,
    id: String,
    root: String,
    name: String,
}

impl<'a> CreateFolder<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            root: String::from("/"),
            name: Default::default(),
        }
    }

    pub fn root(mut self, path: String) -> Self {
        self.root = path;

        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.name.is_empty() {
            return Err(Error::from("a folder name is required"));
        }

        let req = Builder::new(
            &format!("/api/client/servers/{}/files/create-folder", self.id)
        ).method("POST")?
            .body(json!({
                "root": self.root,
                "name": self.name
            }));

        self.http.request::<()>(req).await?;

        Ok(())
    }
}
