use mystiko_core::{Spend, SpendCollection, SpendColumn};
use mystiko_protos::core::document::v1::Spend as ProtoSpend;
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use mystiko_types::BridgeType;
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
                bridge_type: BridgeType::Loop as i32,
                asset_symbol: String::from("asset_symbol 1"),
                asset_decimals: 18_u32,
                asset_address: Some(String::from("asset_address 1")),
                proof: Some(String::from("proof 1")),
                root_hash: Some(BigUint::from(1_u64)),
                input_commitments: vec![BigUint::from(11_u64), BigUint::from(22_u64)],
                output_commitments: Some(vec![BigUint::from(111_u64), BigUint::from(112_u64)]),
                nullifiers: Some(vec![BigUint::from(1111_u64), BigUint::from(1112_u64)]),
                signature_public_key: Some(String::from("signature_public_key 1")),
                signature_public_key_hashes: Some(vec![BigUint::from(11111_u64), BigUint::from(11112_u64)]),
                amount: 101_f64,
                decimal_amount: BigUint::from(1010000_u64),
                rollup_fee_amount: Some(103_f64),
                rollup_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
                rollup_fee_total_amount: Some(206_f64),
                rollup_fee_total_decimal_amount: Some(BigUint::from(2060000_u64)),
                gas_relayer_fee_amount: Some(104_f64),
                gas_relayer_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
                recipient: String::from("recipient 1"),
                gas_relayer_address: Some(String::from("gas_relayer_address 1")),
                gas_relayer_url: Some(String::from("https://gas_relayer_url_1.com")),
                signature: Some(String::from("signature 1")),
                random_auditing_public_key: Some(BigUint::from(111111_u64)),
                encrypted_auditor_notes: Some(vec![BigUint::from(1111111_u64), BigUint::from(1111112_u64)]),
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
                    bridge_type: BridgeType::Loop as i32,
                    asset_symbol: String::from("asset_symbol 2"),
                    asset_decimals: 18_u32,
                    asset_address: Some(String::from("asset_address 2")),
                    proof: Some(String::from("proof 2")),
                    root_hash: Some(BigUint::from(2_u64)),
                    input_commitments: vec![BigUint::from(21_u64), BigUint::from(22_u64)],
                    output_commitments: None,
                    nullifiers: None,
                    signature_public_key: Some(String::from("signature_public_key 2")),
                    signature_public_key_hashes: None,
                    amount: 201_f64,
                    decimal_amount: BigUint::from(2010000_u64),
                    rollup_fee_amount: Some(203_f64),
                    rollup_fee_decimal_amount: Some(BigUint::from(2030000_u64)),
                    rollup_fee_total_amount: Some(406_f64),
                    rollup_fee_total_decimal_amount: Some(BigUint::from(4060000_u64)),
                    gas_relayer_fee_amount: Some(204_f64),
                    gas_relayer_fee_decimal_amount: Some(BigUint::from(2040000_u64)),
                    recipient: String::from("recipient 2"),
                    gas_relayer_address: Some(String::from("gas_relayer_address 2")),
                    gas_relayer_url: Some(String::from("https://gas_relayer_url_2.com")),
                    signature: Some(String::from("signature 2")),
                    random_auditing_public_key: Some(BigUint::from(222222_u64)),
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
                    bridge_type: BridgeType::Loop as i32,
                    asset_symbol: String::from("asset_symbol 3"),
                    asset_decimals: 18_u32,
                    asset_address: Some(String::from("asset_address 3")),
                    proof: Some(String::from("proof 3")),
                    root_hash: Some(BigUint::from(3_u64)),
                    input_commitments: vec![BigUint::from(31_u64), BigUint::from(32_u64)],
                    output_commitments: Some(vec![]),
                    nullifiers: Some(vec![]),
                    signature_public_key: Some(String::from("signature_public_key 3")),
                    signature_public_key_hashes: Some(vec![]),
                    amount: 301_f64,
                    decimal_amount: BigUint::from(3010000_u64),
                    rollup_fee_amount: Some(303_f64),
                    rollup_fee_decimal_amount: Some(BigUint::from(3030000_u64)),
                    rollup_fee_total_amount: Some(606_f64),
                    rollup_fee_total_decimal_amount: Some(BigUint::from(6060000_u64)),
                    gas_relayer_fee_amount: Some(304_f64),
                    gas_relayer_fee_decimal_amount: Some(BigUint::from(3040000_u64)),
                    recipient: String::from("recipient 3"),
                    gas_relayer_address: Some(String::from("gas_relayer_address 3")),
                    gas_relayer_url: Some(String::from("https://gas_relayer_url_3.com")),
                    signature: Some(String::from("signature 3")),
                    random_auditing_public_key: Some(BigUint::from(333333_u64)),
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
    found_spend.data.amount = 10_f64;
    let updated_spend = spends.update(&found_spend).await.unwrap();
    assert_eq!(updated_spend.data, found_spend.data);
    inserted_spends[0].data.amount = 11_f64;
    inserted_spends[1].data.amount = 22_f64;
    inserted_spends[2].data.amount = 33_f64;
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
        .delete_by_filter(SubFilter::equal(SpendColumn::Recipient, "recipient 1"))
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
            bridge_type: BridgeType::Loop as i32,
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 18_u32,
            asset_address: Some(String::from("asset_address 1")),
            proof: Some(String::from("proof 1")),
            root_hash: Some(BigUint::from(1_u64)),
            input_commitments: vec![BigUint::from(11_u64), BigUint::from(22_u64)],
            output_commitments: Some(vec![BigUint::from(111_u64), BigUint::from(112_u64)]),
            nullifiers: Some(vec![BigUint::from(1111_u64), BigUint::from(1112_u64)]),
            signature_public_key: Some(String::from("signature_public_key 1")),
            signature_public_key_hashes: Some(vec![BigUint::from(11111_u64), BigUint::from(11112_u64)]),
            amount: 101_f64,
            decimal_amount: BigUint::from(1010000_u64),
            rollup_fee_amount: Some(103_f64),
            rollup_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
            rollup_fee_total_amount: Some(206_f64),
            rollup_fee_total_decimal_amount: Some(BigUint::from(2060000_u64)),
            gas_relayer_fee_amount: Some(104_f64),
            gas_relayer_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
            recipient: String::from("recipient 1"),
            gas_relayer_address: Some(String::from("gas_relayer_address 1")),
            gas_relayer_url: Some(String::from("https://gas_relayer_url_1.com")),
            signature: Some(String::from("signature 1")),
            random_auditing_public_key: Some(BigUint::from(111111_u64)),
            encrypted_auditor_notes: Some(vec![BigUint::from(1111111_u64), BigUint::from(1111112_u64)]),
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

