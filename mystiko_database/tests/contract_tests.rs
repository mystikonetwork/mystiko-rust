extern crate mystiko_database;

use mystiko_database::document::contract::{Contract, ContractCollection, ContractColumn};
use mystiko_protos::core::document::v1::Contract as ProtoContract;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use mystiko_types::ContractType;
use std::sync::Arc;

async fn create_contracts() -> ContractCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let contracts = ContractCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
    contracts.migrate().await.unwrap();
    assert!(contracts.collection_exists().await.unwrap());
    contracts
}

#[tokio::test]
async fn test_contracts_crud() {
    let contracts = create_contracts().await;

    // testing insert
    let mut inserted_contracts: Vec<Document<Contract>> = Vec::new();
    inserted_contracts.push(
        contracts
            .insert(&Contract {
                contract_type: ContractType::Deposit,
                chain_id: 5,
                contract_address: String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                disabled: false,
                sync_start: 1000000,
                sync_size: 10000,
                synced_block_number: 1100000,
                checked_leaf_index: Some(10),
            })
            .await
            .unwrap(),
    );
    assert_eq!(contracts.count_all().await.unwrap(), 1);
    // testing insert_batch
    inserted_contracts.extend(
        contracts
            .insert_batch(&[
                Contract {
                    contract_type: ContractType::Deposit,
                    chain_id: 5,
                    contract_address: String::from("0x91fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                    disabled: true,
                    sync_start: 2000000,
                    sync_size: 20000,
                    synced_block_number: 2200000,
                    checked_leaf_index: Some(20),
                },
                Contract {
                    contract_type: ContractType::Pool,
                    chain_id: 5,
                    contract_address: String::from("0xEd95f2F493687dFaeefD33C00C6Bc4dF0fbB3404"),
                    disabled: false,
                    sync_start: 3000000,
                    sync_size: 30000,
                    synced_block_number: 3300000,
                    checked_leaf_index: Some(30),
                },
            ])
            .await
            .unwrap(),
    );
    assert_eq!(contracts.count_all().await.unwrap(), 3);

    // testing count
    assert_eq!(
        contracts
            .count(SubFilter::equal(ContractColumn::Disabled, true))
            .await
            .unwrap(),
        1
    );

    // testing find_all
    let mut found_contracts = contracts.find_all().await.unwrap();
    assert_eq!(found_contracts, inserted_contracts);
    // testing find
    found_contracts = contracts
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_contracts, inserted_contracts[1..]);
    // testing find_one
    let mut found_contract = contracts
        .find_one(SubFilter::equal(
            ContractColumn::ContractAddress,
            "0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d",
        ))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_contract, inserted_contracts[0]);
    // testing find_by_id
    found_contract = contracts.find_by_id(&inserted_contracts[1].id).await.unwrap().unwrap();
    assert_eq!(found_contract, inserted_contracts[1]);

    // testing update
    found_contract.data.synced_block_number = 2220000;
    let updated_contract = contracts.update(&found_contract).await.unwrap();
    assert_eq!(updated_contract.data, found_contract.data);
    // testing update_batch
    inserted_contracts[0].data.synced_block_number = 1110000;
    inserted_contracts[2].data.synced_block_number = 3330000;
    found_contracts = contracts.update_batch(&inserted_contracts).await.unwrap();
    assert_eq!(found_contracts[0].data, inserted_contracts[0].data);
    assert_eq!(found_contracts[2].data, inserted_contracts[2].data);

    // testing delete
    contracts.delete(&inserted_contracts[0]).await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 2);
    // testing delete_batch
    contracts.delete_batch(&[inserted_contracts[1].clone()]).await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 1);
    // testing delete_by_filter
    contracts.insert(&inserted_contracts[1].data).await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 2);
    contracts
        .delete_by_filter(SubFilter::equal(
            ContractColumn::ContractAddress,
            "0x91fEF726f3b510521AeF20C27D1d23dcC44Dc84d",
        ))
        .await
        .unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 1);
    // testing delete_all
    contracts.delete_all().await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_contract_serde() {
    let contracts = create_contracts().await;
    let contract = contracts
        .insert(&Contract {
            contract_type: ContractType::Deposit,
            chain_id: 5,
            contract_address: String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
            disabled: false,
            sync_start: 1000000,
            sync_size: 10000,
            synced_block_number: 1100000,
            checked_leaf_index: Some(10),
        })
        .await
        .unwrap();
    assert_eq!(
        contract,
        serde_json::from_str(&serde_json::to_string(&contract).unwrap()).unwrap()
    );
}

#[test]
fn test_from_proto() {
    let proto = ProtoContract::builder()
        .id(String::from("123456"))
        .created_at(1234567890u64)
        .updated_at(1234567891u64)
        .chain_id(5u64)
        .contract_address(String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"))
        .disabled(false)
        .sync_start(1000000u64)
        .sync_size(10000u64)
        .synced_block_number(1100000u64)
        .checked_leaf_index(10)
        .contract_type(1)
        .build();
    let contract = Contract::from_proto(proto);
    assert_eq!(contract.id, String::from("123456"));
    assert_eq!(contract.created_at, 1234567890u64);
    assert_eq!(contract.updated_at, 1234567891u64);
    assert_eq!(contract.data.chain_id, 5u64);
    assert_eq!(
        contract.data.contract_address,
        String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d")
    );
    assert!(!contract.data.disabled);
    assert_eq!(contract.data.sync_start, 1000000u64);
    assert_eq!(contract.data.sync_size, 10000u64);
    assert_eq!(contract.data.synced_block_number, 1100000u64);
    assert_eq!(contract.data.checked_leaf_index, Some(10));
    assert_eq!(contract.data.contract_type, ContractType::Deposit);
}

#[test]
fn test_into_proto() {
    let contract = Document::new(
        String::from("123456"),
        1234567890u64,
        1234567891u64,
        Contract {
            contract_type: ContractType::Deposit,
            chain_id: 5,
            contract_address: String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
            disabled: false,
            sync_start: 1000000,
            sync_size: 10000,
            synced_block_number: 1100000,
            checked_leaf_index: Some(10),
        },
    );
    let proto = Contract::into_proto(contract);
    assert_eq!(proto.id, String::from("123456"));
    assert_eq!(proto.created_at, 1234567890u64);
    assert_eq!(proto.updated_at, 1234567891u64);
    assert_eq!(proto.chain_id, 5u64);
    assert_eq!(
        proto.contract_address,
        String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d")
    );
    assert!(!proto.disabled);
    assert_eq!(proto.sync_start, 1000000u64);
    assert_eq!(proto.sync_size, 10000u64);
    assert_eq!(proto.synced_block_number, 1100000u64);
    assert!(proto.checked_leaf_index.is_some());
    assert_eq!(proto.checked_leaf_index.unwrap(), 10);
    assert_eq!(proto.contract_type, 1);
}
