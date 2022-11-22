use hyper::{
    body::{self, Buf},
    client::HttpConnector,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client as HClient, Request, StatusCode,
};
use hyper_tls::HttpsConnector;
use pteroxide_models::FractalError;
use serde::Deserialize;

use self::users::GetUsers;
use super::{Builder, Error};

pub mod users;

#[derive(Debug)]
pub struct Application {
    http: HClient<HttpsConnector<HttpConnector>>,
    url: String,
    key: String,
}

impl Application {
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

    pub async fn request<T>(&self, builder: Builder) -> Result<T, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let uri = format!("{}{}", self.url, builder.route);
        let req = Request::builder()
            .uri(uri)
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

    pub fn get_users(&self) -> GetUsers<'_> {
        GetUsers::new(self)
    }
}
