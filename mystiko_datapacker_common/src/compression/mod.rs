mod zstd;

use anyhow::Result;
use async_trait::async_trait;
pub use zstd::*;

#[async_trait]
pub trait Compression: Send + Sync {
    fn content_type(&self) -> &'static str;
    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>>;
    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>>;
}

#[async_trait]
impl Compression for Box<dyn Compression> {
    fn content_type(&self) -> &'static str {
        self.as_ref().content_type()
    }

    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.as_ref().compress(data).await
    }

    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.as_ref().decompress(data).await
    }
}
