use base64::{engine::general_purpose, Engine as _};
use mockito;
use mockito::*;
use mystiko_indexer_client::errors::ClientError;
use mystiko_indexer_client::response::ApiResponse;
use mystiko_indexer_client::{
    client::IndexerClient,
    types::commitment_queued::{
        CommitmentQueuedFilter, CommitmentQueuedRequest, CommitmentQueuedResponse,
    },
};
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

#[tokio::test]
async fn test_find_commitment_queued_for_chain() {
    let TestClientSetupData {
        mut mocked_server,
        indexer_client,
    } = setup().await.unwrap();
    let chain_id = 5;
    let start_block = 100;
    let end_block = 10000;
    let where_filter = CommitmentQueuedFilter::builder()
        .commit_hash("commit_hash 1".to_string())
        .build();
    let json_str = serde_json::to_string(&where_filter).unwrap();
    let where_filter2 = serde_json::from_str::<Option<CommitmentQueuedFilter>>(&json_str).unwrap();
    assert!(where_filter2.is_some());
    let resp_list: Vec<CommitmentQueuedResponse> = vec![
        CommitmentQueuedResponse::builder()
            .id(1)
            .chain_id(5)
            .block_num(100)
            .commit_hash("commit hash 1".to_string())
            .contract_address("contract_address 1".to_string())
            .contract_id(1)
            .create_at(Some(100))
            .encrypted_note("encrypted_note 1".to_string())
            .leaf_index(1)
            .rollup_fee("rollup_fee 1".to_string())
            .status(Some(1))
            .tx_hash("tx_hash 1".to_string())
            .build(),
        CommitmentQueuedResponse::builder()
            .id(2)
            .chain_id(25)
            .block_num(200)
            .commit_hash("commit hash 2".to_string())
            .contract_address("contract_address 2".to_string())
            .contract_id(2)
            .create_at(Some(200))
            .encrypted_note("encrypted_note 2".to_string())
            .leaf_index(2)
            .rollup_fee("rollup_fee 2".to_string())
            .status(Some(2))
            .tx_hash("tx_hash 2".to_string())
            .build(),
        CommitmentQueuedResponse::builder()
            .id(3)
            .chain_id(25)
            .block_num(300)
            .commit_hash("commit hash 3".to_string())
            .contract_address("contract_address 3".to_string())
            .contract_id(2)
            .create_at(Some(300))
            .encrypted_note("encrypted_note 3".to_string())
            .leaf_index(3)
            .rollup_fee("rollup_fee 3".to_string())
            .status(None)
            .tx_hash("tx_hash 3".to_string())
            .build(),
    ];
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: &resp_list,
    };
    let m = mocked_server
        .mock(
            "post",
            "/chains/5/events/commitment-queued?startBlock=100&endBlock=10000",
        )
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let where_filter = CommitmentQueuedFilter::builder().build();
    let resp = indexer_client
        .find_commitment_queued_for_chain(
            CommitmentQueuedRequest::builder()
                .chain_id(chain_id)
                .start_block(start_block)
                .end_block(end_block)
                .where_filter(where_filter)
                .build(),
        )
        .await;
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap(), resp_list);
    m.assert_async().await;
    let mocked_api_resp = ApiResponse {
        code: -1,
        result: "unknow error",
    };
    let m = mocked_server
        .mock(
            "post",
            "/chains/5/events/commitment-queued",
        )
        .with_status(200)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client
        .find_commitment_queued_for_chain(
            CommitmentQueuedRequest::builder()
                .chain_id(chain_id)
                .build(),
        )
        .await;
    assert!(resp.is_err());
    m.assert_async().await;
}
