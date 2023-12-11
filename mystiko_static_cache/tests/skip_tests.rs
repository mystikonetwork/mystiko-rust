use mockito::Server;
use mystiko_static_cache::{SkipStaticCache, StaticCache};

#[tokio::test]
async fn test_get() {
    let mut server = Server::new_async().await;
    let mock_path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_body("test get")
        .create_async()
        .await;
    let cache = setup().await;
    let data = cache.get(&format!("{}/test.txt", server.url()), None).await.unwrap();
    assert_eq!(data, b"test get");
    mock_path.assert_async().await;
}

#[tokio::test]
async fn test_get_http_code_error() {
    let mut server = Server::new_async().await;
    let mock_path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_status(404)
        .create_async()
        .await;
    let cache = setup().await;
    let data = cache.get(&format!("{}/test.txt", server.url()), None).await;
    assert_eq!(
        data.unwrap_err().to_string(),
        format!("http resource {}/test.txt returned status code 404", server.url())
    );
    mock_path.assert_async().await;
}

#[tokio::test]
async fn test_get_failover() {
    let mut server = Server::new_async().await;
    let mock_path1 = server
        .mock("GET", "/test1.txt")
        .expect(1)
        .with_status(404)
        .create_async()
        .await;
    let mock_path2 = server
        .mock("GET", "/test2.txt")
        .expect(1)
        .with_body("test get")
        .create_async()
        .await;
    let cache = setup().await;
    let data = cache
        .get_failover(
            &[
                format!("{}/test1.txt", server.url()),
                format!("{}/test2.txt", server.url()),
            ],
            None,
        )
        .await
        .unwrap();
    assert_eq!(data, b"test get");
    mock_path1.assert_async().await;
    mock_path2.assert_async().await;
}

async fn setup() -> SkipStaticCache {
    let _ = env_logger::builder()
        .filter_module("mystiko_static_cache", log::LevelFilter::Info)
        .try_init();
    SkipStaticCache::default()
}
