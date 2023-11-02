use mystiko_core::{Spend, SpendCollection, SpendColumn};
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use num_bigint::BigUint;
use std::sync::Arc;

async fn create_spends() -> SpendCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let spends = SpendCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
    spends.migrate().await.unwrap();
    assert!(spends.collection_exists().await.unwrap());
    spends
}

#[tokio::test]
async fn test_spend_crud() {
    let spends = create_spends().await;

    // testing insert/insert_batch
    let mut inserted_spends: Vec<Document<Spend>> = Vec::new();
    inserted_spends.push(
        spends
            .insert(&Spend {
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
                spend_type: SpendType::Transfer as i32,
                status: SpendStatus::Unspecified as i32,
                error_message: Some(String::from("error_message 1")),
                transaction_hash: Some(String::from("transaction_hash 1")),
                wallet_id: String::from("wallet_id 1"),
            })
            .await
            .unwrap(),
    );
    inserted_spends.extend(
        spends
            .insert_batch(&vec![
                Spend {
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
                    spend_type: SpendType::Withdraw as i32,
                    status: SpendStatus::Pending as i32,
                    error_message: Some(String::from("error_message 2")),
                    transaction_hash: Some(String::from("transaction_hash 2")),
                    wallet_id: String::from("wallet_id 2"),
                },
                Spend {
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
                    spend_type: SpendType::Withdraw as i32,
                    status: SpendStatus::Failed as i32,
                    error_message: Some(String::from("error_message 3")),
                    transaction_hash: Some(String::from("transaction_hash 3")),
                    wallet_id: String::from("wallet_id 3"),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(spends.count_all().await.unwrap(), 3);
    assert_eq!(
        spends
            .count(SubFilter::equal(
                SpendColumn::SignaturePublicKey,
                "signature_public_key 2"
            ))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_spends = spends.find_all().await.unwrap();
    assert_eq!(found_spends, inserted_spends);
    let found_first_spend = spends
        .find_one(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(1)
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_first_spend.data.status, SpendStatus::Unspecified as i32,);
    found_spends = spends
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_spends, inserted_spends[1..]);
    let mut found_spend = spends
        .find_one(SubFilter::equal(SpendColumn::AssetSymbol, "asset_symbol 2"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_spend, inserted_spends[1]);
    found_spend = spends.find_by_id(&inserted_spends[2].id).await.unwrap().unwrap();
    assert_eq!(found_spend, inserted_spends[2]);

    // testing update/update_batch
    found_spend.data.asset_decimals = 10;
    let updated_spend = spends.update(&found_spend).await.unwrap();
    assert_eq!(updated_spend.data, found_spend.data);
    inserted_spends[0].data.asset_decimals = 11;
    inserted_spends[1].data.asset_decimals = 22;
    inserted_spends[2].data.asset_decimals = 33;
    found_spends = spends.update_batch(&inserted_spends).await.unwrap();
    assert_eq!(found_spends[0].data, inserted_spends[0].data);
    assert_eq!(found_spends[1].data, inserted_spends[1].data);
    assert_eq!(found_spends[2].data, inserted_spends[2].data);

    // testing delete/delete_batch/delete_by_filter/delete_all
    spends.delete(&inserted_spends[0]).await.unwrap();
    assert_eq!(spends.count_all().await.unwrap(), 2);
    spends.delete_batch(&vec![inserted_spends[1].clone()]).await.unwrap();
    assert_eq!(spends.count_all().await.unwrap(), 1);
    spends.insert(&inserted_spends[0].data).await.unwrap();
    assert_eq!(spends.count_all().await.unwrap(), 2);
    spends
        .delete_by_filter(SubFilter::equal(SpendColumn::ShieldedAddress, "shielded_address 1"))
        .await
        .unwrap();
    assert_eq!(spends.count_all().await.unwrap(), 1);
    spends.delete_all().await.unwrap();
    assert_eq!(spends.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_spend_serde() {
    let spends = create_spends().await;
    let spend = spends
        .insert(&Spend {
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
            spend_type: SpendType::Transfer as i32,
            status: SpendStatus::Unspecified as i32,
            error_message: Some(String::from("error_message 1")),
            transaction_hash: Some(String::from("transaction_hash 1")),
            wallet_id: String::from("wallet_id 1"),
        })
        .await
        .unwrap();
    assert_eq!(
        spend,
        serde_json::from_str(&serde_json::to_string(&spend).unwrap()).unwrap()
    );
}
