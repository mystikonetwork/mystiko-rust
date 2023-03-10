use mockito;
use mockito::*;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::errors::ClientError;
use mystiko_indexer_client::response::ApiResponse;
use serde_json;

struct TestErrorSetupData {
    mocked_server: mockito::ServerGuard,
    ping_message: String,
    mocked_api_resp: ApiResponse<String>,
    indexer_client: IndexerClient,
}

fn create_indexer_client(base_url: String) -> IndexerClient {
    IndexerClient::builder(base_url).build()
}

async fn setup() -> Result<TestErrorSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let ping_message = String::from("hello");
    let mocked_api_resp = ApiResponse {
        code: 0,
        result: ping_message.clone(),
    };
    let indexer_client = create_indexer_client(mocked_server_url.clone());
    Ok(TestErrorSetupData {
        mocked_server,
        ping_message,
        mocked_api_resp,
        indexer_client,
    })
}

#[tokio::test]
async fn test_with_reqwest_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
        mocked_api_resp: _,
        indexer_client,
    } = setup().await.unwrap();
    let unformable_api_resp = vec!["111"];
    let m = mocked_server
        .mock("get", "/ping")
        .match_query(Matcher::Regex("message=hello".into()))
        .with_status(200)
        .with_body(serde_json::to_string(&unformable_api_resp).unwrap())
        .expect(2)
        .create_async()
        .await;
    let resp = indexer_client.ping(ping_message.clone()).await;
    assert!(resp.is_err());
    let resp2 = indexer_client.ping(ping_message.clone()).await;
    assert_eq!(resp, resp2);
    m.assert_async().await;
}

#[tokio::test]
async fn test_with_http_response_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
        mocked_api_resp,
        indexer_client,
    } = setup().await.unwrap();
    let m = mocked_server
        .mock("get", "/ping")
        .match_query(Matcher::Regex("message=hello".into()))
        .with_status(500)
        .with_body(serde_json::to_string(&mocked_api_resp).unwrap())
        .create_async()
        .await;
    let resp = indexer_client.ping(ping_message.clone()).await;
    assert!(resp.is_err());
    assert_eq!(
        resp.err(),
        Some(ClientError::HttpResponseError {
            code: 500,
            message: String::from("any message")
        })
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_with_api_response_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
        mocked_api_resp: _,
        indexer_client,
    } = setup().await.unwrap();
    let error_api_resp = ApiResponse {
        code: -1,
        result: String::from("test error message"),
    };
    let m = mocked_server
        .mock("get", "/ping")
        .match_query(Matcher::Regex("message=hello".into()))
        .with_status(200)
        .with_body(serde_json::to_string(&error_api_resp).unwrap())
        .create_async()
        .await;
    let resp = indexer_client.ping(ping_message.clone()).await;
    assert!(resp.is_err());
    m.assert_async().await;
}

fn func_with_unknow_error(test_opt: Option<String>) -> Result<String, ClientError> {
    match test_opt {
        Some(s) => Ok(s),
        None => Err(ClientError::UnknowError(String::from("test error"))),
    }
}

#[tokio::test]
async fn test_with_unknow_err() {
    let resp = func_with_unknow_error(None);
    assert!(resp.is_err());
    assert_eq!(
        resp.err(),
        Some(ClientError::UnknowError(String::from("test error")))
    );
    let resp2 = func_with_unknow_error(Some(String::from("hello")));
    assert!(resp2.is_ok());
    assert_eq!(resp2.unwrap(), String::from("hello"));
}

fn func_with_api_response_error_eq(test_opt: Option<String>) -> Result<String, ClientError> {
    match test_opt {
        Some(s) => Ok(s),
        None => Err(ClientError::ApiResponseError {
            code: 1,
            message: String::from("test api response"),
        }),
    }
}

#[test]
fn test_with_api_response_error_eq() {
    let resp = func_with_api_response_error_eq(None);
    assert!(resp.is_err());
    assert_eq!(
        resp.err(),
        Some(ClientError::ApiResponseError {
            code: 1,
            message: String::from("any api response"),
        })
    );
    let resp2 = func_with_api_response_error_eq(Some(String::from("hello")));
    assert!(resp2.is_ok());
    assert_eq!(resp2.unwrap(), String::from("hello"));
    let resp3 = func_with_api_response_error_eq(None);
    assert!(resp3.is_err());
    assert_ne!(
        resp3.err(),
        Some(ClientError::UnknowError(String::from("unknow")))
    );
}
