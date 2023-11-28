mod common;

use common::MockStaticCache;
use mystiko_static_cache::StaticCache;

#[tokio::test]
async fn test_get() {
    let mut inner = MockStaticCache::new();
    inner
        .expect_get()
        .withf(|url, _| url == "http://localhost/test.txt")
        .returning(|_, _| Ok(b"test get".to_vec()));
    let cache = Box::new(inner) as Box<dyn StaticCache>;
    let data = cache.get("http://localhost/test.txt", None).await.unwrap();
    assert_eq!(data, b"test get");
}

#[tokio::test]
async fn test_get_failover() {
    let mut inner = MockStaticCache::new();
    inner
        .expect_get_failover()
        .withf(|urls, _| urls == ["http://localhost/test.txt".to_string()])
        .returning(|_, _| Ok(b"test get".to_vec()));
    let cache = Box::new(inner) as Box<dyn StaticCache>;
    let data = cache
        .get_failover(&["http://localhost/test.txt".to_string()], None)
        .await
        .unwrap();
    assert_eq!(data, b"test get");
}
