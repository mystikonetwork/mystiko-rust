use mockito::*;
use mystiko_indexer_client::{ApiResponse, ClientError, IndexerClient};

struct TestErrorSetupData {
    mocked_server: mockito::ServerGuard,
    ping_message: String,
    indexer_client: IndexerClient,
}

fn create_indexer_client(base_url: &str) -> IndexerClient {
    IndexerClient::builder(base_url).build().unwrap()
}

async fn setup() -> Result<TestErrorSetupData, Error> {
    let mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let ping_message = String::from("hello");
    let indexer_client = create_indexer_client(&mocked_server_url);
    Ok(TestErrorSetupData {
        mocked_server,
        ping_message,
        indexer_client,
    })
}

#[tokio::test]
async fn test_with_reqwest_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
        indexer_client,
    } = setup().await.unwrap();
    let unformable_api_resp = vec!["111"];
    let m = mocked_server
        .mock("get", "/ping")
        .match_query(Matcher::Regex("message=hello".into()))
        .with_status(200)
        .with_body(serde_json::to_string(&unformable_api_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(2)
        .create_async()
        .await;
    let resp = indexer_client.ping(&ping_message).await;
    assert!(resp.is_err());
    let err = resp.unwrap_err();
    assert!(err.downcast_ref::<reqwest::Error>().is_some());
    let resp2 = indexer_client.ping(&ping_message).await;
    assert_eq!(err.to_string(), resp2.unwrap_err().to_string());
    m.assert_async().await;
}

#[tokio::test]
async fn test_with_api_response_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
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
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let resp = indexer_client.ping(&ping_message).await;
    assert!(resp.is_err());
    let err = resp.unwrap_err();
    assert!(err.downcast_ref::<ClientError>().is_some());
    assert_eq!(
        err.to_string(),
        ClientError::ApiResponseError {
            code: -1,
            message: String::from("\"test error message\"")
        }
        .to_string()
    );
    m.assert_async().await;
}

fn func_with_custom_error(test_opt: Option<String>) -> Result<String, ClientError> {
    match test_opt {
        Some(s) => Ok(s),
        None => Err(ClientError::CustomError(String::from("test error"))),
    }
}

#[tokio::test]
async fn test_with_content_type_err() {
    let TestErrorSetupData {
        mut mocked_server,
        ping_message,
        indexer_client,
    } = setup().await.unwrap();
    let error_api_resp = ApiResponse {
        code: 0,
        result: String::from("test error message"),
    };
    let m = mocked_server
        .mock("get", "/ping")
        .match_query(Matcher::Regex("message=hello".into()))
        .with_status(200)
        .with_body(serde_json::to_string(&error_api_resp).unwrap())
        .create_async()
        .await;
    let resp = indexer_client.ping(&ping_message).await;
    assert!(resp.is_err());
    let err = resp.unwrap_err();
    assert!(err.downcast_ref::<ClientError>().is_some());
    assert_eq!(
        err.to_string(),
        ClientError::UnsupportedContentTypeError(String::from("")).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_with_custom_err() {
    let resp = func_with_custom_error(None);
    assert!(resp.is_err());
    assert_eq!(resp.err(), Some(ClientError::CustomError(String::from("test error"))));
    let resp2 = func_with_custom_error(Some(String::from("hello")));
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
    assert_ne!(resp3.err(), Some(ClientError::CustomError(String::from("unknow"))));
}
