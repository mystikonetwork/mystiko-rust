use anyhow::Result;
use mystiko_screening_client::v1::{ApiResponse, ScreeningClientV1, ScreeningClientV1Options};
use mystiko_screening_client::{CertificateResponse, ScreeningClient};
use serde::{Deserialize, Serialize};
use serde_json::json;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_apply_success() {
    let (mut server, client) = setup().await.unwrap();
    let _ = MockCertificateResponse::builder()
        .deadline(1)
        .signature("0x123".to_string())
        .build()
        .into_mock(&mut server)
        .await;
    let request = mystiko_screening_client::CertificateRequest::builder()
        .chain_id(97)
        .asset(None)
        .account("0x123".to_string())
        .message("test".to_string())
        .signature("0x123".to_string())
        .build();
    let response = client.apply_certificate(&request).await.unwrap();
    assert_eq!(response.deadline, 1);
    assert_eq!(response.signature, "0x123");
}

#[tokio::test]
async fn test_apply_meet_error() {
    let (mut server, client) = setup().await.unwrap();
    let _ = MockApiResponse::builder()
        .code(-1)
        .data(None)
        .message(Some("meet_error".to_string()))
        .version("v1".to_string())
        .build()
        .into_mock(&mut server)
        .await;
    let request = mystiko_screening_client::CertificateRequest::builder()
        .chain_id(97)
        .asset(None)
        .account("0x123".to_string())
        .message("test".to_string())
        .signature("0x123".to_string())
        .build();
    let response = client.apply_certificate(&request).await;
    assert!(response.is_err());
    assert!(response.unwrap_err().to_string().contains("-1 meet_error"));

    let _ = MockApiResponse::builder()
        .code(0)
        .data(None)
        .message(None)
        .version("v1".to_string())
        .build()
        .into_mock(&mut server)
        .await;
    let response = client.apply_certificate(&request).await;
    assert!(response.is_err());
    assert!(response.unwrap_err().to_string().contains("empty response error"));
}

async fn setup() -> Result<(mockito::ServerGuard, ScreeningClientV1)> {
    let server = mockito::Server::new_async().await;
    let options = ScreeningClientV1Options::builder()
        .url(server.url())
        .http_client(None)
        .build();
    let client = ScreeningClientV1::new(options);
    Ok((server, client))
}

#[derive(Debug, TypedBuilder)]
struct MockCertificateResponse {
    deadline: u64,
    signature: String,
}

impl MockCertificateResponse {
    async fn into_mock(self, server: &mut mockito::ServerGuard) -> mockito::Mock {
        let api_response = ApiResponse::builder()
            .code(0)
            .data(Some(
                CertificateResponse::builder()
                    .deadline(self.deadline)
                    .signature(self.signature)
                    .build(),
            ))
            .message(None)
            .version("v1".to_string())
            .build();

        server
            .mock("POST", "/v1/certificate/apply")
            .with_status(200)
            .with_body(json!(api_response).to_string())
            .create_async()
            .await
    }
}

#[derive(Debug, TypedBuilder, Serialize, Deserialize)]
struct MockApiResponse {
    code: i32,
    data: Option<CertificateResponse>,
    message: Option<String>,
    version: String,
}

impl MockApiResponse {
    async fn into_mock(self, server: &mut mockito::ServerGuard) -> mockito::Mock {
        server
            .mock("POST", "/v1/certificate/apply")
            .with_status(200)
            .with_body(json!(self).to_string())
            .create_async()
            .await
    }
}
