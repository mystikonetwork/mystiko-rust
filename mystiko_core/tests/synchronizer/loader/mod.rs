mod handler_commitment_dedup_tests;
mod handler_commitment_included_tests;
mod handler_commitment_queued_tests;
mod handler_commitment_src_succeeded_tests;
mod handler_nullifier_tests;
mod handler_query_tests;
mod mock;

use crate::synchronizer::loader::mock::MockSyncDataHandler;
use mystiko_config::MystikoConfig;
use mystiko_core::Deposit;
use mystiko_core::{Database, SyncLoaderHandler};
use mystiko_dataloader::data::FullData;
use mystiko_protos::common::v1::BridgeType;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use num_bigint::BigUint;
use std::sync::Arc;

pub type MystikoDatabase = Database<SqlStatementFormatter, SqliteStorage>;
type FullDataSyncLoaderHandler = SyncLoaderHandler<SqlStatementFormatter, SqliteStorage, MockSyncDataHandler<FullData>>;

pub async fn create_sync_loader_handler(
    mock_database_handler: MockSyncDataHandler<FullData>,
) -> (FullDataSyncLoaderHandler, Arc<MystikoDatabase>, Arc<MystikoConfig>) {
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/mystiko/config.json")
            .await
            .unwrap(),
    );

    let mystiko_db = create_database().await;
    let mystiko_db = Arc::new(mystiko_db);
    let handler = SyncLoaderHandler::builder()
        .mystiko_db(mystiko_db.clone())
        .raw(mock_database_handler)
        .build();
    (handler, mystiko_db, config)
}

pub async fn create_database() -> MystikoDatabase {
    let _ = env_logger::builder()
        .filter_module("mystiko_core", log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    let formatter = SqlStatementFormatter::sqlite();
    let storage = SqliteStorage::from_memory().await.unwrap();
    let mystiko_db = Database::new(formatter, storage);
    let _ = mystiko_db.migrate().await.unwrap();

    mystiko_db
}

pub fn build_deposits(deposit_status: i32, dst_chain_id: u64, count: usize) -> Vec<Deposit> {
    let mut deposits = vec![];
    for i in 0..count {
        let deposit = Deposit {
            chain_id: 5,
            contract_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
            pool_address: String::from("pool_address 1"),
            commitment_hash: BigUint::from(1000 + i as u32),
            hash_k: BigUint::from(11u32),
            random_s: BigUint::from(111u32),
            encrypted_note: String::from("encrypted_note 1"),
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 6,
            asset_address: Some(String::from("asset_address 1")),
            bridge_type: BridgeType::Loop as i32,
            amount: BigUint::from(101u32),
            rollup_fee_amount: BigUint::from(102u32),
            bridge_fee_amount: BigUint::from(103u32),
            bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 1")),
            executor_fee_amount: BigUint::from(104u32),
            executor_fee_asset_address: Some(String::from("executor_fee_asset_address 1")),
            service_fee_amount: BigUint::from(105u32),
            shielded_recipient_address: String::from("shielded_recipient_address 1"),
            status: deposit_status,
            error_message: Some(String::from("error_message 1")),
            wallet_id: String::from("wallet_id 1"),
            dst_chain_id,
            dst_chain_contract_address: String::from("dst chain contract address 1"),
            dst_pool_address: String::from("0x4fd0ade06b9654437f46EA59e6edEe056F9d5EF7"),
            asset_approve_transaction_hash: Some(String::from("asset_approve_transaction_hash 1")),
            transaction_hash: Some(String::from("transaction_hash 1")),
            queued_transaction_hash: Some(String::from("relay_transaction_hash 1")),
            included_transaction_hash: Some(String::from("rollup_transaction_hash 1")),
        };
        deposits.push(deposit);
    }
    deposits
}
