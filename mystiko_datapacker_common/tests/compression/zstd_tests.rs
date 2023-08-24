use mystiko_datapacker_common::{Compression, ZstdCompression};

#[tokio::test]
async fn test_compression() {
    let compression = ZstdCompression;
    let bytes = "Hello Mystiko!\n".as_bytes();
    let compressed = compression.compress(bytes).await.unwrap();
    let decompressed = compression.decompress(&compressed).await.unwrap();
    assert_eq!(bytes, decompressed.as_slice());
    assert_eq!(compression.content_type(), "application/zstd");
}
