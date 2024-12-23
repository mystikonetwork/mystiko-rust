use crate::{extract_data, setup};
use mystiko_lib::synchronizer;
use mystiko_protos::api::synchronizer::v1::{
    ChainSyncedBlockRequest, ChainSyncedBlockResponse, ContractSyncedBlockRequest, ContractSyncedBlockResponse,
    SynchronizerResetRequest, SynchronizerStatusRequest, SynchronizerStatusResponse, SynchronizerSyncRequest,
};
use mystiko_protos::api::v1::status_code::Error;
use mystiko_protos::api::v1::SynchronizerError;
use mystiko_protos::core::synchronizer::v1::{SynchronizerResetOptions, SynchronizerSyncOptions};
use serial_test::serial;

#[test]
#[serial]
fn test_chain_synced_block() {
    setup(None);
    let response = synchronizer::chain_synced_block(ChainSyncedBlockRequest::builder().chain_id(97u64).build());
    assert!(response.code.unwrap().success);
    assert!(ChainSyncedBlockResponse::try_from(extract_data(response.result.unwrap())).is_ok());
}

#[test]
#[serial]
fn test_contract_synced_block() {
    setup(None);
    let response = synchronizer::contract_synced_block(
        ContractSyncedBlockRequest::builder()
            .chain_id(97u64)
            .contract_address("0x")
            .build(),
    );
    assert!(response.code.unwrap().success);
    let result = ContractSyncedBlockResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .result;
    assert!(result.is_none());

    let response = synchronizer::contract_synced_block(
        ContractSyncedBlockRequest::builder()
            .chain_id(1u64)
            .contract_address("0x")
            .build(),
    );
    assert_eq!(
        response.code.unwrap().error.unwrap(),
        Error::Synchronizer(SynchronizerError::UnsupportedChainError as i32)
    );
}

#[test]
#[serial]
fn test_status() {
    setup(None);
    let response = synchronizer::status(SynchronizerStatusRequest::builder().with_contracts(false).build());
    assert!(response.code.unwrap().success);
    let status = SynchronizerStatusResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .status
        .unwrap();
    let mut chains = status.chains;
    chains.sort_by_key(|cs| cs.chain_id);
    assert_eq!(chains.len(), 2);
    assert_eq!(chains[0].chain_id, 5);
    assert_eq!(chains[0].contracts.len(), 0);
    assert_eq!(chains[1].chain_id, 97);
    assert_eq!(chains[1].contracts.len(), 0);

    let response = synchronizer::status(SynchronizerStatusRequest::builder().with_contracts(true).build());
    assert!(response.code.unwrap().success);
    let status = SynchronizerStatusResponse::try_from(extract_data(response.result.unwrap()))
        .unwrap()
        .status
        .unwrap();
    let mut chains = status.chains;
    chains.sort_by_key(|cs| cs.chain_id);
    assert_eq!(chains.len(), 2);
    assert_eq!(chains[0].chain_id, 5);
    assert_eq!(chains[0].contracts.len(), 1);
    assert_eq!(
        chains[0].contracts[0].contract_address,
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(chains[1].chain_id, 97);
    assert_eq!(chains[1].contracts.len(), 1);
    assert_eq!(
        chains[1].contracts[0].contract_address,
        "0xBe2C9c8a00951662dF3a978b25F448968F0595AE"
    );
}

#[test]
#[serial]
#[ignore]
fn test_sync() {
    setup(None);
    let mut sync_options = SynchronizerSyncOptions::builder().build();
    sync_options.fetcher_query_loaded_block_timeout_ms = Some(1000);
    sync_options.fetcher_fetch_timeout_ms = Some(2000);
    sync_options.validator_validate_concurrency = Some(300);
    let response = synchronizer::sync(SynchronizerSyncRequest::builder().options(sync_options).build());
    assert!(response.code.unwrap().success);
}

#[test]
#[serial]
fn test_reset() {
    setup(None);
    let response = synchronizer::reset(
        SynchronizerResetRequest::builder()
            .options(SynchronizerResetOptions::default())
            .build(),
    );
    assert!(response.code.unwrap().success);
}
