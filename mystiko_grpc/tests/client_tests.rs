mod common;

use crate::common::{StartOptions, TestingServer, TlsKeys};
use mystiko_grpc::grpc_client;
use mystiko_protos::service::v1::ClientOptions;
use mystiko_protos::testing::v1::testing_service_client::TestingServiceClient;
use mystiko_protos::testing::v1::EchoRequest;
use std::path::PathBuf;

#[tokio::test]
async fn test_grpc_client() {
    let options = StartOptions::builder().port(50051u16).build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder().port(50051u32).build();
    let mut client = TestingServiceClient::new(grpc_client(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await;
}

#[tokio::test]
async fn test_grpc_client_with_ssl() {
    let tls_keys = TlsKeys::generate("example.com").unwrap();
    let options = StartOptions::builder()
        .port(50052u16)
        .tls_keys(tls_keys.clone())
        .build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder()
        .port(50052u32)
        .is_ssl(true)
        .ssl_cert(tls_keys.ca_pem)
        .ssl_server_name("example.com".to_string())
        .build();
    let mut client = TestingServiceClient::new(grpc_client(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await;
}

#[tokio::test]
async fn test_grpc_client_with_ssl_key_file() {
    let tls_keys = TlsKeys::generate("127.0.0.1").unwrap();
    let temp_folder = tempfile::tempdir().unwrap();
    let path = PathBuf::from(temp_folder.path()).join("ca.pem");
    tokio::fs::write(path.as_path(), tls_keys.ca_pem.clone()).await.unwrap();
    let options = StartOptions::builder()
        .port(50053u16)
        .tls_keys(tls_keys.clone())
        .build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder()
        .port(50053u32)
        .is_ssl(true)
        .ssl_cert_path(path.to_string_lossy().to_string())
        .build();
    let mut client = TestingServiceClient::new(grpc_client(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await;
}

async fn setup(options: StartOptions) -> TestingServer {
    let mut server = TestingServer::default();
    let sever_ready = server.start(options).await.unwrap();
    sever_ready.await.unwrap();
    server
}
