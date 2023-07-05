extern crate mockito;

use crate::common::{env_init, ENV_MUTEX};
use crate::context::mock_context::provider_server_port;
use mystiko_roller::context::{Context, ContextTrait};

#[tokio::test]
pub async fn test_context_new() {
    let _guard = ENV_MUTEX.write().await;
    env_init();

    let _ = create_mock_provider_server(provider_server_port()).await;
    let c = Context::new("testnet", "./tests/test_files/config/base").await;
    assert!(c.is_ok());

    let c = Context::new("testnet", "./tests/test_files/config/no_indexer_explorer").await;
    assert!(c.is_ok());
    let c = c.unwrap();
    assert!(c.indexer().is_none());
    assert!(c.chain_explorer().is_none());
}

// todo remove this test
//
// #[tokio::test]
// pub async fn test_mock_context_new() {
//     let c = create_mock_context(20001).await;
//     let provider = c.provider().await.unwrap();
//     let moc = c.mock_provider().await;
//     let block_number = U64::from("0x100");
//     moc.push(block_number).expect("push block number failed");
//     let number = provider.get_block_number().await.unwrap();
//     assert_eq!(number, block_number);
// }

async fn create_mock_provider_server(port: u16) -> mockito::Server {
    let mut http_server = mockito::Server::new_with_port_async(port).await;
    let _ = http_server
        .mock("post", "/")
        .with_body("{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":\"0xbaadbabe\"}")
        .create_async()
        .await;
    http_server
}
