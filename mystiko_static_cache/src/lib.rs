#[cfg(feature = "fs")]
mod file;
#[cfg(feature = "gzip")]
mod gzip;

#[cfg(feature = "fs")]
pub use file::*;
#[cfg(feature = "gzip")]
pub use gzip::*;

use anyhow::Result;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct GetOptions {
    pub skip_cache: bool,
}

#[async_trait]
pub trait StaticCache: Send + Sync {
    async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>>;

    async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>>;
}

#[async_trait]
impl StaticCache for Box<dyn StaticCache> {
    async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>> {
        self.as_ref().get(url, options).await
    }

    async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>> {
        self.as_ref().get_failover(urls, options).await
    }
}
