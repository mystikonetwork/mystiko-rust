use crate::context::mock_context::{
    create_mock_context, get_pool_contracts, indexer_server_port, token_price_server_port, MockContext,
};
use crate::rollup::rollup_test_data::{get_proof, get_transaction, get_transaction_receipt};
use crate::test_files::load::load_commitments;
use ethers_core::types::{Bytes, H256, U256};
use httptest::responders::json_encoded;
use httptest::{matchers::*, Expectation, Server, ServerBuilder};
use mystiko_fs::read_file_bytes;
use mystiko_indexer_client::response::ApiResponse;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::handler::{DataHandler, RollupPlan};
use mystiko_roller::rollup::handler::RollupHandle;
use mystiko_server_utils::token_price::query::CurrencyQuoteResponse;
use serde_json::json;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
pub async fn test_rollup_with_provider() {
    let chain_id = 301;
    let (mut handle, data, c) = create_rollup_handle(chain_id, true).await;
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::ContractCallError(_)));

    let mock = c.mock_provider().await;
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentQueueSlow));

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, cms2) = cms.split_at(1);
    data.write().await.insert_commitments(cms1).await;
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(result.is_ok());
    assert_eq!(data.read().await.get_commitments_queue_count(), 1);
    assert_eq!(data.read().await.get_included_count(), 0);

    let (cms3, _) = cms2.split_at(1);
    data.write().await.insert_commitments(cms3).await;
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));
}

#[tokio::test]
pub async fn test_rollup_with_indexer() {
    let test_chain_id = 302;
    let (mut handle, data, c) = create_rollup_handle(test_chain_id, false).await;
    let result = handle.rollup().await;
    assert!(matches!(result.err().unwrap(), RollerError::ContractCallError(_)));

    let server = create_mock_indexer_server(test_chain_id, handle.pool_contract_cfg.address(), 3).await;
    let mock = c.mock_provider().await;
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
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
    data.write().await.insert_commitments(cms1).await;
    let server = create_mock_indexer_server(test_chain_id, handle.pool_contract_cfg.address(), 3).await;
    let result = handle.rollup().await;
    assert!(result.is_ok());
    std::mem::drop(server);
}

#[tokio::test]
pub async fn test_rollup_send_transaction() {
    let test_chain_id = 1;
    let (mut handle, _, c) = create_rollup_handle(test_chain_id, true).await;
    let plan = RollupPlan {
        sizes: vec![1],
        total_fee: U256::from("10000000000000000"),
        force: false,
    };

    let proof = get_proof();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));

    let nonce = U256::from(100);
    let price = U256::from(1000000);
    let mock = c.mock_provider().await;
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));

    let token_price_server = create_mock_token_price_server(test_chain_id).await;
    let gas = U256::from(100_000_000_000u64);
    let nonce = U256::from(100);
    let price = U256::from(1000000);
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));

    let transaction = get_transaction();
    let transaction_receipt = get_transaction_receipt();
    let tx_hash = H256::from_str("0x090b19818d9d087a49c3d2ecee4829ee4acea46089c1381ac5e588188627466d").unwrap();
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(price).unwrap();
    mock.push(price).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(result.is_ok());

    let plan = RollupPlan {
        sizes: vec![1],
        total_fee: U256::from("10000000000000000"),
        force: true,
    };
    mock.push(price).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(matches!(result.err().unwrap(), RollerError::TxManagerError(_)));

    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(price).unwrap();
    mock.push(price).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert!(result.is_ok());
    std::mem::drop(token_price_server);
}

async fn create_mock_indexer_server(chain_id: u64, contract_address: &str, included_count: u32) -> Server {
    let addr = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        (indexer_server_port(chain_id)) as u16,
    );
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();
    let included_count_path = format!(
        "/chains/{}/address/{}/count/commitment-included",
        chain_id, contract_address
    );

    let included_count_rsp = ApiResponse {
        code: 0,
        result: included_count,
    };
    let included_count_json = json_encoded(json!(included_count_rsp));

    server.expect(Expectation::matching(request::path(matches(included_count_path))).respond_with(included_count_json));
    server
}

async fn create_mock_token_price_server(chain_id: u64) -> Server {
    let indexer_port = indexer_server_port(chain_id);
    let token_price_port = token_price_server_port(indexer_port);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), (token_price_port) as u16);
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();

    let id_bytes = read_file_bytes("./../mystiko_server_utils/tests/token_price/files/token_price.json")
        .await
        .unwrap();
    let currency_quote: CurrencyQuoteResponse = serde_json::from_slice(&id_bytes).unwrap();
    let resp_json = json_encoded(currency_quote);
    server.expect(
        Expectation::matching(request::method_path("GET", "/v2/cryptocurrency/quotes/latest")).respond_with(resp_json),
    );
    server
}

async fn create_rollup_handle(
    chain_id: u64,
    disable_indexer: bool,
) -> (RollupHandle, Arc<RwLock<DataHandler>>, Arc<MockContext>) {
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
    let mut handle = RollupHandle::new(&pool_contract, context_trait2, Arc::clone(&data_rc)).await;
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}
