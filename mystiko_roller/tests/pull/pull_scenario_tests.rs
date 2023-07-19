use crate::pull::{create_mock_indexer_server, create_pull_handle};
use crate::test_files::load::load_indexer_commitments;
use ethers_core::types::{Log, U64};
use mystiko_roller::chain::provider::ProviderStub;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::context::ContextTrait;
use std::sync::Arc;

#[tokio::test]
pub async fn test_block_number_slow() {
    let test_chain_id = 211;
    let (handle, data, c) = create_pull_handle(test_chain_id, true).await;
    let stub_provider = Arc::new(ProviderStub::new(&handle.contract_address, c.provider()));
    let mock = c.mock_provider().await;
    let block_number1 = U64::from("0x100");
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push(block_number1).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        block_number1.as_u64() + 1,
        "next sync block should be 257"
    );

    let block_number2 = U64::from("0x10");
    mock.push(block_number2).unwrap();
    let result = handle.pull(stub_provider.clone()).await;
    assert!(result.is_ok());
    assert_eq!(
        data.read().await.get_next_sync_block(),
        block_number1.as_u64() + 1,
        "next sync block should be 257"
    );
}

#[tokio::test]
pub async fn test_leaf_index_mismatch() {
    let test_chain_id = 212;
    let (handle, data, c) = create_pull_handle(test_chain_id, false).await;
    let mut cms = load_indexer_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(&handle.contract_address),
    )
    .await;
    cms[1].leaf_index = 2;
    let block_number = 128;
    let cm_count: usize = 3;
    let (cms_rsp, _) = cms.split_at(cm_count);
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, cms_rsp).await;
    let result = handle.pull(c.indexer().unwrap()).await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentMissing));
    assert_eq!(
        data.read().await.get_next_sync_block(),
        cms[0].block_num,
        "next sync block error"
    );
    assert_eq!(
        data.read().await.get_commitments_queue_count(),
        1,
        "commitments mismatch"
    );
    assert_eq!(data.read().await.get_included_count(), 0, "included count mismatch");
    std::mem::drop(server);

    let block_number = cms[0].block_num + 1;
    let cms = load_indexer_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(&handle.contract_address),
    )
    .await;
    let cm_count: usize = 3;
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
}
