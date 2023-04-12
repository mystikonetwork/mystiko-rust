use mystiko_database::collection::transaction::TransactionCollection;
use mystiko_database::document::transaction::{Transaction, TransactionStatus, TransactionType};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use num_bigint::BigInt;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn create_transactions() -> TransactionCollection<SqlFormatter, SqliteRawData, SqliteStorage>
{
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let transactions = TransactionCollection::new(Arc::new(Mutex::new(Collection::new(
        SqlFormatter {},
        storage,
    ))));
    transactions.migrate().await.unwrap();
    assert!(transactions.collection_exists().await.unwrap());
    transactions
}

#[tokio::test]
async fn test_transactions_crud() {
    let transactions = create_transactions().await;

    // testing insert/insert_batch
    let mut inserted_transactions: Vec<Document<Transaction>> = Vec::new();
    inserted_transactions.push(
        transactions
            .insert(&Transaction {
                chain_id: 1,
                contract_address: String::from("contract_address 1"),
                asset_symbol: String::from("asset_symbol 1"),
                asset_decimals: 6,
                asset_address: Some(String::from("asset_address 1")),
                proof: Some(String::from("proof 1")),
                root_hash: String::from("root_hash 1"),
                input_commitments: vec![String::from("ic1"), String::from("ic2")],
                output_commitments: Some(vec![String::from("oc1"), String::from("oc2")]),
                serial_numbers: Some(vec![String::from("sn1"), String::from("sn2")]),
                signature_public_key: Some(String::from("signature_public_key 1")),
                signature_public_key_hashes: Some(vec![
                    String::from("spkh1"),
                    String::from("spkh2"),
                ]),
                amount: BigInt::from(101),
                public_amount: BigInt::from(102),
                rollup_fee_amount: BigInt::from(103),
                gas_relayer_fee_amount: BigInt::from(104),
                shielded_address: Some(String::from("shielded_address 1")),
                public_address: Some(String::from("public_address 1")),
                gas_relayer_address: Some(String::from("gas_relayer_address 1")),
                signature: Some(String::from("signature 1")),
                random_auditing_public_key: Some(String::from("random_auditing_public_key 1")),
                encrypted_auditor_notes: Some(vec![String::from("ean1"), String::from("ean2")]),
                transaction_type: TransactionType::Transfer,
                status: TransactionStatus::Init,
                error_message: Some(String::from("error_message 1")),
                transaction_hash: Some(String::from("transaction_hash 1")),
                wallet_id: String::from("wallet_id 1"),
            })
            .await
            .unwrap(),
    );
    inserted_transactions.extend(
        transactions
            .insert_batch(&vec![
                Transaction {
                    chain_id: 2,
                    contract_address: String::from("contract_address 2"),
                    asset_symbol: String::from("asset_symbol 2"),
                    asset_decimals: 12,
                    asset_address: Some(String::from("asset_address 2")),
                    proof: Some(String::from("proof 2")),
                    root_hash: String::from("root_hash 2"),
                    input_commitments: vec![String::from("ic10"), String::from("ic20")],
                    output_commitments: None,
                    serial_numbers: None,
                    signature_public_key: Some(String::from("signature_public_key 2")),
                    signature_public_key_hashes: None,
                    amount: BigInt::from(201),
                    public_amount: BigInt::from(202),
                    rollup_fee_amount: BigInt::from(203),
                    gas_relayer_fee_amount: BigInt::from(204),
                    shielded_address: Some(String::from("shielded_address 2")),
                    public_address: Some(String::from("public_address 2")),
                    gas_relayer_address: Some(String::from("gas_relayer_address 2")),
                    signature: Some(String::from("signature 2")),
                    random_auditing_public_key: Some(String::from("random_auditing_public_key 2")),
                    encrypted_auditor_notes: None,
                    transaction_type: TransactionType::Withdraw,
                    status: TransactionStatus::ProofGenerated,
                    error_message: Some(String::from("error_message 2")),
                    transaction_hash: Some(String::from("transaction_hash 2")),
                    wallet_id: String::from("wallet_id 2"),
                },
                Transaction {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    asset_symbol: String::from("asset_symbol 3"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("asset_address 3")),
                    proof: Some(String::from("proof 3")),
                    root_hash: String::from("root_hash 3"),
                    input_commitments: vec![String::from("ic100"), String::from("ic200")],
                    output_commitments: Some(vec![]),
                    serial_numbers: Some(vec![]),
                    signature_public_key: Some(String::from("signature_public_key 3")),
                    signature_public_key_hashes: Some(vec![]),
                    amount: BigInt::from(301),
                    public_amount: BigInt::from(302),
                    rollup_fee_amount: BigInt::from(303),
                    gas_relayer_fee_amount: BigInt::from(304),
                    shielded_address: Some(String::from("shielded_address 3")),
                    public_address: Some(String::from("public_address 3")),
                    gas_relayer_address: Some(String::from("gas_relayer_address 3")),
                    signature: Some(String::from("signature 3")),
                    random_auditing_public_key: Some(String::from("random_auditing_public_key 3")),
                    encrypted_auditor_notes: Some(vec![]),
                    transaction_type: TransactionType::Withdraw,
                    status: TransactionStatus::Failed,
                    error_message: Some(String::from("error_message 3")),
                    transaction_hash: Some(String::from("transaction_hash 3")),
                    wallet_id: String::from("wallet_id 3"),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(transactions.count_all().await.unwrap(), 3);
    assert_eq!(
        transactions
            .count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("signature_public_key"),
                        "signature_public_key 2".to_string()
                    )))
                    .build()
            )
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_transactions = transactions.find_all().await.unwrap();
    assert_eq!(found_transactions, inserted_transactions);
    let found_first_transaction = transactions
        .find_one(QueryFilterBuilder::new().limit(1).build())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        found_first_transaction.data.status.to_string(),
        "Init".to_string()
    );
    found_transactions = transactions
        .find(QueryFilterBuilder::new().limit(2).offset(1).build())
        .await
        .unwrap();
    assert_eq!(found_transactions, inserted_transactions[1..]);
    let mut found_transaction = transactions
        .find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("asset_symbol"),
                    String::from("asset_symbol 2"),
                )))
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_transaction, inserted_transactions[1]);
    found_transaction = transactions
        .find_by_id(&inserted_transactions[2].id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_transaction, inserted_transactions[2]);

    // testing update/update_batch
    found_transaction.data.asset_decimals = 10;
    let updated_transaction = transactions.update(&found_transaction).await.unwrap();
    assert_eq!(updated_transaction.data, found_transaction.data);
    inserted_transactions[0].data.asset_decimals = 11;
    inserted_transactions[1].data.asset_decimals = 22;
    inserted_transactions[2].data.asset_decimals = 33;
    found_transactions = transactions
        .update_batch(&inserted_transactions)
        .await
        .unwrap();
    assert_eq!(found_transactions[0].data, inserted_transactions[0].data);
    assert_eq!(found_transactions[1].data, inserted_transactions[1].data);
    assert_eq!(found_transactions[2].data, inserted_transactions[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    transactions
        .delete(&inserted_transactions[0])
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 2);
    transactions
        .delete_batch(&vec![inserted_transactions[1].clone()])
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 1);
    transactions
        .insert(&inserted_transactions[0].data)
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 2);
    transactions
        .delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("shielded_address"),
                    String::from("shielded_address 1"),
                )))
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 1);
    transactions.delete_all().await.unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_transaction_status_serde() {
    assert!(TransactionStatus::from_str("invalid").is_err());
}
