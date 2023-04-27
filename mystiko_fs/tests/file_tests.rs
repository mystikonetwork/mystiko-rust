use mystiko_fs::{read_file_bytes, read_gzip_file_bytes};
use tokio::test;

#[test]
async fn test_read_file_bytes() {
    let content = read_file_bytes("./tests/files/file1.txt").await.unwrap();
    assert_eq!(content, "hello world\n".as_bytes().to_vec());
}

#[test]
async fn test_read_gzip_file_bytes() {
    let content = read_gzip_file_bytes("./tests/files/file2.txt.gz").await.unwrap();
    assert_eq!(content, "GZIP file\n".as_bytes().to_vec());
}
