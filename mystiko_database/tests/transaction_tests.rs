use futures::lock::Mutex;
use mystiko_database::collection::transaction::TransactionCollection;
use mystiko_database::document::transaction::{Transaction, TransactionStatus, TransactionType};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::str::FromStr;
use std::sync::Arc;
use tokio_test::block_on;

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

#[test]
fn test_transactions_crud() {
    let transactions = block_on(create_transactions());

    // testing insert/insert_batch
    let mut inserted_transactions: Vec<Document<Transaction>> = Vec::new();
    inserted_transactions.push(
        block_on(transactions.insert(&Transaction {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 6,
            asset_address: Some(String::from("asset_address 1")),
            proof: Some(String::from("proof 1")),
            root_hash: String::from("root_hash 1"),
            input_commitments: String::from("input_commitments 1"),
            output_commitments: Some(String::from("output_commitments 1")),
            serial_numbers: Some(String::from("serial_numbers 1")),
            signature_public_key: Some(String::from("signature_public_key 1")),
            signature_public_key_hashes: Some(String::from("signature_public_key_hashes 1")),
            amount: String::from("amount 1"),
            public_amount: String::from("public_amount 1"),
            rollup_fee_amount: String::from("rollup_fee_amount 1"),
            gas_relayer_fee_amount: String::from("gas_relayer_fee_amount 1"),
            shielded_address: Some(String::from("shielded_address 1")),
            public_address: Some(String::from("public_address 1")),
            gas_relayer_address: Some(String::from("gas_relayer_address 1")),
            signature: Some(String::from("signature 1")),
            random_auditing_public_key: Some(String::from("random_auditing_public_key 1")),
            encrypted_auditor_notes: Some(String::from("encrypted_auditor_notes 1")),
            transaction_type: TransactionType::Transfer,
            status: TransactionStatus::Init,
            error_message: Some(String::from("error_message 1")),
            transaction_hash: Some(String::from("transaction_hash 1")),
            wallet_id: String::from("wallet_id 1"),
        }))
        .unwrap(),
    );
    inserted_transactions.extend(
        block_on(transactions.insert_batch(&vec![
            Transaction {
                chain_id: 2,
                contract_address: String::from("contract_address 2"),
                asset_symbol: String::from("asset_symbol 2"),
                asset_decimals: 12,
                asset_address: Some(String::from("asset_address 2")),
                proof: Some(String::from("proof 2")),
                root_hash: String::from("root_hash 2"),
                input_commitments: String::from("input_commitments 2"),
                output_commitments: Some(String::from("output_commitments 2")),
                serial_numbers: Some(String::from("serial_numbers 2")),
                signature_public_key: Some(String::from("signature_public_key 2")),
                signature_public_key_hashes: Some(String::from("signature_public_key_hashes 2")),
                amount: String::from("amount 2"),
                public_amount: String::from("public_amount 2"),
                rollup_fee_amount: String::from("rollup_fee_amount 2"),
                gas_relayer_fee_amount: String::from("gas_relayer_fee_amount 2"),
                shielded_address: Some(String::from("shielded_address 2")),
                public_address: Some(String::from("public_address 2")),
                gas_relayer_address: Some(String::from("gas_relayer_address 2")),
                signature: Some(String::from("signature 2")),
                random_auditing_public_key: Some(String::from("random_auditing_public_key 2")),
                encrypted_auditor_notes: Some(String::from("encrypted_auditor_notes 2")),
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
                input_commitments: String::from("input_commitments 3"),
                output_commitments: Some(String::from("output_commitments 3")),
                serial_numbers: Some(String::from("serial_numbers 3")),
                signature_public_key: Some(String::from("signature_public_key 3")),
                signature_public_key_hashes: Some(String::from("signature_public_key_hashes 3")),
                amount: String::from("amount 3"),
                public_amount: String::from("public_amount 3"),
                rollup_fee_amount: String::from("rollup_fee_amount 3"),
                gas_relayer_fee_amount: String::from("gas_relayer_fee_amount 3"),
                shielded_address: Some(String::from("shielded_address 3")),
                public_address: Some(String::from("public_address 3")),
                gas_relayer_address: Some(String::from("gas_relayer_address 3")),
                signature: Some(String::from("signature 3")),
                random_auditing_public_key: Some(String::from("random_auditing_public_key 3")),
                encrypted_auditor_notes: Some(String::from("encrypted_auditor_notes 3")),
                transaction_type: TransactionType::Withdraw,
                status: TransactionStatus::Failed,
                error_message: Some(String::from("error_message 3")),
                transaction_hash: Some(String::from("transaction_hash 3")),
                wallet_id: String::from("wallet_id 3"),
            },
        ]))
        .unwrap(),
    );

    // testing count/count_all
    assert_eq!(block_on(transactions.count_all()).unwrap(), 3);
    assert_eq!(
        block_on(
            transactions.count(
                QueryFilterBuilder::new()
                    .filter(Condition::FILTER(SubFilter::Equal(
                        String::from("signature_public_key"),
                        "signature_public_key 2".to_string()
                    )))
                    .build()
            )
        )
        .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_transactions = block_on(transactions.find_all()).unwrap();
    assert_eq!(found_transactions, inserted_transactions);
    let found_first_transaction =
        block_on(transactions.find_one(QueryFilterBuilder::new().limit(1).build()))
            .unwrap()
            .unwrap();
    assert_eq!(
        found_first_transaction.data.status.to_string(),
        "Init".to_string()
    );
    found_transactions =
        block_on(transactions.find(QueryFilterBuilder::new().limit(2).offset(1).build())).unwrap();
    assert_eq!(found_transactions, inserted_transactions[1..]);
    let mut found_transaction = block_on(
        transactions.find_one(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("serial_numbers"),
                    String::from("serial_numbers 2"),
                )))
                .build(),
        ),
    )
    .unwrap()
    .unwrap();
    assert_eq!(found_transaction, inserted_transactions[1]);
    found_transaction = block_on(transactions.find_by_id(&inserted_transactions[2].id))
        .unwrap()
        .unwrap();
    assert_eq!(found_transaction, inserted_transactions[2]);

    // testing update/update_batch
    found_transaction.data.asset_decimals = 10;
    let updated_transaction = block_on(transactions.update(&found_transaction)).unwrap();
    assert_eq!(updated_transaction.data, found_transaction.data);
    inserted_transactions[0].data.asset_decimals = 11;
    inserted_transactions[1].data.asset_decimals = 22;
    inserted_transactions[2].data.asset_decimals = 33;
    found_transactions = block_on(transactions.update_batch(&inserted_transactions)).unwrap();
    assert_eq!(found_transactions[0].data, inserted_transactions[0].data);
    assert_eq!(found_transactions[1].data, inserted_transactions[1].data);
    assert_eq!(found_transactions[2].data, inserted_transactions[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    block_on(transactions.delete(&inserted_transactions[0])).unwrap();
    assert_eq!(block_on(transactions.count_all()).unwrap(), 2);
    block_on(transactions.delete_batch(&vec![inserted_transactions[1].clone()])).unwrap();
    assert_eq!(block_on(transactions.count_all()).unwrap(), 1);
    block_on(transactions.insert(&inserted_transactions[0].data)).unwrap();
    assert_eq!(block_on(transactions.count_all()).unwrap(), 2);
    block_on(
        transactions.delete_by_filter(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("shielded_address"),
                    String::from("shielded_address 1"),
                )))
                .build(),
        ),
    )
    .unwrap();
    assert_eq!(block_on(transactions.count_all()).unwrap(), 1);
    block_on(transactions.delete_all()).unwrap();
    assert_eq!(block_on(transactions.count_all()).unwrap(), 0);
}

#[test]
fn test_transaction_status_serde() {
    assert!(TransactionStatus::from_str("invalid").is_err());
}
