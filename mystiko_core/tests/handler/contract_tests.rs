use crate::common::create_database;
use mystiko_config::raw::create_raw_from_file;
use mystiko_config::raw::mystiko::RawMystikoConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_core::handler::contract::ContractHandler;
use mystiko_database::database::Database;
use mystiko_storage::document::DOCUMENT_ID_FIELD;
use mystiko_storage::filter::SubFilter;
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage};
use mystiko_types::ContractType;
use std::sync::Arc;

type TypedDatabase = Database<SqlFormatter, SqliteRawData, SqliteStorage>;
type TypedContractHandler = ContractHandler<SqlFormatter, SqliteRawData, SqliteStorage>;

async fn setup() -> (TypedContractHandler, Arc<TypedDatabase>, Arc<MystikoConfig>) {
    let database = Arc::new(create_database().await);
    database.migrate().await.unwrap();
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/handler/contract/config.json")
            .await
            .unwrap(),
    );
    let handler = TypedContractHandler::new(database.clone(), config.clone());
    (handler, database, config)
}

#[tokio::test]
async fn test_contract_initialize() {
    let (handler, _, config) = setup().await;
    let contracts = handler.initialize().await.unwrap();
    for contract in contracts.iter() {
        let chain_config = config.find_chain(contract.data.chain_id).unwrap();
        if let Some(deposit_contract_config) =
            config.find_deposit_contract_by_address(contract.data.chain_id, &contract.data.contract_address)
        {
            assert_eq!(contract.data.contract_type, ContractType::Deposit);
            assert_eq!(contract.data.disabled, deposit_contract_config.disabled());
            assert_eq!(
                contract.data.sync_size,
                chain_config.contract_event_filter_size(&contract.data.contract_address)
            );
            assert_eq!(contract.data.sync_start, deposit_contract_config.start_block());
            assert_eq!(contract.data.synced_block_number, deposit_contract_config.start_block());
            assert_eq!(contract.data.checked_leaf_index, None);
        } else if let Some(pool_contract_config) =
            config.find_pool_contract_by_address(contract.data.chain_id, &contract.data.contract_address)
        {
            assert_eq!(contract.data.contract_type, ContractType::Pool);
            assert!(!contract.data.disabled);
            assert_eq!(
                contract.data.sync_size,
                chain_config.contract_event_filter_size(&contract.data.contract_address)
            );
            assert_eq!(contract.data.sync_start, pool_contract_config.start_block());
            assert_eq!(contract.data.synced_block_number, pool_contract_config.start_block());
            assert_eq!(contract.data.checked_leaf_index, None);
        } else {
            panic!("Contract not found in config");
        }
    }
}

#[tokio::test]
async fn test_contract_initialize_upsert() {
    let (handler, database, _) = setup().await;
    handler.initialize().await.unwrap();
    let count = handler.count_all().await.unwrap();
    let mut raw_config = create_raw_from_file::<RawMystikoConfig>("tests/files/handler/contract/config.json")
        .await
        .unwrap();
    let mut chain_config = raw_config.chains.remove(2).as_ref().clone();
    let mut deposit_contract_config = chain_config.deposit_contracts.remove(0).as_ref().clone();
    deposit_contract_config.disabled = true;
    deposit_contract_config.event_filter_size = Some(1024);
    let chain_id = chain_config.chain_id;
    let contract_address = deposit_contract_config.address.clone();
    chain_config
        .deposit_contracts
        .insert(0, Arc::new(deposit_contract_config));
    raw_config.chains.insert(2, Arc::new(chain_config));
    let new_handler = TypedContractHandler::new(database, Arc::new(MystikoConfig::from_raw(raw_config).unwrap()));
    new_handler.initialize().await.unwrap();
    let contract = new_handler
        .find_by_address(chain_id, &contract_address)
        .await
        .unwrap()
        .unwrap();
    assert!(contract.data.disabled);
    assert_eq!(contract.data.sync_size, 1024);
    assert_eq!(count, new_handler.count_all().await.unwrap());
}

#[tokio::test]
async fn test_contract_find() {
    let (handler, _, _) = setup().await;
    let mut contracts = handler.initialize().await.unwrap();
    assert_eq!(
        handler.find_all().await.unwrap().len() as u64,
        handler.count_all().await.unwrap()
    );
    contracts.sort_by_key(|c| c.id.to_string());

    let filter = SubFilter::IN(
        DOCUMENT_ID_FIELD.to_string(),
        vec![contracts[0].id.clone(), contracts[1].id.clone()],
    );
    let mut found_contracts = handler.find(filter).await.unwrap();
    found_contracts.sort_by_key(|c| c.id.to_string());
    assert_eq!(found_contracts[0], contracts[0]);
    assert_eq!(found_contracts[1], contracts[1]);
    let found_contract = handler.find_by_id(&contracts[0].id).await.unwrap().unwrap();
    assert_eq!(found_contract, contracts[0]);
    let found_contract = handler
        .find_by_address(contracts[0].data.chain_id, &contracts[0].data.contract_address)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_contract, contracts[0]);
    assert_eq!(handler.find_by_chain_id(11155111).await.unwrap().len(), 6);
    assert_eq!(handler.find_by_chain_id(97).await.unwrap().len(), 13);
}

#[tokio::test]
async fn test_contract_count() {
    let (handler, _, _) = setup().await;
    let contracts = handler.initialize().await.unwrap();
    assert_eq!(contracts.len() as u64, handler.count_all().await.unwrap());
    let filter = SubFilter::IN(
        DOCUMENT_ID_FIELD.to_string(),
        vec![contracts[0].id.clone(), contracts[1].id.clone()],
    );
    assert_eq!(handler.count(filter).await.unwrap(), 2);
}

#[tokio::test]
async fn test_contract_reset_synced_block() {
    let (handler, db, _) = setup().await;
    let mut contracts = handler.initialize().await.unwrap();
    contracts[0].data.synced_block_number *= 2;
    db.contracts.update(&contracts[0]).await.unwrap();
    let found_contract = handler.find_by_id(&contracts[0].id).await.unwrap().unwrap();
    assert_ne!(found_contract.data.synced_block_number, contracts[0].data.sync_start);
    handler
        .reset_synced_block(contracts[0].data.chain_id, &contracts[0].data.contract_address)
        .await
        .unwrap();
    let found_contract = handler.find_by_id(&contracts[0].id).await.unwrap().unwrap();
    assert_eq!(found_contract.data.synced_block_number, contracts[0].data.sync_start);
    handler
        .reset_synced_block_to(contracts[0].data.chain_id, &contracts[0].data.contract_address, 100)
        .await
        .unwrap();
    let found_contract = handler.find_by_id(&contracts[0].id).await.unwrap().unwrap();
    assert_eq!(found_contract.data.synced_block_number, 100);
    assert!(handler.find_by_id("wrong_id").await.unwrap().is_none());
    assert!(handler
        .reset_synced_block(11155111, "wrong_address")
        .await
        .unwrap()
        .is_none());
}
