use crate::{GetOptions, StaticCache};
use anyhow::Result;
use async_compression::tokio::write::GzipDecoder;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use typed_builder::TypedBuilder;

const GZIP_MAGIC: &[u8] = &[0x1f, 0x8b];

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GzipStaticCache<C: StaticCache = Box<dyn StaticCache>> {
    inner: C,
}

impl<C> GzipStaticCache<C>
where
    C: StaticCache,
{
    pub fn new(inner: C) -> Self {
        Self::builder().inner(inner).build()
    }
}

#[async_trait]
impl<C> StaticCache for GzipStaticCache<C>
where
    C: StaticCache,
{
    async fn get(&self, url: &str, options: Option<GetOptions>) -> Result<Vec<u8>> {
        let raw = self.inner.get(url, options).await?;
        try_decompress(raw).await
    }

    async fn get_failover(&self, urls: &[String], options: Option<GetOptions>) -> Result<Vec<u8>> {
        let raw = self.inner.get_failover(urls, options).await?;
        try_decompress(raw).await
    }
}

async fn try_decompress(data: Vec<u8>) -> Result<Vec<u8>> {
    if !is_gzip(&data) {
        return Ok(data);
    }
    let mut decoder = GzipDecoder::new(Vec::new());
    decoder.write_all(&data).await?;
    decoder.shutdown().await?;
    Ok(decoder.into_inner())
}

fn is_gzip(data: &[u8]) -> bool {
    data.len() >= 2 && data[0..2] == *GZIP_MAGIC
}
