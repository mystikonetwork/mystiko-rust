mod common;

use crate::common::MockStaticCache;
use async_compression::tokio::write::GzipEncoder;
use mystiko_static_cache::{GzipStaticCache, StaticCache};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_get_compressed() {
    let mut inner = MockStaticCache::new();
    let compressed_data = generate_compressed_data("test get").await;
    inner
        .expect_get()
        .withf(|url, _| url == "http://localhost/test.txt")
        .returning(move |_, _| Ok(compressed_data.clone()));
    let cache = GzipStaticCache::new(Arc::new(inner));
    let data = cache.get("http://localhost/test.txt", None).await.unwrap();
    assert_eq!(data, b"test get");
}

#[tokio::test]
async fn test_get_plaintext() {
    let mut inner = MockStaticCache::new();
    inner
        .expect_get()
        .withf(|url, _| url == "http://localhost/test.txt")
        .returning(|_, _| Ok(b"test get".to_vec()));
    let cache = GzipStaticCache::new(Arc::new(inner));
    let data = cache.get("http://localhost/test.txt", None).await.unwrap();
    assert_eq!(data, b"test get");
}

#[tokio::test]
async fn test_get_failover() {
    let mut inner = MockStaticCache::new();
    let compressed_data = generate_compressed_data("test get").await;
    inner
        .expect_get_failover()
        .withf(|urls, _| urls == ["http://localhost/test.txt".to_string()])
        .returning(move |_, _| Ok(compressed_data.clone()));
    let cache = GzipStaticCache::new(Arc::new(inner));
    let data = cache
        .get_failover(&["http://localhost/test.txt".to_string()], None)
        .await
        .unwrap();
    assert_eq!(data, b"test get");
}

async fn generate_compressed_data(text: &str) -> Vec<u8> {
    let mut decoder = GzipEncoder::new(Vec::new());
    decoder.write_all(text.as_bytes()).await.unwrap();
    decoder.shutdown().await.unwrap();
    decoder.into_inner()
}