#[test]
fn test_from_proto() {
    let proto = ProtoSpend::builder()
        .id(String::from("1234"))
        .created_at(1234567890u64)
        .updated_at(1234567891u64)
        .chain_id(1_u64)
        .contract_address(String::from("contract_address 1"))
        .asset_symbol(String::from("asset_symbol 1"))
        .asset_decimals(18_u32)
        .asset_address(String::from("asset_address 1"))
        .proof(String::from("proof 1"))
        .root_hash(String::from("1"))
        .input_commitments(vec![String::from("11"), String::from("12")])
        .output_commitments(vec![String::from("111"), String::from("112")])
        .nullifiers(vec![String::from("1111"), String::from("1112")])
        .signature_public_key(String::from("signature_public_key 1"))
        .signature_public_key_hashes(vec![String::from("11111"), String::from("11112")])
        .amount(101_f64)
        .decimal_amount(String::from("1010000"))
        .rollup_fee_amount(103_f64)
        .rollup_fee_decimal_amount(String::from("1030000"))
        .rollup_fee_total_amount(206_f64)
        .rollup_fee_total_decimal_amount(String::from("2060000"))
        .gas_relayer_fee_amount(104_f64)
        .gas_relayer_fee_decimal_amount(String::from("1040000"))
        .recipient(String::from("recipient 1"))
        .gas_relayer_address(String::from("gas_relayer_address 1"))
        .gas_relayer_url(String::from("https://gas_relayer_url_1.com"))
        .signature(String::from("signature 1"))
        .random_auditing_public_key(String::from("111111"))
        .encrypted_auditor_notes(vec![String::from("1111111"), String::from("1111112")])
        .spend_type(SpendType::Transfer as i32)
        .status(SpendStatus::Unspecified as i32)
        .error_message(String::from("error_message 1"))
        .transaction_hash(String::from("transaction_hash 1"))
        .wallet_id(String::from("wallet_id 1"))
        .build();
    let spend = Spend::document_from_proto(proto).unwrap();
    assert_eq!(spend.id, String::from("1234"));
    assert_eq!(spend.created_at, 1234567890u64);
    assert_eq!(spend.updated_at, 1234567891u64);
    assert_eq!(spend.data.chain_id, 1_u64);
    assert_eq!(spend.data.contract_address, String::from("contract_address 1"));
    assert_eq!(spend.data.bridge_type, BridgeType::Loop as i32);
    assert_eq!(spend.data.asset_symbol, String::from("asset_symbol 1"));
    assert_eq!(spend.data.asset_decimals, 18_u32);
    assert_eq!(spend.data.asset_address, Some(String::from("asset_address 1")));
    assert_eq!(spend.data.proof, Some(String::from("proof 1")));
    assert_eq!(spend.data.root_hash, Some(BigUint::from(1_u64)));
    assert_eq!(
        spend.data.input_commitments,
        vec![BigUint::from(11_u64), BigUint::from(12_u64)]
    );
    assert_eq!(
        spend.data.output_commitments,
        Some(vec![BigUint::from(111_u64), BigUint::from(112_u64)])
    );
    assert_eq!(
        spend.data.nullifiers,
        Some(vec![BigUint::from(1111_u64), BigUint::from(1112_u64)])
    );
    assert_eq!(
        spend.data.signature_public_key,
        Some(String::from("signature_public_key 1"))
    );
    assert_eq!(
        spend.data.signature_public_key_hashes,
        Some(vec![BigUint::from(11111_u64), BigUint::from(11112_u64)])
    );
    assert_eq!(spend.data.amount, 101_f64);
    assert_eq!(spend.data.decimal_amount, BigUint::from(1010000_u64));
    assert_eq!(spend.data.rollup_fee_amount, Some(103_f64));
    assert_eq!(spend.data.rollup_fee_decimal_amount, Some(BigUint::from(1030000_u64)));
    assert_eq!(spend.data.rollup_fee_total_amount, Some(206_f64));
    assert_eq!(
        spend.data.rollup_fee_total_decimal_amount,
        Some(BigUint::from(2060000_u64))
    );
    assert_eq!(spend.data.gas_relayer_fee_amount, Some(104_f64));
    assert_eq!(
        spend.data.gas_relayer_fee_decimal_amount,
        Some(BigUint::from(1040000_u64))
    );
    assert_eq!(spend.data.recipient, String::from("recipient 1"));
    assert_eq!(
        spend.data.gas_relayer_address,
        Some(String::from("gas_relayer_address 1"))
    );
    assert_eq!(
        spend.data.gas_relayer_url,
        Some(String::from("https://gas_relayer_url_1.com"))
    );
    assert_eq!(spend.data.signature, Some(String::from("signature 1")));
    assert_eq!(spend.data.random_auditing_public_key, Some(BigUint::from(111111_u64)));
    assert_eq!(
        spend.data.encrypted_auditor_notes,
        Some(vec![BigUint::from(1111111_u64), BigUint::from(1111112_u64)])
    );
    assert_eq!(spend.data.spend_type, SpendType::Transfer as i32);
    assert_eq!(spend.data.status, SpendStatus::Unspecified as i32);
    assert_eq!(spend.data.error_message, Some(String::from("error_message 1")));
    assert_eq!(spend.data.transaction_hash, Some(String::from("transaction_hash 1")));
    assert_eq!(spend.data.wallet_id, String::from("wallet_id 1"));
}

