use base64::{engine::general_purpose, Engine as _};
use mockito;
use mockito::*;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::errors::ClientError;
use mystiko_indexer_client::response::ApiResponse;
use serde_json;

static AUTH_USERNAME: &str = "test_username";
static AUTH_PASSWORD: &str = "110110";

struct TestClientSetupData {
    mocked_server: mockito::ServerGuard,
    indexer_client: IndexerClient,
}

fn create_indexer_client(base_url: &str) -> IndexerClient {
    IndexerClient::builder(base_url).build().unwrap()
}

fn create_indexer_client_with_auth(base_url: &str) -> IndexerClient {
    IndexerClient::builder(base_url)
        .auth_username(Some(String::from(AUTH_USERNAME)))
        .auth_password(Some(String::from(AUTH_PASSWORD)))
        .build()
        .unwrap()
}

async fn setup() -> Result<TestClientSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = create_indexer_client(&mocked_server_url);
    Ok(TestClientSetupData {
        mocked_server,
        indexer_client,
    })
}

async fn setup_with_auth() -> Result<TestClientSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = create_indexer_client_with_auth(&mocked_server_url);
    Ok(TestClientSetupData {
        mocked_server,
        indexer_client,
    })
}

fn generate_auth_header() -> String {
    let str = AUTH_USERNAME.to_owned() + ":" + AUTH_PASSWORD;
    let encoded = general_purpose::STANDARD.encode(&str);
    String::from("Basic ".to_owned() + &encoded)
}

#[test]
fn test_create_indexer_client() {
    let base_url = "http://test_url:3098";
    let client = create_indexer_client(base_url);
    assert_eq!(client.base_url, base_url);
}

#[tokio::test]
async fn test_ping() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();
    let message = String::from("hello");
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/ping?message=hello")
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.ping(&message).await.unwrap();
    assert_eq!(resp, message);
    m.assert_async().await;
}

#[tokio::test]
async fn test_auth_ping() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup_with_auth().await.unwrap();
    let message = String::from("helloauth");
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.auth_ping(&message).await.unwrap();
    assert_eq!(resp, message);
    m.assert_async().await;
    let mocked_error_api_resp = ApiResponse {
        code: -1,
        result: message.clone(),
    };
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_error_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp2 = indexer_client.auth_ping(&message).await;
    assert!(resp2.is_err());
    assert_eq!(
        resp2.err(),
        Some(ClientError::ApiResponseError {
            code: -1,
            message: String::from("any message")
        })
    );
    m.assert_async().await;
    let m = mocked_server
        .mock("get", "/auth-ping?message=helloauth")
        .match_header("Authorization", Matcher::Exact(generate_auth_header()))
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_error_api_resp).unwrap())
        .create_async()
        .await;
    let resp3 = indexer_client.auth_ping(&message).await;
    assert!(resp3.is_err());
    assert_eq!(
        resp3.err(),
        Some(ClientError::UnsupportedContentTypeError(String::from("")))
    );
    m.assert_async().await;
}
