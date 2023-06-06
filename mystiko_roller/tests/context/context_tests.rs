use crate::common::env_tests::ENV_CONFIG_PATH_MUTEX;
use crate::context::mock_context::create_mock_context;
use ethers_core::types::U64;
use ethers_providers::Middleware;
use mystiko_roller::context::{Context, ContextTrait};
use std::env;

pub fn do_evn_init() {
    let _guard = ENV_CONFIG_PATH_MUTEX.lock().unwrap();

    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/2");
    env::set_var(
        "MYSTIKO_ROLLER_CIRCUITS_PATH",
        "./../mystiko_circuits/dist/zokrates/dev",
    );
    env::set_var("MYSTIKO_ROLLER_DATA_PATH", "./tests/db");

    env::set_var("MYSTIKO_ROLLER_X_SCAN_API_KEY", "x_scan_api_key");
    env::set_var("MYSTIKO_ROLLER_COIN_MARKET_CAP_API_KEY", "coin_market_api_key");
    env::set_var(
        "MYSTIKO_ROLLER_PRIVATE_KEY",
        "0x3c87ede2a7d0a38de10b522ec6a7a3ea73be79e469c2aae38b5e0030131a9afa",
    );
}

#[tokio::test]
pub async fn test_context_new() {
    do_evn_init();
    let _ = Context::new().await.unwrap();
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
