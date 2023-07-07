use crate::context::mock_context::{
    create_mock_context, get_pool_contracts, indexer_server_port, token_price_server_port, MockContext,
};
use crate::rollup::rollup_test_data::{get_proof, get_transaction, get_transaction_receipt};
use crate::test_files::load::load_commitments;
use ethers_core::types::{Bytes, H256, U256, U64};
use httptest::responders::json_encoded;
use httptest::{matchers::*, Expectation, Server, ServerBuilder};
use mystiko_fs::read_file_bytes;
use mystiko_indexer_client::response::ApiResponse;
use mystiko_indexer_client::types::sync_response::ContractSyncResponse;
use mystiko_roller::chain::provider::ProviderStub;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::config::roller::create_tx_manager_config;
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
    let (handle, data, c) = create_rollup_handle(chain_id, true).await;
    let stub_provider = Arc::new(ProviderStub::new(handle.pool_contract_cfg.address(), c.provider()));
    let result = handle.rollup(stub_provider.clone()).await;
    assert!(matches!(result.err().unwrap(), RollerError::ProviderError(_)));

    let mock = c.mock_provider().await;
    let block_number = U64::from("0x1");
    mock.push(block_number).unwrap();
    let result = handle.rollup(stub_provider.clone()).await;
    assert!(result.is_ok());

    let mock = c.mock_provider().await;
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    let block_number = U64::from("0x100");
    mock.push(block_number).unwrap();
    let result = handle.rollup(stub_provider.clone()).await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentQueueSlow));

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, cms2) = cms.split_at(1);
    data.write().await.insert_commitments(cms1).await.unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    mock.push(block_number).unwrap();
    let result = handle.rollup(stub_provider.clone()).await;
    assert!(result.is_ok());
    assert_eq!(data.read().await.get_commitments_queue_count(), 1);
    assert_eq!(data.read().await.get_included_count(), 0);

    let (cms3, _) = cms2.split_at(1);
    data.write().await.insert_commitments(cms3).await.unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    mock.push(block_number).unwrap();
    let result = handle.rollup(stub_provider.clone()).await;
    assert!(matches!(result.err().unwrap(), RollerError::ProtocolError(_)));
}

#[tokio::test]
pub async fn test_rollup_with_indexer() {
    let test_chain_id = 302;
    let (handle, data, c) = create_rollup_handle(test_chain_id, false).await;
    let result = handle.rollup(c.indexer().unwrap()).await;
    assert!(matches!(result.err().unwrap(), RollerError::AnyhowError(_)));

    let server = create_mock_indexer_server(test_chain_id, handle.pool_contract_cfg.address(), Some(2), Some(1)).await;
    let mock = c.mock_provider().await;
    let include_count = Bytes::from_str("0000000000000000000000000000000000000000000000000000000000000001").unwrap();
    mock.push::<Bytes, _>(include_count.clone()).unwrap();
    let result = handle.rollup(c.indexer().unwrap()).await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentQueueSlow));
    std::mem::drop(server);

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, _) = cms.split_at(3);
    data.write().await.insert_commitments(cms1).await.unwrap();

    let server = create_mock_indexer_server(test_chain_id, handle.pool_contract_cfg.address(), Some(2), Some(2)).await;
    let result = handle.rollup(c.indexer().unwrap()).await;
    assert!(result.is_err());
    std::mem::drop(server);
}

#[tokio::test]
pub async fn test_rollup_with_indexer2() {
    let test_chain_id = 303;
    let (handle, data, c) = create_rollup_handle(test_chain_id, false).await;

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(handle.pool_contract_cfg.address()),
    )
    .await;
    let (cms1, _) = cms.split_at(3);
    data.write().await.insert_commitments(cms1).await.unwrap();

    let server = create_mock_indexer_server(test_chain_id, handle.pool_contract_cfg.address(), Some(2), Some(3)).await;
    let result = handle.rollup(c.indexer().unwrap()).await;
    assert!(result.is_ok());
    std::mem::drop(server);
}

