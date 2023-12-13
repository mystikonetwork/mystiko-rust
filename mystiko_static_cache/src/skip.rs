use crate::{GetOptions, StaticCache};
use anyhow::Result;
use async_trait::async_trait;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct SkipStaticCache {
    http_client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum SkipStaticCacheError {
    #[error(transparent)]
    HttpClientError(#[from] reqwest::Error),
    #[error("http resource {0} returned status code {1}")]
    HttpError(String, u16),
    #[error("exhausted all urls")]
    FailoverUrlExhaustedError,
}

#[async_trait]
impl StaticCache for SkipStaticCache {
    async fn get(&self, url: &str, _options: Option<GetOptions>) -> Result<Vec<u8>> {
        let response = self.http_client.get(url).send().await?;
        if response.status().is_success() {
            let data = response.bytes().await?;
            Ok(data.to_vec())
        } else {
            Err(SkipStaticCacheError::HttpError(url.to_string(), response.status().as_u16()).into())
        }
    }

    async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>> {
        for url in urls {
            match self.get(url, options.clone()).await {
                Ok(data) => return Ok(data),
                Err(err) => log::warn!("failed to get {}: {:?}", url, err),
            }
        }
        Err(SkipStaticCacheError::FailoverUrlExhaustedError.into())
    }
}
