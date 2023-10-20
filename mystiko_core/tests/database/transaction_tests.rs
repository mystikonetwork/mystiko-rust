use mystiko_core::{Transaction, TransactionCollection, TransactionColumn};
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_types::{TransactionStatus, TransactionType};
use num_bigint::BigUint;
use std::sync::Arc;

async fn create_transactions() -> TransactionCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let transactions = TransactionCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
                root_hash: BigUint::from(1u32),
                input_commitments: vec![BigUint::from(11u32), BigUint::from(12u32)],
                output_commitments: Some(vec![BigUint::from(111u32), BigUint::from(112u32)]),
                nullifiers: Some(vec![BigUint::from(1111u32), BigUint::from(1112u32)]),
                signature_public_key: Some(String::from("signature_public_key 1")),
                signature_public_key_hashes: Some(vec![String::from("spkh1"), String::from("spkh2")]),
                amount: BigUint::from(101u32),
                public_amount: BigUint::from(102u32),
                rollup_fee_amount: BigUint::from(103u32),
                gas_relayer_fee_amount: BigUint::from(104u32),
                shielded_address: Some(String::from("shielded_address 1")),
                public_address: Some(String::from("public_address 1")),
                gas_relayer_address: Some(String::from("gas_relayer_address 1")),
                signature: Some(String::from("signature 1")),
                random_auditing_public_key: Some(BigUint::from(11111u32)),
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
                    root_hash: BigUint::from(2u32),
                    input_commitments: vec![BigUint::from(22u32), BigUint::from(23u32)],
                    output_commitments: None,
                    nullifiers: None,
                    signature_public_key: Some(String::from("signature_public_key 2")),
                    signature_public_key_hashes: None,
                    amount: BigUint::from(201u32),
                    public_amount: BigUint::from(202u32),
                    rollup_fee_amount: BigUint::from(203u32),
                    gas_relayer_fee_amount: BigUint::from(204u32),
                    shielded_address: Some(String::from("shielded_address 2")),
                    public_address: Some(String::from("public_address 2")),
                    gas_relayer_address: Some(String::from("gas_relayer_address 2")),
                    signature: Some(String::from("signature 2")),
                    random_auditing_public_key: Some(BigUint::from(22222u32)),
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
                    root_hash: BigUint::from(3u32),
                    input_commitments: vec![BigUint::from(33u32), BigUint::from(34u32)],
                    output_commitments: Some(vec![]),
                    nullifiers: Some(vec![]),
                    signature_public_key: Some(String::from("signature_public_key 3")),
                    signature_public_key_hashes: Some(vec![]),
                    amount: BigUint::from(301u32),
                    public_amount: BigUint::from(302u32),
                    rollup_fee_amount: BigUint::from(303u32),
                    gas_relayer_fee_amount: BigUint::from(304u32),
                    shielded_address: Some(String::from("shielded_address 3")),
                    public_address: Some(String::from("public_address 3")),
                    gas_relayer_address: Some(String::from("gas_relayer_address 3")),
                    signature: Some(String::from("signature 3")),
                    random_auditing_public_key: Some(BigUint::from(33333u32)),
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
            .count(SubFilter::equal(
                TransactionColumn::SignaturePublicKey,
                "signature_public_key 2"
            ))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_transactions = transactions.find_all().await.unwrap();
    assert_eq!(found_transactions, inserted_transactions);
    let found_first_transaction = transactions
        .find_one(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(1)
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_first_transaction.data.status, TransactionStatus::Init,);
    found_transactions = transactions
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_transactions, inserted_transactions[1..]);
    let mut found_transaction = transactions
        .find_one(SubFilter::equal(TransactionColumn::AssetSymbol, "asset_symbol 2"))
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
    found_transactions = transactions.update_batch(&inserted_transactions).await.unwrap();
    assert_eq!(found_transactions[0].data, inserted_transactions[0].data);
    assert_eq!(found_transactions[1].data, inserted_transactions[1].data);
    assert_eq!(found_transactions[2].data, inserted_transactions[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    transactions.delete(&inserted_transactions[0]).await.unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 2);
    transactions
        .delete_batch(&vec![inserted_transactions[1].clone()])
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 1);
    transactions.insert(&inserted_transactions[0].data).await.unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 2);
    transactions
        .delete_by_filter(SubFilter::equal(
            TransactionColumn::ShieldedAddress,
            "shielded_address 1",
        ))
        .await
        .unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 1);
    transactions.delete_all().await.unwrap();
    assert_eq!(transactions.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_transaction_serde() {
    let transactions = create_transactions().await;
    let transaction = transactions
        .insert(&Transaction {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 6,
            asset_address: Some(String::from("asset_address 1")),
            proof: Some(String::from("proof 1")),
            root_hash: BigUint::from(1u32),
            input_commitments: vec![BigUint::from(11u32), BigUint::from(12u32)],
            output_commitments: Some(vec![BigUint::from(111u32), BigUint::from(112u32)]),
            nullifiers: Some(vec![BigUint::from(1111u32), BigUint::from(1112u32)]),
            signature_public_key: Some(String::from("signature_public_key 1")),
            signature_public_key_hashes: Some(vec![String::from("spkh1"), String::from("spkh2")]),
            amount: BigUint::from(101u32),
            public_amount: BigUint::from(102u32),
            rollup_fee_amount: BigUint::from(103u32),
            gas_relayer_fee_amount: BigUint::from(104u32),
            shielded_address: Some(String::from("shielded_address 1")),
            public_address: Some(String::from("public_address 1")),
            gas_relayer_address: Some(String::from("gas_relayer_address 1")),
            signature: Some(String::from("signature 1")),
            random_auditing_public_key: Some(BigUint::from(11111u32)),
            encrypted_auditor_notes: Some(vec![String::from("ean1"), String::from("ean2")]),
            transaction_type: TransactionType::Transfer,
            status: TransactionStatus::Init,
            error_message: Some(String::from("error_message 1")),
            transaction_hash: Some(String::from("transaction_hash 1")),
            wallet_id: String::from("wallet_id 1"),
        })
        .await
        .unwrap();
    assert_eq!(
        transaction,
        serde_json::from_str(&serde_json::to_string(&transaction).unwrap()).unwrap()
    );
}
