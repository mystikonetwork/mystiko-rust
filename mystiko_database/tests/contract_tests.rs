extern crate mystiko_database;

use mystiko_database::collection::contract::ContractCollection;
use mystiko_database::document::contract::Contract;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use mystiko_types::ContractType;
use std::sync::Arc;

async fn create_contracts() -> ContractCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let contracts = ContractCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
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
            .insert_batch(&vec![
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
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("disabled"),
                        1.to_string()
                    )))
                    .build()
            )
            .await
            .unwrap(),
        1
    );

    // testing find_all
    let mut found_contracts = contracts.find_all().await.unwrap();
    assert_eq!(found_contracts, inserted_contracts);
    // testing find
    found_contracts = contracts
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_contracts, inserted_contracts[1..]);
    // testing find_one
    let mut found_contract = contracts
        .find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                )))
                .build(),
        )
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
    contracts
        .delete_batch(&vec![inserted_contracts[1].clone()])
        .await
        .unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 1);
    // testing delete_by_filter
    contracts.insert(&inserted_contracts[1].data).await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 2);
    contracts
        .delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("0x91fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                )))
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 1);
    // testing delete_all
    contracts.delete_all().await.unwrap();
    assert_eq!(contracts.count_all().await.unwrap(), 0);
}
