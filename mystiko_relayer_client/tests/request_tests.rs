use http::response::Builder;
use mystiko_relayer_client::request::handle_response;
use mystiko_relayer_types::response::ApiResponse;
use reqwest::Response;

#[tokio::test]
async fn test_handle_success_response() {
    let body = ApiResponse {
        code: 0,
        data: Some("None".to_string()),
        message: None,
    };
    let http_response = Builder::new()
        .header("content-type", "application/json")
        .status(200)
        .body(body)
        .unwrap();
    let response = Response::from(http_response);

    let result = handle_response::<String>(response).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_error_response() {
    let body = ApiResponse {
        code: -1,
        data: Some("None".to_string()),
        message: Some("error message".to_string()),
    };
    let http_response = Builder::new()
        .header("content-type", "application/json")
        .status(200)
        .body(body)
        .unwrap();
    let response = Response::from(http_response);

    let result = handle_response::<String>(response).await;
    assert!(result.is_err());
}
