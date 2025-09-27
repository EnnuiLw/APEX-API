pub mod cache;
pub mod http;
pub mod url;

use bytes::Bytes;
use reqwest::header;
use serde::de::DeserializeOwned;
use std::path::Path;

use crate::client::cache::{Holder, MODEL_HASH};

const CACHE_EXPIRE_SECOND_TIME: u64 = 3600;

pub struct Client(reqwest::Client);

impl Client {
    pub fn new(token: impl Into<String>) -> Self {
        let token: String = token.into();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            token
                .parse()
                .expect("Unable to parse for Authorization header."),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Unable to build Client.");

        Self(client)
    }

    #[cfg(feature = "env")]
    pub fn from_env_path<P: AsRef<Path>>(path: P) -> crate::ClientResult<Self> {
        dotenvy::from_path(path)?;

        let token = std::env::var("apex_token").expect("Unable to find `apex_token` in .env file");
        Ok(Self::new(token))
    }

    #[cfg(feature = "env")]
    pub fn from_env() -> crate::ClientResult<Self> {
        Self::from_env_path(".env")
    }

    pub async fn connect(&self, endpoint: &str) -> crate::ClientResult<reqwest::Response> {
        Ok(self
            .0
            .get(endpoint)
            .send()
            .await
            .map_err(|e| crate::Error::Reqwest(e))?)
    }

    async fn _connect(
        &self,
        endpoint: &str,
        cache_key: Option<&str>,
    ) -> crate::ClientResult<Bytes> {
        let req = || async { Ok(self.connect(endpoint).await?.bytes().await?) };

        if let Some(cache_key) = cache_key {
            let data =
                Holder::get_and_set(cache_key, &MODEL_HASH, CACHE_EXPIRE_SECOND_TIME, req).await?;
            return Ok(data);
        } else {
            req().await
        }
    }

    pub async fn connect_with_json<T>(
        &self,
        endpoint: &str,
        cache_key: Option<&str>,
    ) -> crate::ClientResult<T>
    where
        T: DeserializeOwned,
    {
        let req = self._connect(endpoint, cache_key).await?;
        Ok(serde_json::from_slice(&req).map_err(|e| crate::Error::Serialize(e))?)
    }
}

#[async_trait::async_trait]
impl http::BaseClient for Client {
    async fn inner_connect<T>(&self, endpoint: &str) -> crate::ClientResult<T>
    where
        T: DeserializeOwned,
    {
        let cache_key = cache::gen_cache_key(endpoint);
        Ok(self
            .connect_with_json::<T>(endpoint, Some(&cache_key))
            .await?)
    }
}
