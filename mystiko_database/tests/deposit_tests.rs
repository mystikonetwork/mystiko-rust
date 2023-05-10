extern crate mystiko_database;

use mystiko_database::collection::deposit::DepositCollection;
use mystiko_database::document::deposit::Deposit;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use mystiko_types::{BridgeType, DepositStatus};
use num_bigint::BigInt;
use std::sync::Arc;

async fn create_deposits() -> DepositCollection<SqlFormatter, SqliteRawData, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let deposits = DepositCollection::new(Arc::new(Collection::new(SqlFormatter {}, storage)));
    deposits.migrate().await.unwrap();
    assert!(deposits.collection_exists().await.unwrap());
    deposits
}

#[tokio::test]
async fn test_deposits_crud() {
    let deposits = create_deposits().await;

    // testing insert/insert_batch
    let mut inserted_deposits: Vec<Document<Deposit>> = Vec::new();
    inserted_deposits.push(
        deposits
            .insert(&Deposit {
                chain_id: 1,
                contract_address: String::from("contract_address 1"),
                pool_address: String::from("pool_address 1"),
                commitment_hash: BigInt::from(1),
                hash_k: BigInt::from(11),
                random_s: BigInt::from(111),
                encrypted_note: String::from("encrypted_note 1"),
                asset_symbol: String::from("asset_symbol 1"),
                asset_decimals: 6,
                asset_address: Some(String::from("asset_address 1")),
                bridge_type: BridgeType::Axelar,
                amount: BigInt::from(101),
                rollup_fee_amount: BigInt::from(102),
                bridge_fee_amount: BigInt::from(103),
                bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 1")),
                executor_fee_amount: BigInt::from(104),
                executor_fee_asset_address: Some(String::from("executor_fee_asset_address 1")),
                service_fee_amount: BigInt::from(105),
                shielded_recipient_address: String::from("shielded_recipient_address 1"),
                status: DepositStatus::Init,
                error_message: Some(String::from("error_message 1")),
                wallet_id: String::from("wallet_id 1"),
                dst_chain_id: 11,
                dst_chain_contract_address: String::from("dst_chain_contract_address 1"),
                dst_pool_address: String::from("dst_pool_address 1"),
                asset_approve_transaction_hash: Some(String::from("asset_approve_transaction_hash 1")),
                transaction_hash: Some(String::from("transaction_hash 1")),
                relay_transaction_hash: Some(String::from("relay_transaction_hash 1")),
                rollup_transaction_hash: Some(String::from("rollup_transaction_hash 1")),
            })
            .await
            .unwrap(),
    );
    inserted_deposits.extend(
        deposits
            .insert_batch(&vec![
                Deposit {
                    chain_id: 2,
                    contract_address: String::from("contract_address 2"),
                    pool_address: String::from("pool_address 2"),
                    commitment_hash: BigInt::from(2),
                    hash_k: BigInt::from(22),
                    random_s: BigInt::from(222),
                    encrypted_note: String::from("encrypted_note 2"),
                    asset_symbol: String::from("asset_symbol 2"),
                    asset_decimals: 12,
                    asset_address: Some(String::from("asset_address 2")),
                    bridge_type: BridgeType::Celer,
                    amount: BigInt::from(201),
                    rollup_fee_amount: BigInt::from(202),
                    bridge_fee_amount: BigInt::from(203),
                    bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 2")),
                    executor_fee_amount: BigInt::from(204),
                    executor_fee_asset_address: Some(String::from("executor_fee_asset_address 2")),
                    service_fee_amount: BigInt::from(205),
                    shielded_recipient_address: String::from("shielded_recipient_address 2"),
                    status: DepositStatus::Included,
                    error_message: Some(String::from("error_message 2")),
                    wallet_id: String::from("wallet_id 2"),
                    dst_chain_id: 22,
                    dst_chain_contract_address: String::from("dst_chain_contract_address 2"),
                    dst_pool_address: String::from("dst_pool_address 2"),
                    asset_approve_transaction_hash: Some(String::from("asset_approve_transaction_hash 2")),
                    transaction_hash: Some(String::from("transaction_hash 2")),
                    relay_transaction_hash: Some(String::from("relay_transaction_hash 2")),
                    rollup_transaction_hash: Some(String::from("rollup_transaction_hash 2")),
                },
                Deposit {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    pool_address: String::from("pool_address 3"),
                    commitment_hash: BigInt::from(3),
                    hash_k: BigInt::from(33),
                    random_s: BigInt::from(333),
                    encrypted_note: String::from("encrypted_note 3"),
                    asset_symbol: String::from("asset_symbol 3"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("asset_address 3")),
                    bridge_type: BridgeType::LayerZero,
                    amount: BigInt::from(301),
                    rollup_fee_amount: BigInt::from(302),
                    bridge_fee_amount: BigInt::from(303),
                    bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 3")),
                    executor_fee_amount: BigInt::from(304),
                    executor_fee_asset_address: Some(String::from("executor_fee_asset_address 3")),
                    service_fee_amount: BigInt::from(305),
                    shielded_recipient_address: String::from("shielded_recipient_address 3"),
                    status: DepositStatus::Queued,
                    error_message: Some(String::from("error_message 3")),
                    wallet_id: String::from("wallet_id 3"),
                    dst_chain_id: 33,
                    dst_chain_contract_address: String::from("dst_chain_contract_address 3"),
                    dst_pool_address: String::from("dst_pool_address 3"),
                    asset_approve_transaction_hash: Some(String::from("asset_approve_transaction_hash 3")),
                    transaction_hash: Some(String::from("transaction_hash 3")),
                    relay_transaction_hash: Some(String::from("relay_transaction_hash 3")),
                    rollup_transaction_hash: Some(String::from("rollup_transaction_hash 3")),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(deposits.count_all().await.unwrap(), 3);
    assert_eq!(
        deposits
            .count(SubFilter::Equal(String::from("hash_k"), 22.to_string()))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_deposits = deposits.find_all().await.unwrap();
    assert_eq!(found_deposits, inserted_deposits);
    let found_first_deposit = deposits
        .find_one(QueryFilterBuilder::new().limit(1).build())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_first_deposit.data.status, DepositStatus::Init,);
    found_deposits = deposits
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_deposits, inserted_deposits[1..]);
    let mut found_deposit = deposits
        .find_one(SubFilter::Equal(String::from("random_s"), 222.to_string()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_deposit, inserted_deposits[1]);
    found_deposit = deposits.find_by_id(&inserted_deposits[2].id).await.unwrap().unwrap();
    assert_eq!(found_deposit, inserted_deposits[2]);

    // testing update/update_batch
    found_deposit.data.dst_chain_id = 38;
    let updated_deposit = deposits.update(&found_deposit).await.unwrap();
    assert_eq!(updated_deposit.data, found_deposit.data);
    inserted_deposits[0].data.dst_chain_id = 16;
    inserted_deposits[1].data.dst_chain_id = 26;
    inserted_deposits[2].data.dst_chain_id = 36;
    found_deposits = deposits.update_batch(&inserted_deposits).await.unwrap();
    assert_eq!(found_deposits[0].data, inserted_deposits[0].data);
    assert_eq!(found_deposits[1].data, inserted_deposits[1].data);
    assert_eq!(found_deposits[2].data, inserted_deposits[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    deposits.delete(&inserted_deposits[0]).await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 2);
    deposits
        .delete_batch(&vec![inserted_deposits[1].clone()])
        .await
        .unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 1);
    deposits.insert(&inserted_deposits[0].data).await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 2);
    deposits
        .delete_by_filter(SubFilter::Equal(
            String::from("shielded_recipient_address"),
            String::from("shielded_recipient_address 1"),
        ))
        .await
        .unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 1);
    deposits.delete_all().await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 0);
}
