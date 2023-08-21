use anyhow::Result;
use async_compression::tokio::bufread::{ZstdDecoder, ZstdEncoder};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Debug, Clone, Default)]
pub struct ZstdCompression;

#[async_trait]
impl crate::Compression for ZstdCompression {
    fn content_type(&self) -> &'static str {
        "application/zstd"
    }

    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = ZstdEncoder::new(BufReader::new(data));
        let mut bytes = Vec::<u8>::new();
        encoder.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }

    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZstdDecoder::new(BufReader::new(data));
        let mut bytes = Vec::<u8>::new();
        decoder.read_to_end(&mut bytes).await?;
        Ok(bytes)
    }
}
