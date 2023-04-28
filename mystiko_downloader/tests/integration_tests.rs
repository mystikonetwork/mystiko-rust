use flate2::write::GzEncoder;
use flate2::Compression;
use mockito::{Server, ServerGuard};
use mystiko_downloader::{DownloadOptions, Downloader, DownloaderBuilder};
use std::io::Write;
use tempfile::{tempdir, TempDir};
use tokio::fs;
use tokio::fs::remove_dir_all;

async fn build_resource() -> (ServerGuard, Downloader, TempDir) {
    let server = Server::new_async().await;
    let cache_folder = tempdir().unwrap();
    let downloader = DownloaderBuilder::new()
        .folder(cache_folder.path().to_str().unwrap())
        .build()
        .await
        .unwrap();
    (server, downloader, cache_folder)
}

fn generate_compression_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

#[tokio::test]
async fn test_download() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_body("test file")
        .create_async()
        .await;
    let url = &format!("{}/test.txt", server.url());
    let file_path1 = downloader.download(url, None).await.unwrap();
    let file_path2 = downloader.download(url, None).await.unwrap();
    path.assert_async().await;
    assert_eq!(file_path1, file_path2);
    assert_eq!(fs::read_to_string(&file_path1).await.unwrap(), "test file");
}

#[tokio::test]
async fn test_download_skip_cache() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path = server
        .mock("GET", "/test.txt")
        .expect(2)
        .with_body("test file")
        .create_async()
        .await;
    let url = &format!("{}/test.txt", server.url());
    let options = DownloadOptions {
        skip_cache: true,
        ..DownloadOptions::default()
    };
    let file_path1 = downloader.download(url, Some(options.clone())).await.unwrap();
    let file_path2 = downloader.download(url, Some(options)).await.unwrap();
    path.assert_async().await;
    assert_eq!(file_path1, file_path2);
    assert_eq!(fs::read_to_string(&file_path2).await.unwrap(), "test file");
}

#[tokio::test]
async fn test_download_compressed() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path1 = server
        .mock("GET", "/test_file1.gz")
        .expect(1)
        .with_body(generate_compression_data(b"file1"))
        .create_async()
        .await;
    let path2 = server
        .mock("GET", "/test_file2.tar.gz")
        .expect(1)
        .with_body(generate_compression_data(b"file2"))
        .create_async()
        .await;
    let path3 = server
        .mock("GET", "/test_file3.tgz")
        .expect(2)
        .with_body(generate_compression_data(b"file3"))
        .create_async()
        .await;
    let file1_path = downloader
        .download(&format!("{}/test_file1.gz", server.url()), None)
        .await
        .unwrap();
    let file2_path = downloader
        .download(&format!("{}/test_file2.tar.gz", server.url()), None)
        .await
        .unwrap();
    let file3_path = downloader
        .download(&format!("{}/test_file3.tgz", server.url()), None)
        .await
        .unwrap();
    let file4_path = downloader
        .download(&format!("{}/test_file1.gz", server.url()), None)
        .await
        .unwrap();
    let file5_path = downloader
        .download(
            &format!("{}/test_file3.tgz", server.url()),
            Some(DownloadOptions {
                skip_cache: true,
                ..DownloadOptions::default()
            }),
        )
        .await
        .unwrap();
    path1.assert_async().await;
    path2.assert_async().await;
    path3.assert_async().await;
    assert_eq!(fs::read_to_string(&file1_path).await.unwrap(), "file1");
    assert_eq!(fs::read_to_string(&file2_path).await.unwrap(), "file2");
    assert_eq!(fs::read_to_string(&file3_path).await.unwrap(), "file3");
    assert_eq!(file1_path, file4_path);
    assert_eq!(file3_path, file5_path);
}

#[tokio::test]
async fn test_download_skip_decompression() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path = server
        .mock("GET", "/test_file.gz")
        .expect(1)
        .with_body(generate_compression_data(b"file content"))
        .create_async()
        .await;
    let options = DownloadOptions {
        skip_decompression: true,
        ..DownloadOptions::default()
    };
    let file_path = downloader
        .download(&format!("{}/test_file.gz", server.url()), Some(options))
        .await
        .unwrap();
    path.assert_async().await;
    assert_eq!(
        fs::read(&file_path).await.unwrap(),
        generate_compression_data(b"file content")
    );
}

#[tokio::test]
async fn test_download_error() {
    let mut downloader = DownloaderBuilder::new().build().await.unwrap();
    let mut server = Server::new_async().await;
    let path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_status(500)
        .create_async()
        .await;
    assert!(downloader
        .download(&format!("{}/test.txt", server.url()), None)
        .await
        .is_err());
    path.assert_async().await;
    assert!(downloader.download("not a url", None).await.is_err());
    remove_dir_all(&downloader.folder).await.unwrap();
}

#[tokio::test]
async fn test_download_failover() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    assert!(downloader.download_failover(&vec![], None).await.is_err());
    let path1 = server
        .mock("GET", "/test1.txt")
        .expect(1)
        .with_status(500)
        .create_async()
        .await;
    let path2 = server
        .mock("GET", "/test2.txt")
        .expect(1)
        .with_body("file content")
        .create_async()
        .await;
    let file_path = downloader
        .download_failover(
            &vec![
                format!("{}/test1.txt", server.url()),
                format!("{}/test2.txt", server.url()),
            ],
            None,
        )
        .await
        .unwrap();
    path1.assert_async().await;
    path2.assert_async().await;
    assert_eq!(fs::read_to_string(&file_path).await.unwrap(), "file content");
}

#[tokio::test]
async fn test_read_bytes() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path = server
        .mock("GET", "/test.txt")
        .expect(1)
        .with_body("file content")
        .create_async()
        .await;
    let bytes = downloader
        .read_bytes(&format!("{}/test.txt", server.url()), None)
        .await
        .unwrap();
    path.assert_async().await;
    assert_eq!(String::from_utf8(bytes).unwrap(), "file content");
}

#[tokio::test]
async fn test_read_bytes_failover() {
    let (mut server, mut downloader, _cache_folder) = build_resource().await;
    let path1 = server
        .mock("GET", "/test1.txt")
        .expect(1)
        .with_status(500)
        .create_async()
        .await;
    let path2 = server
        .mock("GET", "/test2.txt")
        .expect(1)
        .with_body("file content")
        .create_async()
        .await;
    let bytes = downloader
        .read_bytes_failover(
            &vec![
                format!("{}/test1.txt", server.url()),
                format!("{}/test2.txt", server.url()),
            ],
            None,
        )
        .await
        .unwrap();
    path1.assert_async().await;
    path2.assert_async().await;
    assert_eq!(String::from_utf8(bytes).unwrap(), "file content");
}
