use anyhow::Result;
use async_compression::tokio::bufread::{ZstdDecoder, ZstdEncoder};
use async_trait::async_trait;
use thiserror::Error;
use tokio::io::{AsyncReadExt, BufReader};

#[derive(Debug, Clone, Default)]
pub struct ZstdCompression;

#[derive(Debug, Error)]
pub enum ZstdCompressionError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[async_trait]
impl crate::Compression for ZstdCompression {
    type Err = ZstdCompressionError;

    fn content_type() -> &'static str {
        "application/zstd"
    }

    async fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Err> {
        let mut encoder = ZstdEncoder::new(BufReader::new(data));
        let mut bytes = Vec::<u8>::new();
        encoder.read_to_end(&mut bytes).await.map_err(Self::Err::IoError)?;
        Ok(bytes)
    }

    async fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Self::Err> {
        let mut decoder = ZstdDecoder::new(BufReader::new(data));
        let mut bytes = Vec::<u8>::new();
        decoder.read_to_end(&mut bytes).await.map_err(Self::Err::IoError)?;
        Ok(bytes)
    }
}
