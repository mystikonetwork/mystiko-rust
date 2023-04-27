use crate::provider::common::{MockWebSocketResponse, MockWebSocketServer};
use ethers_providers::{Middleware, Provider};
use mystiko_ethers::provider::ws::WsWithTimeout;
use std::time::Duration;

#[tokio::test]
async fn test_websocket_normal_request() {
    let responses = vec![MockWebSocketResponse::Normal(String::from(
        "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xdeadbeef\"}",
    ))];
    let url = MockWebSocketServer::new(responses)
        .start(19099)
        .await
        .unwrap();
    let ws_provider = WsWithTimeout::connect(url, None).await.unwrap();
    let provider = Provider::new(ws_provider);
    assert_eq!(
        provider.get_block_number().await.unwrap().as_u64(),
        0xdeadbeef
    );
}

#[tokio::test]
async fn test_websocket_jsonrpc_error() {
    let responses = vec![MockWebSocketResponse::Normal(String::from(
        "{\"jsonrpc\":\"2.0\",\"id\":0,\
            \"error\": {\"code\": -1, \"message\": \"something went wrong\"}}",
    ))];
    let url = MockWebSocketServer::new(responses)
        .start(19100)
        .await
        .unwrap();
    let ws_provider = WsWithTimeout::connect(url, None).await.unwrap();
    let provider = Provider::new(ws_provider);
    assert!(provider.get_block_number().await.is_err());
}

#[tokio::test]
async fn test_websocket_timeout() {
    let responses = vec![MockWebSocketResponse::Stall(
        String::from("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xdeadbeef\"}"),
        300,
    )];
    let url = MockWebSocketServer::new(responses)
        .start(19101)
        .await
        .unwrap();
    let ws_provider =
        WsWithTimeout::connect_with_reconnects(url, 0, Some(Duration::from_millis(50)))
            .await
            .unwrap();
    let provider = Provider::new(ws_provider);
    let error = provider.get_block_number().await.err().unwrap();
    assert_eq!(error.to_string(), "IO error: timed out after 50ms");
}
