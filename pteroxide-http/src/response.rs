use hyper::{body::{self, Buf}, Response as HResponse, StatusCode};

use crate::error::Error;

pub struct Response<T> {
    inner: HResponse<T>,
}

impl<T: body::HttpBody> Response<T> {
    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    pub async fn bytes(self) -> Result<body::Bytes, Error>
    where
        Error: From<<T as body::HttpBody>::Error>
    {
        Ok(body::to_bytes(self.inner.into_body()).await?)
    }

    pub async fn model<D>(self) -> Result<D, Error>
    where
        D: for<'de> serde::Deserialize<'de>,
        Error: From<<T as body::HttpBody>::Error>
    {
        let buf = body::aggregate(self.inner).await?;
        let data = serde_json::from_reader(buf.reader()).expect("failed to deserialize into model");

        Ok(data)
    }
}
