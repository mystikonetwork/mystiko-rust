use crate::context::mock_context::{create_mock_context, indexer_server_port, MockContext};
use crate::test_files::load::{load_commitment_logs, load_commitments};
use ethers_core::types::{Log, U64};
use mystiko_roller::chain::provider::ProviderStub;
use mystiko_roller::config::roller::create_tx_manager_config;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::pool::Pool;
use std::sync::Arc;

#[tokio::test]
pub async fn test_run() {
    let test_chain_id = 401;
    let (pool, _, _) = create_pool_handle(test_chain_id).await;
    let result = pool.run().await;
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn test_run_from_one_source() {
    let test_chain_id = 402;
    let (pool, c, addr) = create_pool_handle(test_chain_id).await;
    let mock = c.mock_provider().await;
    let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
    let block_number = U64::from(256);
    mock.push::<Vec<Log>, _>(vec![logs[0].clone()]).unwrap();
    mock.push(block_number).unwrap();
    let stub_provider = Arc::new(ProviderStub::new(&addr, c.provider()));
    let result = pool.run_from_one_source(stub_provider).await;
    assert!(result.is_err());
}

#[tokio::test]
pub async fn test_check_commitment_queue() {
    let test_chain_id = 403;
    let (pool, _, _) = create_pool_handle(test_chain_id).await;
    let result = pool.check_commitment_queue().await;
    assert!(result.is_ok());

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some("0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411"),
    )
    .await;
    let (cms1, _) = cms.split_at(3);
    pool.data.write().await.insert_commitments(cms1).await.unwrap();
    let result = pool.check_commitment_queue().await;
    assert!(result.is_ok());

    for _ in 0..pool.context.cfg().pull.max_empty_queue_count + 1 {
        pool.data.write().await.inc_empty_queue_counter();
    }
    let result = pool.check_commitment_queue().await;
    assert!(result.is_ok());
}

async fn create_pool_handle(test_chain_id: u64) -> (Pool, Arc<MockContext>, String) {
    let c = create_mock_context(indexer_server_port(test_chain_id)).await;
    let c = Arc::new(c);
    let pool_contracts = c.core_cfg_parser().pool_contracts_cfg(c.cfg().chain.chain_id);
    let tx_manager_cfg = create_tx_manager_config("testnet", "tests/test_files/config/base").unwrap();
    let c2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let pool = Pool::new(0, pool_contracts[0].clone(), &tx_manager_cfg, c2).await;
    (pool, c, pool_contracts[0].address().to_string())
}
