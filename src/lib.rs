#![deny(clippy::all)]
#![deny(clippy::suspicious)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![doc = include_str!("../README.md")]

pub mod endpoints;
pub mod model;

use crate::model::internal::ExtractJson;
use reqwest::{Method, Url, header};
use serde::{Deserialize, Serialize};
use tracing::info;

pub struct KromerClient {
    url: Url,
    http: reqwest::Client,
}

#[derive(Debug, thiserror::Error)]
pub enum KromerError {
    #[error("krist error({error}): {message}")]
    Krist { error: String, message: String },
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}

impl KromerClient {
    /// Create a new client for the Kromer2 API. This will reuse connections.
    /// # Errors
    /// Errors if the passed in value is not valid.
    pub fn new(url: &str) -> Result<Self, KromerError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Ok(Self {
            url: Url::parse(url)?,
            http: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()?,
        })
    }

    pub(crate) async fn request<T, J, Q>(
        &self,
        method: Method,
        endpoint: &str,
        query: Option<Q>,
    ) -> Result<J, KromerError>
    where
        T: for<'a> Deserialize<'a> + ExtractJson<J>,
        Q: Serialize + Sized,
    {
        let mut req = self.http.request(method, self.url.join(endpoint)?);

        if let Some(q) = query {
            req = req.query(&q);
        }

        let req = req.build()?;
        info!(headers = ?req.headers(), "{} request sent to {}", req.method(), req.url());

        self.http.execute(req).await?.json::<T>().await?.extract()
    }

    #[inline]
    pub(crate) async fn get<T, J, Q>(
        &self,
        endpoint: &str,
        query: Option<Q>,
    ) -> Result<J, KromerError>
    where
        T: for<'a> Deserialize<'a> + ExtractJson<J>,
        Q: Serialize + Sized,
    {
        self.request::<T, J, Q>(Method::GET, endpoint, query).await
    }
}
