use pteroxide_models::{
    fractal::{FractalList, FractalData},
    client::database::Database,
};
use serde_json::json;

use crate::{
    client::Client,
    errors::Error,
    request::Builder,
};

pub struct GetDatabases<'a> {
    http: &'a Client,
    id: String,
}

impl<'a> GetDatabases<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self { http, id }
    }

    pub async fn exec(self) -> Result<Vec<Database>, Error> {
        let res = self.http.request::<FractalList<Database>>(
            Builder::new(&format!("/api/client/servers/{}/databases", self.id))
        ).await?;

        Ok(res.unwrap()
            .data
            .iter()
            .map(|d| d.attributes.clone())
            .collect())
    }
}

pub struct CreateDatabase<'a> {
    http: &'a Client,
    id: String,
    database: String,
    remote: String,
}

impl<'a> CreateDatabase<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            database: Default::default(),
            remote: Default::default(),
        }
    }

    pub fn database(mut self, db: String) -> Self {
        self.database = db;

        self
    }

    pub fn remote(mut self, remote: String) -> Self {
        self.remote = remote;

        self
    }

    pub async fn exec(self) -> Result<Database, Error> {
        if self.database.len() == 0 || self.remote.len() == 0 {
            return Err(Error::from("database and remote fields are required"));
        }

        let req = Builder::new(&format!("/api/client/servers/{}/databases", self.id))
            .method("POST")?
            .body(json!({
                "database": self.database,
                "remote": self.remote
            }));

        let res = self.http.request::<FractalData<Database>>(req).await?;

        Ok(res.unwrap().attributes)
    }
}

pub struct RotateDatabasePassword<'a> {
    http: &'a Client,
    id: String,
    uid: String,
}

impl<'a> RotateDatabasePassword<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            uid: Default::default(),
        }
    }

    pub fn database(mut self, uid: String) -> Self {
        self.uid = uid;

        self
    }

    pub async fn exec(self) -> Result<Database, Error> {
        if self.uid.len() == 0 {
            return Err(Error::from("database id is required"));
        }

        let req = Builder::new(
            &format!("/api/client/servers/{}/database/{}/rotate-password", self.id, self.uid)
        ).method("POST")?;
        let res = self.http.request::<FractalData<Database>>(req).await?;

        Ok(res.unwrap().attributes)
    }
}

pub struct DeleteDatabase<'a> {
    http: &'a Client,
    id: String,
    uid: String,
}

impl<'a> DeleteDatabase<'a> {
    #[doc(hidden)]
    pub fn new(http: &'a Client, id: String) -> Self {
        Self {
            http,
            id,
            uid: Default::default(),
        }
    }

    pub fn database(mut self, uid: String) -> Self {
        self.uid = uid;

        self
    }

    pub async fn exec(self) -> Result<(), Error> {
        if self.uid.len() == 0 {
            return Err(Error::from("database id is required"));
        }

        let req = Builder::new(
            &format!("/api/client/servers/{}/databases/{}", self.id, self.uid)
        ).method("DELETE")?;
        self.http.request::<()>(req).await?;

        Ok(())
    }
}