#[tokio::test]
pub async fn test_rollup_send_transaction() {
    let test_chain_id = 1;
    let (handle, _, c) = create_rollup_handle(test_chain_id, true).await;
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
    let block_number = U64::from(6203183);
    let transaction_receipt = get_transaction_receipt();
    let tx_hash = H256::from_str("0x090b19818d9d087a49c3d2ecee4829ee4acea46089c1381ac5e588188627466d").unwrap();
    mock.push(transaction_receipt.clone()).unwrap();
    mock.push(block_number).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(price).unwrap();
    mock.push(price).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert_eq!(result.unwrap(), tx_hash.to_string());

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
    mock.push(block_number).unwrap();
    mock.push(transaction.clone()).unwrap();
    mock.push(tx_hash).unwrap();
    mock.push(price).unwrap();
    mock.push(price).unwrap();
    mock.push(gas).unwrap();
    mock.push(nonce).unwrap();
    mock.push(price).unwrap();
    let result = handle.send_rollup_transaction(&plan, &proof).await;
    assert_eq!(result.unwrap(), tx_hash.to_string());
    std::mem::drop(token_price_server);
}

#[tokio::test]
pub async fn test_rollup_log_transaction() {
    let test_chain_id = 305;
    let (handle, _, _) = create_rollup_handle(test_chain_id, false).await;
    handle.log_rollup_transaction("", 1, 1).await;
}

#[tokio::test]
#[should_panic(expected = "unexpected estimate gas error")]
pub async fn test_commitment_queue_check_by_transaction() {
    let test_chain_id = 306;
    let (handle, _, c) = create_rollup_handle(test_chain_id, false).await;
    let result = handle.commitment_queue_check_by_transaction().await;
    assert!(result.is_ok());

    let mock = c.mock_provider().await;
    let nonce = U256::from(100);
    let gas_price = U256::from(1000000);
    mock.push(gas_price).unwrap();
    mock.push(nonce).unwrap();
    let _ = handle.commitment_queue_check_by_transaction().await;
}

#[tokio::test]
#[should_panic(expected = "must error when check queue")]
pub async fn test_commitment_queue_check_by_transaction2() {
    let test_chain_id = 307;
    let (handle, _, c) = create_rollup_handle(test_chain_id, false).await;

    let mock = c.mock_provider().await;
    let gas = U256::from(100_000_000_000u64);
    let nonce = U256::from(100);
    let gas_price = U256::from(1000000);
    mock.push(gas).unwrap();
    mock.push(gas_price).unwrap();
    mock.push(nonce).unwrap();
    let _ = handle.commitment_queue_check_by_transaction().await;
}

async fn create_mock_indexer_server(
    chain_id: u64,
    contract_address: &str,
    block_number: Option<u64>,
    included_count: Option<u32>,
) -> Server {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), indexer_server_port(chain_id));
    let server = ServerBuilder::new().bind_addr(addr).run().unwrap();
    let block_number_path = format!("/chains/{}/contracts/{}/block-number", chain_id, contract_address);
    let included_count_path = format!(
        "/chains/{}/address/{}/count/commitment-included",
        chain_id, contract_address
    );

    if let Some(number) = block_number {
        let block_number_rsp = ApiResponse {
            code: 0,
            result: ContractSyncResponse {
                chain_id: Some(chain_id),
                contract_address: contract_address.to_string(),
                current_sync_block_num: number,
                current_sync_time: None,
            },
        };
        let block_number_json = json_encoded(json!(block_number_rsp));
        server.expect(Expectation::matching(request::path(matches(block_number_path))).respond_with(block_number_json));
    }

    if let Some(included) = included_count {
        let included_count_rsp = ApiResponse {
            code: 0,
            result: included,
        };
        let included_count_json = json_encoded(json!(included_count_rsp));
        server.expect(
            Expectation::matching(request::path(matches(included_count_path))).respond_with(included_count_json),
        );
    }

    server
}

async fn create_mock_token_price_server(chain_id: u64) -> Server {
    let indexer_port = indexer_server_port(chain_id);
    let token_price_port = token_price_server_port(indexer_port);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), token_price_port);
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
    let tx_manager_cfg = create_tx_manager_config("testnet", "tests/test_files/config/base").unwrap();
    let context_trait: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let data = DataHandler::new(chain_id, &pool_contract, context_trait).await;
    let data_rc = Arc::new(RwLock::new(data));
    let context_trait2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let mut handle = RollupHandle::new(&pool_contract, &tx_manager_cfg, context_trait2, Arc::clone(&data_rc)).await;
    handle.chain_id = chain_id;
    (handle, data_rc, c)
}
