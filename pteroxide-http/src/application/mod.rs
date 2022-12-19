use hyper::{
    body::{self, Buf},
    client::HttpConnector,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client as HClient, Request, StatusCode,
};
use hyper_tls::HttpsConnector;
use pteroxide_models::fractal::FractalError;
use serde::Deserialize;

use self::servers::{CreateServer, GetServer, GetServers};
use self::users::{CreateUser, DeleteUser, GetUser, GetUsers, UpdateUser};
use super::{Builder, Error};

pub mod servers;
pub mod users;

/// The main interface for interacting with the application API.
#[derive(Debug)]
pub struct Application {
    http: HClient<HttpsConnector<HttpConnector>>,
    url: String,
    key: String,
}

impl Application {
    /// Constructs a new [`Application`] with the given API credentials.
    pub fn new(url: String, mut key: String) -> Self {
        let conn = HttpsConnector::new();
        if !key.starts_with("Bearer ") {
            key.insert_str(0, "Bearer ");
        }

        Self {
            http: HClient::builder().build(conn),
            url,
            key,
        }
    }

    /// Performs an API request using the [`Builder`] with the set fields. Returns a result with
    /// the deserialized API response, if any.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let builder = Builder::default().route(Route::GetUser { id: 2 });
    /// let data = app.request::<FractalItem<User>>(builder).await?;
    /// println!("{:#?}", data.attributes);
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an [`Error`] if the request fails or if the response fails to be deserialized.
    pub async fn request<T>(&self, mut builder: Builder) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let req = Request::builder()
            .uri(builder.uri(self.url.clone())) // problematic
            .method(builder.method)
            .header(USER_AGENT, "Pteroxide HTTP Client")
            .header(AUTHORIZATION, self.key.clone())
            .header(CONTENT_TYPE, builder.content_type)
            .header(ACCEPT, builder.accept_type)
            .body(builder.body)?;

        let res = self.http.request(req).await?;
        match res.status() {
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::ACCEPTED
            | StatusCode::NO_CONTENT => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader(buf.reader())
                    .expect("failed to deserialize into model");

                Ok(data)
            }
            _ => {
                let buf = body::aggregate(res).await?;
                let data = serde_json::from_reader::<_, FractalError>(buf.reader())
                    .expect("failed to deserialize into model");

                Err(Error::from(data))
            }
        }
    }

    /// Returns a request builder for getting a list of [`User`]s.
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn get_users(&self) -> GetUsers<'_> {
        GetUsers::new(self)
    }

    /// Returns a request builder for getting a specified [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn get_user(&self, id: i32) -> GetUser<'_> {
        GetUser::new(self, id)
    }

    /// Returns a request builder for creating a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub fn create_user(&self) -> CreateUser<'_> {
        CreateUser::new(self)
    }

    /// Returns a request builder for updating a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub fn update_user(&self, id: i32) -> UpdateUser<'_> {
        UpdateUser::new(self, id)
    }

    /// Returns a request builder for deleting a [`User`].
    ///
    /// [`User`]: pteroxide_models::application::User
    pub const fn delete_user(&self, id: i32) -> DeleteUser<'_> {
        DeleteUser::new(self, id)
    }

    /// Returns a request builder for getting a list of [`Server`]s.
    ///
    /// [`Server`]: pteroxide_models::application::Server
    pub const fn get_servers(&self) -> GetServers<'_> {
        GetServers::new(self)
    }

    pub const fn get_server(&self, id: i32) -> GetServer<'_> {
        GetServer::new(self, id)
    }

    pub fn create_server(&self) -> CreateServer<'_> {
        CreateServer::new(self)
    }
}
