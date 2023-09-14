mod common;

use crate::common::MockTestingService;
use anyhow::Result;
use mystiko_grpc::{connect, GrpcServer};
use mystiko_protos::service::v1::{ClientOptions, ServerOptions};
use mystiko_protos::testing::v1::testing_service_client::TestingServiceClient;
use mystiko_protos::testing::v1::testing_service_server::TestingServiceServer;
use mystiko_protos::testing::v1::{EchoRequest, EchoResponse};
use rcgen::{generate_simple_self_signed, Certificate, CertificateParams};
use std::path::PathBuf;
use tonic::Response;

#[tokio::test]
async fn test_grpc_client() {
    let options = ServerOptions::builder().port(50051u32).build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder().port(50051u32).build();
    let mut client = TestingServiceClient::new(connect(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_grpc_client_with_ssl() {
    let tls_keys = TlsKeys::generate("example.com").unwrap();
    let options = ServerOptions::builder()
        .port(50052u32)
        .tls_key(tls_keys.server_key)
        .tls_pem(tls_keys.server_pem)
        .build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder()
        .port(50052u32)
        .is_ssl(true)
        .ssl_cert(tls_keys.ca_pem)
        .ssl_server_name("example.com".to_string())
        .build();
    let mut client = TestingServiceClient::new(connect(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await.unwrap();
}

#[tokio::test]
async fn test_grpc_client_with_ssl_key_file() {
    let tls_keys = TlsKeys::generate("127.0.0.1").unwrap();
    let temp_folder = tempfile::tempdir().unwrap();
    let path = PathBuf::from(temp_folder.path()).join("ca.pem");
    tokio::fs::write(path.as_path(), tls_keys.ca_pem.clone()).await.unwrap();
    let options = ServerOptions::builder()
        .port(50053u32)
        .tls_key(tls_keys.server_key)
        .tls_pem(tls_keys.server_pem)
        .build();
    let mut server = setup(options).await;
    let client_options = ClientOptions::builder()
        .port(50053u32)
        .is_ssl(true)
        .ssl_cert_path(path.to_string_lossy().to_string())
        .build();
    let mut client = TestingServiceClient::new(connect(&client_options).await.unwrap());
    let response = client
        .echo(EchoRequest::builder().message("hello world").build())
        .await
        .unwrap();
    assert_eq!(response.get_ref().message, "hello world");
    server.stop().await.unwrap();
}

async fn setup(options: ServerOptions) -> GrpcServer {
    let mut server = GrpcServer::default();
    let mut service = MockTestingService::new();
    service.expect_echo().returning(|request| {
        Ok(Response::new(
            EchoResponse::builder()
                .message(request.get_ref().message.to_string())
                .build(),
        ))
    });
    server.start(TestingServiceServer::new(service), options).await.unwrap();
    server
}

#[derive(Debug, Clone)]
pub struct TlsKeys {
    pub ca_pem: String,
    pub server_key: String,
    pub server_pem: String,
}

impl TlsKeys {
    pub fn generate(domain_name: &str) -> Result<Self> {
        let ca_cert = generate_simple_self_signed(vec![domain_name.to_string()])?;
        let ca_pem = ca_cert.serialize_pem()?;
        let mut server_params = CertificateParams::new(vec![domain_name.to_string()]);
        server_params.is_ca = rcgen::IsCa::NoCa;
        let server_cert = Certificate::from_params(server_params)?;
        let server_pem = server_cert.serialize_pem_with_signer(&ca_cert)?;
        let server_key = server_cert.serialize_private_key_pem();
        Ok(Self {
            ca_pem,
            server_key,
            server_pem,
        })
    }
}
