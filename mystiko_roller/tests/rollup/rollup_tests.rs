extern crate hex_literal;

use crate::context::mock_context::{create_mock_context, get_pool_contracts, MockContext};
use crate::test_files::load::load_commitments;
use ethers_core::types::Bytes;
use hex_literal::hex;
use httptest::responders::json_encoded;
use httptest::{matchers::*, Expectation, Server, ServerBuilder};
use mystiko_indexer_client::response::ApiResponse;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::data::DataHandle;
use mystiko_roller::rollup::handle::RollupHandle;
use serde_json::json;
use std::cell::RefCell;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::rc::Rc;
use std::sync::Arc;

#[tokio::test]
pub async fn test_rollup_with_provider() {
    let chain_id = 301;
    let (mut handle, data, c) = create_rollup_handle(chain_id, true).await;
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::RpcCallError(_)));

    let mock = c.mock_provider().await;
    let include_count = hex!("0000000000000000000000000000000000000000000000000000000000000001");
    let encoded = Bytes::from(include_count);
    mock.push::<Bytes, _>(encoded.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentQueueSlow));

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, cms2) = cms.split_at(1);
    data.borrow_mut().insert_commitments(&cms1).await;
    mock.push::<Bytes, _>(encoded.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(result.is_ok());
    assert_eq!(data.borrow().get_commitments_queue_count(), 1);
    assert_eq!(data.borrow().get_included_count(), 0);

    let (cms3, _) = cms2.split_at(1);
    data.borrow_mut().insert_commitments(&cms3).await;
    mock.push::<Bytes, _>(encoded).unwrap();
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));
}

#[tokio::test]
pub async fn test_rollup_with_indexer() {
    let test_chain_id = 302;
    let (mut handle, data, c) = create_rollup_handle(test_chain_id, false).await;
    let server = create_mock_indexer_server(test_chain_id, &handle.pool_contract_cfg.address()).await;
    let mock = c.mock_provider().await;
    let include_count = hex!("0000000000000000000000000000000000000000000000000000000000000001");
    let encoded = Bytes::from(include_count);
    mock.push::<Bytes, _>(encoded.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentQueueSlow));
    std::mem::drop(server);

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, _) = cms.split_at(3);
    data.borrow_mut().insert_commitments(&cms1).await;
    let server = create_mock_indexer_server(test_chain_id, &handle.pool_contract_cfg.address()).await;
    let result = handle.rollup().await;
    assert!(result.is_ok());
    std::mem::drop(server);
}

#[tokio::test]
pub async fn test_rollup_send_transaction() {}

async fn create_mock_indexer_server(chain_id: u64, contract_address: &str) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), (chain_id + 20000) as u16);
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();
    let included_count_path = format!(
        "/chains/{}/address/{}/count/commitment-included",
        chain_id, contract_address
    );

    let included_count_rsp = ApiResponse { code: 0, result: 3 };
    let included_count_json = json_encoded(json!(included_count_rsp));

    server.expect(Expectation::matching(request::path(matches(included_count_path))).respond_with(included_count_json));
    server
}

async fn create_rollup_handle(
    chain_id: u64,
    disable_indexer: bool,
) -> (RollupHandle, Rc<RefCell<DataHandle>>, Arc<MockContext>) {
    let mut c = create_mock_context(chain_id + 20000).await;
    if disable_indexer {
        c.disable_indexer();
    }

    let c = Arc::new(c);
    let pool_contract = get_pool_contracts(&c);

    let context_trait: Arc<dyn ContextTrait> = Arc::clone(&c) as Arc<dyn ContextTrait>;
    let data = DataHandle::new(chain_id, &pool_contract, context_trait).await;
    let data_rc = Rc::new(RefCell::new(data));
    let context_trait2: Arc<dyn ContextTrait> = Arc::clone(&c) as Arc<dyn ContextTrait>;
    let mut handle = RollupHandle::new(&pool_contract, context_trait2, Rc::clone(&data_rc)).await;
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}
