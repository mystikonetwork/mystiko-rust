use crate::context::mock_context::{create_mock_context, get_pool_contracts, indexer_server_port, MockContext};
use httptest::matchers::{matches, request};
use httptest::responders::json_encoded;
use httptest::{Expectation, Server, ServerBuilder};
use mystiko_indexer_client::response::ApiResponse;
use mystiko_indexer_client::types::commitment_queued::CommitmentQueuedResponse;
use mystiko_indexer_client::types::sync_response::ContractSyncResponse;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::handler::DataHandler;
use mystiko_roller::pull::handler::PullHandle;
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod pull_scenario_tests;
pub mod pull_tests;

pub async fn create_pull_handle(
    chain_id: u64,
    disable_indexer: bool,
) -> (PullHandle, Arc<RwLock<DataHandler>>, Arc<MockContext>) {
    let mut c = create_mock_context(indexer_server_port(chain_id)).await;
    if disable_indexer {
        c.disable_indexer();
    }

    let c = Arc::new(c);
    let pool_contract = get_pool_contracts(&c);

    let context_trait: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let data = DataHandler::new(chain_id, &pool_contract, context_trait).await;
    let data_rc = Arc::new(RwLock::new(data));
    let context_trait2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let mut handle = PullHandle::new(pool_contract.address(), context_trait2, Arc::clone(&data_rc));
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}

pub async fn create_mock_indexer_server(
    chain_id: u64,
    contract_address: &str,
    block_number: u64,
    cms: &[CommitmentQueuedResponse],
) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), indexer_server_port(chain_id));
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
    let cms_rsp = ApiResponse { code: 0, result: cms };
    let cms_rsp_json = json_encoded(json!(cms_rsp));
    server.expect(Expectation::matching(request::path(matches(block_number_path))).respond_with(block_number_json));
    server.expect(Expectation::matching(request::path(matches(queued_commitment_path))).respond_with(cms_rsp_json));
    server
}
