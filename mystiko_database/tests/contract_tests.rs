extern crate mystiko_database;

use mystiko_config::MystikoConfig;
use mystiko_database::document::{Contract, ContractCollection, ContractColumn};
use mystiko_dataloader::handler::document::DatabaseContract;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_types::ContractType;
use std::sync::Arc;

async fn create_contracts() -> ContractCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
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
                loaded_block_number: 1100000,
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
                    loaded_block_number: 2200000,
                },
                Contract {
                    contract_type: ContractType::Pool,
                    chain_id: 5,
                    contract_address: String::from("0xEd95f2F493687dFaeefD33C00C6Bc4dF0fbB3404"),
                    disabled: false,
                    loaded_block_number: 3300000,
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
    found_contract.data.loaded_block_number = 2220000;
    let updated_contract = contracts.update(&found_contract).await.unwrap();
    assert_eq!(updated_contract.data, found_contract.data);
    // testing update_batch
    inserted_contracts[0].data.loaded_block_number = 1110000;
    inserted_contracts[2].data.loaded_block_number = 3330000;
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
            loaded_block_number: 1100000,
        })
        .await
        .unwrap();
    assert_eq!(
        contract,
        serde_json::from_str(&serde_json::to_string(&contract).unwrap()).unwrap()
    );
}

#[tokio::test]
async fn test_loader_database_contract() {
    assert_eq!(Contract::column_chain_id(), "chain_id");
    assert_eq!(Contract::column_contract_address(), "contract_address");
    assert_eq!(Contract::column_loaded_block(), "loaded_block_number");
    let mut contract = Contract {
        contract_type: ContractType::Pool,
        chain_id: 5,
        contract_address: String::from("0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"),
        disabled: false,
        loaded_block_number: 1000000,
    };
    assert_eq!(contract.get_chain_id(), 5);
    assert_eq!(
        contract.get_contract_address(),
        "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d"
    );
    assert_eq!(contract.get_loaded_block(), 1000000);
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let converted_contract =
        Contract::from_config(config.clone(), 5, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d").unwrap();
    assert_eq!(converted_contract, contract);
    assert!(Contract::from_config(config, 1, "0xF55Dbe8D71Df9Bbf5841052C75c6Ea9eA717fc6d").is_err());

    contract.loaded_block_number = 2000000;
    contract.update_loaded_block(2000001);
    assert_eq!(contract.get_loaded_block(), 2000001);
}
