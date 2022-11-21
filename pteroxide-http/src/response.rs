use hyper::{body::{self, Buf}, Response as HResponse, StatusCode};

use crate::error::Error;

/// A wrapper struct over the default HTTP client's responses for easier handling of API responses.
pub struct Response<T> {
    inner: HResponse<T>,
}

impl<T: body::HttpBody> Response<T> {
    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    /// Consumes the inner response body into a [`bytes`] object.
    /// 
    /// [`bytes`]: hyper::body::Bytes
    pub async fn bytes(self) -> Result<body::Bytes, Error>
    where
        Error: From<<T as body::HttpBody>::Error>
    {
        Ok(body::to_bytes(self.inner.into_body()).await?)
    }

    /// Consumes the inner response body into a model object. While used primarily for
    /// deserializing into [`pteroxide-models`] models, this can be used for external or
    /// third-party extensions of the API that implement the [`Deserialize`] trait.
    /// 
    /// [`pteroxide-models`]: pteroxide_models
    /// [`Deserialize`]: serde::de::Deserialize
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

impl<T> From<HResponse<T>> for Response<T> {
    fn from(inner: HResponse<T>) -> Self {
        Self { inner }
    }
}
