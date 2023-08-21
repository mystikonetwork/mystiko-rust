mod zstd;

use anyhow::Result;
use async_trait::async_trait;
pub use zstd::*;

#[async_trait]
pub trait Compression {
    type Err;

    fn content_type() -> &'static str;
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Err>;
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Err>;
}
