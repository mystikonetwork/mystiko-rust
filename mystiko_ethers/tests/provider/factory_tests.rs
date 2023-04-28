use crate::provider::common::{MockWebSocketResponse, MockWebSocketServer};
use ethers_providers::{Authorization, HttpRateLimitRetryPolicy, Middleware, Quorum};
use mystiko_ethers::provider::factory::{DefaultProviderFactory, ProviderFactory, ProvidersOptions};
use mystiko_ethers::provider::types::{ProviderOptions, QuorumProviderOptions};
use std::time::Duration;

#[tokio::test]
async fn test_default_factory_create_failover() {
    let factory = DefaultProviderFactory::new();
    let ws_url = MockWebSocketServer::new(vec![MockWebSocketResponse::Stall(
        String::from("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xdeadbeef\"}"),
        200,
    )])
    .start(29099)
    .await
    .unwrap();
    let mut http_server = mockito::Server::new_async().await;
    let mock_path = http_server
        .mock("post", "/")
        .with_body("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}")
        .create_async()
        .await;
    let provider_options1 = ProviderOptions::builder()
        .url(ws_url)
        .request_timeout(Duration::from_millis(20))
        .build();
    let provider_options2 = ProviderOptions::builder()
        .url(http_server.url())
        .request_timeout(Duration::from_millis(500))
        .authorization(Authorization::basic(String::from("user"), String::from("pass")))
        .timeout_retries(1)
        .rate_limit_retries(2)
        .initial_backoff(Duration::from_millis(100))
        .compute_units_per_second(200)
        .build();
    let provider_options3 = ProviderOptions::builder()
        .url(String::from("http://localhost:8546"))
        .http_retry_policy(Box::<HttpRateLimitRetryPolicy>::default())
        .build();
    let providers_options = ProvidersOptions::Failover(vec![provider_options1, provider_options2, provider_options3]);
    let provider = factory.create_provider(providers_options).await.unwrap();
    assert_eq!(provider.get_block_number().await.unwrap().as_u64(), 0xbaadbabe);
    mock_path.assert_async().await;
}

#[tokio::test]
async fn test_default_factory_create_quorum() {
    let ws_url = MockWebSocketServer::new(vec![MockWebSocketResponse::Normal(String::from(
        "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xdeadbeef\"}",
    ))])
    .start(29100)
    .await
    .unwrap();
    let mut server1 = mockito::Server::new_async().await;
    let mut server2 = mockito::Server::new_async().await;
    let mut server3 = mockito::Server::new_async().await;
    let mock_path1 = server1
        .mock("post", "/")
        .with_body("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}")
        .create_async()
        .await;
    let mock_path2 = server2
        .mock("post", "/")
        .with_body("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}")
        .create_async()
        .await;
    let mock_path3 = server3.mock("post", "/").with_status(500).create_async().await;
    let provider_options1 = ProviderOptions::builder().quorum_weight(2).url(server1.url()).build();
    let provider_options2 = ProviderOptions::builder().quorum_weight(2).url(server2.url()).build();
    let provider_options3 = ProviderOptions::builder().quorum_weight(2).url(ws_url).build();
    let provider_options4 = ProviderOptions::builder().url(server3.url()).build();
    let quorum_options = QuorumProviderOptions::builder().quorum(Quorum::Majority).build();
    let providers_options = ProvidersOptions::Quorum(
        vec![
            provider_options1,
            provider_options2,
            provider_options3,
            provider_options4,
        ],
        quorum_options,
    );
    let factory = DefaultProviderFactory::new();
    let provider = factory.create_provider(providers_options).await.unwrap();
    assert_eq!(provider.get_block_number().await.unwrap().as_u64(), 0xbaadbabe);
    mock_path1.assert_async().await;
    mock_path2.assert_async().await;
    mock_path3.assert_async().await;
}

#[tokio::test]
async fn test_default_factory_create_http() {
    let mut server = mockito::Server::new_async().await;
    let mock_path = server
        .mock("post", "/")
        .with_body("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}")
        .create_async()
        .await;
    let provider_options = ProviderOptions::builder()
        .url(server.url())
        .authorization(Authorization::basic(String::from("user"), String::from("pass")))
        .timeout_retries(1)
        .rate_limit_retries(2)
        .initial_backoff(Duration::from_millis(100))
        .compute_units_per_second(200)
        .build();
    let providers_options = ProvidersOptions::Http(provider_options);
    let factory = DefaultProviderFactory::new();
    let provider = factory.create_provider(providers_options).await.unwrap();
    assert_eq!(provider.get_block_number().await.unwrap().as_u64(), 0xbaadbabe);
    mock_path.assert_async().await;
}

#[tokio::test]
async fn test_default_factory_create_ws() {
    let ws_url = MockWebSocketServer::new(vec![MockWebSocketResponse::Normal(String::from(
        "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}",
    ))])
    .start(29101)
    .await
    .unwrap();
    let provider_options = ProviderOptions::builder()
        .url(ws_url)
        .request_timeout(Duration::from_millis(500))
        .build();
    let providers_options = ProvidersOptions::Ws(provider_options);
    let factory = DefaultProviderFactory::new();
    let provider = factory.create_provider(providers_options).await.unwrap();
    assert_eq!(provider.get_block_number().await.unwrap().as_u64(), 0xbaadbabe);
}
