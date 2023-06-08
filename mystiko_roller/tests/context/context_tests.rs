use crate::common::evn_init;
use crate::common::ENV_MUTEX;
use crate::context::mock_context::create_mock_context;
use ethers_core::types::U64;
use ethers_providers::Middleware;
use mystiko_roller::context::{Context, ContextTrait};

#[tokio::test]
pub async fn test_context_new() {
    let _guard = ENV_MUTEX.write().await;
    evn_init();
    let c = Context::new().await;
    assert!(c.is_ok());
}

#[tokio::test]
pub async fn test_mock_context_new() {
    let c = create_mock_context(20001).await;
    let provider = c.provider().await.unwrap();
    let moc = c.mock_provider().await;
    let block_number = U64::from("0x100");
    moc.push(block_number).expect("push block number failed");
    let number = provider.get_block_number().await.unwrap();
    assert_eq!(number, block_number);
}
