use crate::{GetOptions, SkipStaticCache, StaticCache};
use anyhow::Result;
use async_trait::async_trait;
use blake2::digest::Digest;
use blake2::Blake2s256;
use std::path::{Path, PathBuf};
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FileStaticCache {
    cache_folder: PathBuf,
    #[builder(default)]
    inner: SkipStaticCache,
}

#[derive(Debug, Error)]
pub enum FileStaticCacheError {
    #[error("exhausted all urls")]
    FailoverUrlExhaustedError,
}

impl FileStaticCache {
    pub async fn new<P>(cache_folder: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let cache_folder = cache_folder.as_ref().to_path_buf();
        if !tokio::fs::try_exists(&cache_folder).await? {
            tokio::fs::create_dir_all(&cache_folder).await?;
        }
        Ok(Self::builder().cache_folder(cache_folder).build())
    }
}

#[async_trait]
impl StaticCache for FileStaticCache {
    async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>> {
        let options = options.unwrap_or_default();
        let cached_file = self.cache_folder.join(calc_url_hash(url));
        let data = if options.skip_cache {
            self.inner.get(url, None).await?
        } else if tokio::fs::try_exists(&cached_file).await? {
            return Ok(tokio::fs::read(&cached_file).await?);
        } else {
            self.inner.get(url, None).await?
        };
        tokio::fs::write(&cached_file, &data).await?;
        Ok(data)
    }

    async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>> {
        for url in urls {
            match self.get(url, options.clone()).await {
                Ok(data) => return Ok(data),
                Err(err) => log::warn!("failed to get {}: {:?}", url, err),
            }
        }
        Err(FileStaticCacheError::FailoverUrlExhaustedError.into())
    }
}

fn calc_url_hash(url: &str) -> String {
    let mut hasher = Blake2s256::default();
    hasher.update(url.as_bytes());
    hex::encode(hasher.finalize())
}
