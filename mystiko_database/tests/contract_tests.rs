extern crate mystiko_database;

use futures::lock::Mutex;
use mystiko_database::collection::contract::ContractCollection;
use mystiko_database::document::contract::{Contract, ContractType};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::sync::Arc;
use tokio_test::block_on;

async fn create_contracts() -> ContractCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let contracts = ContractCollection::new(Arc::new(Mutex::new(Collection::new(
        SqlFormatter {},
        storage,
    ))));
    contracts.migrate().await.unwrap();
    assert!(contracts.collection_exists().await.unwrap());
    contracts
}

#[test]
fn test_contracts_crud() {
    let contracts = block_on(create_contracts());

    // testing insert
    let mut inserted_contracts: Vec<Document<Contract>> = Vec::new();
    inserted_contracts.push(
        block_on(contracts.insert(&Contract {
            contract_type: ContractType::Deposit,
            chain_id: 5,
            contract_address: String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
            disabled: 0,
            sync_start: 1000000,
            sync_size: 10000,
            synced_block_number: 1100000,
            checked_leaf_index: Some(10),
        }))
        .unwrap(),
    );
    assert_eq!(block_on(contracts.count_all()).unwrap(), 1);
    // testing insert_batch
    inserted_contracts.extend(
        block_on(contracts.insert_batch(&vec![
            Contract {
                contract_type: ContractType::Deposit,
                chain_id: 5,
                contract_address: String::from("0x91fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                disabled: 1,
                sync_start: 2000000,
                sync_size: 20000,
                synced_block_number: 2200000,
                checked_leaf_index: Some(20),
            },
            Contract {
                contract_type: ContractType::Pool,
                chain_id: 5,
                contract_address: String::from("0xEd95f2F493687dFaeefD33C00C6Bc4dF0fbB3404"),
                disabled: 0,
                sync_start: 3000000,
                sync_size: 30000,
                synced_block_number: 3300000,
                checked_leaf_index: Some(30),
            },
        ]))
        .unwrap(),
    );
    assert_eq!(block_on(contracts.count_all()).unwrap(), 3);

    // testing count
    assert_eq!(
        block_on(
            contracts.count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("disabled"),
                        1.to_string()
                    )))
                    .build()
            )
        )
        .unwrap(),
        1
    );

    // testing find_all
    let mut found_contracts = block_on(contracts.find_all()).unwrap();
    assert_eq!(found_contracts, inserted_contracts);
    // testing find
    found_contracts =
        block_on(contracts.find(QueryFilterBuilder::new().limit(2).offset(1).build())).unwrap();
    assert_eq!(found_contracts, inserted_contracts[1..]);
    // testing find_one
    let mut found_contract = block_on(
        contracts.find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("0x90fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                )))
                .build(),
        ),
    )
    .unwrap()
    .unwrap();
    assert_eq!(found_contract, inserted_contracts[0]);
    // testing find_by_id
    found_contract = block_on(contracts.find_by_id(&inserted_contracts[1].id))
        .unwrap()
        .unwrap();
    assert_eq!(found_contract, inserted_contracts[1]);

    // testing update
    found_contract.data.synced_block_number = 2220000;
    let updated_contract = block_on(contracts.update(&found_contract)).unwrap();
    assert_eq!(updated_contract.data, found_contract.data);
    // testing update_batch
    inserted_contracts[0].data.synced_block_number = 1110000;
    inserted_contracts[2].data.synced_block_number = 3330000;
    found_contracts = block_on(contracts.update_batch(&inserted_contracts)).unwrap();
    assert_eq!(found_contracts[0].data, inserted_contracts[0].data);
    assert_eq!(found_contracts[2].data, inserted_contracts[2].data);

    // testing delete
    block_on(contracts.delete(&inserted_contracts[0])).unwrap();
    assert_eq!(block_on(contracts.count_all()).unwrap(), 2);
    // testing delete_batch
    block_on(contracts.delete_batch(&vec![inserted_contracts[1].clone()])).unwrap();
    assert_eq!(block_on(contracts.count_all()).unwrap(), 1);
    // testing delete_by_filter
    block_on(contracts.insert(&inserted_contracts[1].data)).unwrap();
    assert_eq!(block_on(contracts.count_all()).unwrap(), 2);
    block_on(
        contracts.delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("contract_address"),
                    String::from("0x91fEF726f3b510521AeF20C27D1d23dcC44Dc84d"),
                )))
                .build(),
        ),
    )
    .unwrap();
    assert_eq!(block_on(contracts.count_all()).unwrap(), 1);
    // testing delete_all
    block_on(contracts.delete_all()).unwrap();
    assert_eq!(block_on(contracts.count_all()).unwrap(), 0);
}