#[test]
fn test_into_proto() {
    let spend = Document::new(
        String::from("1234"),
        1234567890_u64,
        1234567891_u64,
        Spend {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            bridge_type: BridgeType::Loop as i32,
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 18_u32,
            asset_address: Some(String::from("asset_address 1")),
            proof: Some(String::from("proof 1")),
            root_hash: Some(BigUint::from(1_u64)),
            input_commitments: vec![BigUint::from(11_u64), BigUint::from(12_u64)],
            output_commitments: Some(vec![BigUint::from(111_u64), BigUint::from(112_u64)]),
            nullifiers: Some(vec![BigUint::from(1111_u64), BigUint::from(1112_u64)]),
            signature_public_key: Some(String::from("signature_public_key 1")),
            signature_public_key_hashes: Some(vec![BigUint::from(11111_u64), BigUint::from(11112_u64)]),
            amount: 101_f64,
            decimal_amount: BigUint::from(1010000_u64),
            rollup_fee_amount: Some(103_f64),
            rollup_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
            rollup_fee_total_amount: Some(206_f64),
            rollup_fee_total_decimal_amount: Some(BigUint::from(2060000_u64)),
            gas_relayer_fee_amount: Some(104_f64),
            gas_relayer_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
            recipient: String::from("recipient 1"),
            gas_relayer_address: Some(String::from("gas_relayer_address 1")),
            gas_relayer_url: Some(String::from("https://gas_relayer_url_1.com")),
            signature: Some(String::from("signature 1")),
            random_auditing_public_key: Some(BigUint::from(111111_u64)),
            encrypted_auditor_notes: Some(vec![BigUint::from(1111111_u64), BigUint::from(1111112_u64)]),
            spend_type: SpendType::Transfer as i32,
            status: SpendStatus::Unspecified as i32,
            error_message: Some(String::from("error_message 1")),
            transaction_hash: Some(String::from("transaction_hash 1")),
            wallet_id: String::from("wallet_id 1"),
        },
    );
    let proto = Spend::document_into_proto(spend);
    assert_eq!(proto.id, String::from("1234"));
    assert_eq!(proto.created_at, 1234567890_u64);
    assert_eq!(proto.updated_at, 1234567891_u64);
    assert_eq!(proto.chain_id, 1_u64);
    assert_eq!(proto.contract_address, String::from("contract_address 1"));
    assert_eq!(proto.bridge_type, BridgeType::Loop as i32);
    assert_eq!(proto.asset_symbol, String::from("asset_symbol 1"));
    assert_eq!(proto.asset_decimals, 18_u32);
    assert_eq!(proto.asset_address.unwrap(), String::from("asset_address 1"));
    assert_eq!(proto.proof.unwrap(), String::from("proof 1"));
    assert_eq!(proto.root_hash, Some(String::from("1")));
    assert_eq!(proto.input_commitments, vec![String::from("11"), String::from("12")]);
    assert_eq!(proto.output_commitments, vec![String::from("111"), String::from("112")]);
    assert_eq!(proto.nullifiers, vec![String::from("1111"), String::from("1112")]);
    assert_eq!(
        proto.signature_public_key.unwrap(),
        String::from("signature_public_key 1")
    );
    assert_eq!(
        proto.signature_public_key_hashes,
        vec![String::from("11111"), String::from("11112")]
    );
    assert_eq!(proto.amount, 101_f64);
    assert_eq!(proto.decimal_amount, String::from("1010000"));
    assert_eq!(proto.rollup_fee_amount.unwrap(), 103_f64);
    assert_eq!(proto.rollup_fee_decimal_amount.unwrap(), String::from("1030000"));
    assert_eq!(proto.rollup_fee_total_amount.unwrap(), 206_f64);
    assert_eq!(proto.rollup_fee_total_decimal_amount.unwrap(), String::from("2060000"));
    assert_eq!(proto.gas_relayer_fee_amount.unwrap(), 104_f64);
    assert_eq!(proto.gas_relayer_fee_decimal_amount.unwrap(), String::from("1040000"));
    assert_eq!(proto.recipient, String::from("recipient 1"));
    assert_eq!(
        proto.gas_relayer_address.unwrap(),
        String::from("gas_relayer_address 1")
    );
    assert_eq!(
        proto.gas_relayer_url.unwrap(),
        String::from("https://gas_relayer_url_1.com")
    );
    assert_eq!(proto.signature.unwrap(), String::from("signature 1"));
    assert_eq!(proto.random_auditing_public_key.unwrap(), String::from("111111"));
    assert_eq!(
        proto.encrypted_auditor_notes,
        vec![String::from("1111111"), String::from("1111112")]
    );
    assert_eq!(proto.spend_type, SpendType::Transfer as i32);
    assert_eq!(proto.status, SpendStatus::Unspecified as i32);
    assert_eq!(proto.error_message.unwrap(), String::from("error_message 1"));
    assert_eq!(proto.transaction_hash.unwrap(), String::from("transaction_hash 1"));
    assert_eq!(proto.wallet_id, String::from("wallet_id 1"));
}
