extern crate ethers_contract;
extern crate httptest;

use crate::pull::{create_mock_indexer_server, create_pull_handle};
use crate::test_files::load::{load_commitment_logs, load_indexer_commitments};
use ethers_core::types::{Log, U64};
use mystiko_roller::chain::provider::ProviderStub;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::context::ContextTrait;
use std::sync::Arc;

#[tokio::test]
pub async fn test_pull_from_provider() {
    let test_chain_id = 201;
    let (handle, data, c) = create_pull_handle(test_chain_id, true).await;
    let stub_provider = Arc::new(ProviderStub::new(&handle.contract_address, c.provider()));
    let result = handle.pull(stub_provider.clone()).await;
    assert!(matches!(result.err().unwrap(), RollerError::ProviderError(_)));
    assert_eq!(
        data.read().await.get_next_sync_block(),
        0,
        "next sync block should be 0"
    );

    let mock = c.mock_provider().await;
    let block_number = U64::from("0x100");
    mock.push(block_number).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(matches!(result.err().unwrap(), RollerError::ProviderError(_)));
    assert_eq!(
        data.read().await.get_next_sync_block(),
        0,
        "next sync block should be 0"
    );

    data.write().await.update_next_sync_block(10);
    let block_number = U64::from(2);
    mock.push(block_number).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(result.is_ok());

    let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
    let block_number = U64::from(256);
    mock.push::<Vec<Log>, _>(vec![logs[0].clone()]).unwrap();
    mock.push(block_number).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        256 + 1,
        "next sync block should be 0"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        1,
        "commitments mismatch"
    );

    let block_number = U64::from(512);
    mock.push::<Vec<Log>, _>(logs.clone()).unwrap();
    mock.push(block_number).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        512 + 1,
        "next sync block should be 0"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        logs.len(),
        "commitments mismatch"
    );
}

#[tokio::test]
pub async fn test_pull_from_indexer() {
    let test_chain_id = 202;
    let (handle, data, c) = create_pull_handle(test_chain_id, false).await;

    let block_number = 10;
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, &[]).await;
    let result = handle.pull(c.indexer().unwrap()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        block_number + 1,
        "next sync block error"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        0,
        "commitments mismatch"
    );
    assert_eq!(data.read().await.get_included_count(), 0, "included count mismatch");
    std::mem::drop(server);

    let cms = load_indexer_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(&handle.contract_address),
    )
    .await;
    let block_number = 128;
    let cm_count: usize = 2;
    let (cms_rsp, _) = cms.split_at(cm_count);
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, cms_rsp).await;
    let result = handle.pull(c.indexer().unwrap()).await;
    assert!(result.is_ok(), "pull failed {:?}", result.err());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        block_number + 1,
        "next sync block error"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        cm_count,
        "commitments mismatch"
    );
    assert_eq!(data.read().await.get_included_count(), 0, "included count mismatch");
    std::mem::drop(server);

    let cms = load_indexer_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(&handle.contract_address),
    )
    .await;
    let block_number = 256;
    let cm_count: usize = 18;
    let (cms_rsp, _) = cms.split_at(cm_count);
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, cms_rsp).await;
    let result = handle.pull(c.indexer().unwrap()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        block_number + 1,
        "next sync block error"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        cm_count,
        "commitments mismatch"
    );
    assert_eq!(data.read().await.get_included_count(), 0, "included count mismatch");
    std::mem::drop(server);
}
