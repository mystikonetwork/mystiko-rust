use mockito::Server;
use mystiko_static_cache::{FileStaticCache, GetOptions, StaticCache};
use tempfile::{tempdir, TempDir};

#[tokio::test]
async fn test_get() {
    let mut server = Server::new_async().await;
    let mock_path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_body("test get")
        .create_async()
        .await;
    let (cache, _temp_dir) = setup().await;
    let data = cache.get(&format!("{}/test.txt", server.url()), None).await.unwrap();
    assert_eq!(data, b"test get");
    let data = cache.get(&format!("{}/test.txt", server.url()), None).await.unwrap();
    assert_eq!(data, b"test get");
    mock_path.assert_async().await;
}

#[tokio::test]
async fn test_get_skip_cache() {
    let mut server = Server::new_async().await;
    let mock_path = server
        .mock("GET", "/test.txt")
        .expect(2)
        .with_body("test get")
        .create_async()
        .await;
    let (cache, _temp_dir) = setup().await;
    let options = GetOptions::builder().skip_cache(true).build();
    let data = cache
        .get(&format!("{}/test.txt", server.url()), Some(options.clone()))
        .await
        .unwrap();
    assert_eq!(data, b"test get");
    let data = cache
        .get(&format!("{}/test.txt", server.url()), Some(options))
        .await
        .unwrap();
    assert_eq!(data, b"test get");
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
    let (cache, _temp_dir) = setup().await;
    let err = cache
        .get(&format!("{}/test.txt", server.url()), None)
        .await
        .unwrap_err();
    assert_eq!(
        err.to_string(),
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
    let (cache, _temp_dir) = setup().await;
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

#[tokio::test]
async fn test_get_failover_exhausted() {
    let mut server = Server::new_async().await;
    let mock_path1 = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_status(404)
        .create_async()
        .await;
    let mock_path2 = server
        .mock("GET", "/test2.txt")
        .expect(1)
        .with_status(404)
        .create_async()
        .await;
    let (cache, _temp_dir) = setup().await;
    let err = cache
        .get_failover(
            &[
                format!("{}/test.txt", server.url()),
                format!("{}/test2.txt", server.url()),
            ],
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(err.to_string(), "exhausted all urls");
    mock_path1.assert_async().await;
    mock_path2.assert_async().await;
}

async fn setup() -> (FileStaticCache, TempDir) {
    let _ = env_logger::builder()
        .filter_module("mystiko_static_cache", log::LevelFilter::Info)
        .try_init();
    let temp_dir = tempdir().unwrap();
    let cache = FileStaticCache::new(temp_dir.path()).await.unwrap();
    (cache, temp_dir)
}
