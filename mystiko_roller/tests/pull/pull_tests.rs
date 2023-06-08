extern crate ethers_contract;
extern crate httptest;

use crate::context::mock_context::{create_mock_context, get_pool_contracts, indexer_server_port, MockContext};
use crate::test_files::load::{load_commitment_logs, load_indexer_commitments};
use ethers_core::types::{Log, U64};
use httptest::{matchers::*, responders::*, Expectation, Server, ServerBuilder};
use mystiko_indexer_client::response::ApiResponse;
use mystiko_indexer_client::types::sync_response::ContractSyncResponse;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::handler::DataHandler;
use mystiko_roller::pull::handler::PullHandle;
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
pub async fn test_pull_from_provider() {
    let test_chain_id = 201;
    let (handle, data, c) = create_pull_handle(test_chain_id, true).await;
    let result = handle.pull().await;
    assert!(matches!(result.err().unwrap(), RollerError::ProviderError(_)));
    assert_eq!(
        data.read().await.get_next_sync_block(),
        0,
        "next sync block should be 0"
    );

    let mock = c.mock_provider().await;
    let block_number = U64::from("0x100");
    mock.push(block_number).unwrap();
    let result = handle.pull().await;
    assert!(matches!(result.err().unwrap(), RollerError::ContractCallError(_)));
    assert_eq!(
        data.read().await.get_next_sync_block(),
        0,
        "next sync block should be 0"
    );

    data.write().await.set_new_next_sync_block(10);
    let block_number = U64::from(2);
    mock.push(block_number).unwrap();
    let result = handle.pull().await;
    assert!(matches!(result.err().unwrap(), RollerError::InvalidStartBlock));

    let logs = load_commitment_logs("tests/test_files/data/commitment_logs.json").await;
    let block_number = U64::from(256);
    mock.push::<Vec<Log>, _>(vec![logs[0].clone()]).unwrap();
    mock.push(block_number).unwrap();
    let result = handle.pull().await;
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
    let result = handle.pull().await;
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
    let (handle, data, _) = create_pull_handle(test_chain_id, false).await;

    let block_number = 128;
    let cm_count: usize = 2;
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, cm_count).await;
    let result = handle.pull().await;
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

    let block_number = 256;
    let cm_count: usize = 18;
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, cm_count).await;
    let result = handle.pull().await;
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

    let block_number = 10;
    let server = create_mock_indexer_server(test_chain_id, &handle.contract_address, block_number, 0).await;
    let result = handle.pull().await;
    assert!(matches!(result.err().unwrap(), RollerError::ProviderError(_)));
    std::mem::drop(server);
}

async fn create_mock_indexer_server(
    chain_id: u64,
    contract_address: &str,
    block_number: u64,
    cm_count: usize,
) -> Server {
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        (indexer_server_port(chain_id)) as u16,
    );
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();
    let block_number_path = format!("/chains/{}/contracts/{}/block-number", chain_id, contract_address);
    let queued_commitment_path = format!(
        "/chains/{}/contracts/{}/events/commitment-queued",
        chain_id, contract_address
    );
    let block_number_rsp = ApiResponse {
        code: 0,
        result: ContractSyncResponse {
            chain_id: Some(chain_id),
            contract_address: contract_address.to_string(),
            current_sync_block_num: block_number,
            current_sync_time: None,
        },
    };
    let block_number_json = json_encoded(json!(block_number_rsp));
    let cms = load_indexer_commitments(
        "tests/test_files/data/commitments.json",
        Some(chain_id),
        Some(contract_address),
    )
    .await;
    let (cms, _) = cms.split_at(cm_count);
    let cms_rsp = ApiResponse { code: 0, result: cms };
    let cms_rsp_json = json_encoded(json!(cms_rsp));
    server.expect(Expectation::matching(request::path(matches(block_number_path))).respond_with(block_number_json));
    if cm_count > 0 {
        server.expect(Expectation::matching(request::path(matches(queued_commitment_path))).respond_with(cms_rsp_json));
    }
    server
}

async fn create_pull_handle(
    chain_id: u64,
    disable_indexer: bool,
) -> (PullHandle, Arc<RwLock<DataHandler>>, Arc<MockContext>) {
    let mut c = create_mock_context(indexer_server_port(chain_id)).await;
    if disable_indexer {
        c.disable_indexer();
    }

    let c = Arc::new(c);
    let pool_contract = get_pool_contracts(&c);

    let context_trait: Arc<dyn ContextTrait> = Arc::clone(&c) as Arc<dyn ContextTrait>;
    let data = DataHandler::new(chain_id, &pool_contract, context_trait).await;
    let data_rc = Arc::new(RwLock::new(data));
    let context_trait2: Arc<dyn ContextTrait> = Arc::clone(&c) as Arc<dyn ContextTrait>;
    let mut handle = PullHandle::new(pool_contract.address(), context_trait2, Arc::clone(&data_rc));
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}
