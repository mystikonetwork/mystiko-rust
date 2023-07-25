use crate::context::mock_context::{create_mock_context, MockContext};
use crate::test_files::load::load_commitment_logs;
use ethers_core::types::{Log, U64};
use mystiko_roller::chain::provider::ProviderStub;
use mystiko_roller::config::roller::create_tx_manager_config;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::pool::Pool;
use std::sync::Arc;

#[tokio::test]
pub async fn test_run() {
    let (pool, _, _) = create_pool_handle(0, 0).await;
    let result = pool.run().await;
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn test_run_from_one_source() {
    let (pool, c, addr) = create_pool_handle(0, 0).await;
    let mock = c.mock_provider().await;
    let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
    let block_number = U64::from(256);
    mock.push::<Vec<Log>, _>(vec![logs[0].clone()]).unwrap();
    mock.push(block_number).unwrap();
    let stub_provider = Arc::new(ProviderStub::new(&addr, c.provider()));
    let result = pool.run_from_one_source(stub_provider).await;
    assert!(result.is_err());
}

async fn create_pool_handle(indexer_port: u16, token_price_port: u16) -> (Pool, Arc<MockContext>, String) {
    let c = create_mock_context(indexer_port, token_price_port).await;
    let c = Arc::new(c);
    let pool_contracts = c.core_cfg_parser().pool_contracts_cfg(c.cfg().chain.chain_id);
    let tx_manager_cfg = create_tx_manager_config("tests/test_files/config/base").unwrap();
    let c2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let pool = Pool::new(0, pool_contracts[0].clone(), &tx_manager_cfg, c2).await;
    (pool, c, pool_contracts[0].address().to_string())
}
