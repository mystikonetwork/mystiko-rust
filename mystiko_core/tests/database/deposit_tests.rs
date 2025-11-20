use mystiko_core::{Deposit, DepositCollection, DepositColumn};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Deposit as ProtoDeposit;
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::storage::v1::{ConditionOperator, QueryFilter, SubFilter};
use mystiko_storage::{Collection, Document, SqlStatementFormatter};
use mystiko_storage_sqlite::SqliteStorage;
use num_bigint::BigUint;
use std::sync::Arc;

async fn create_deposits() -> DepositCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorage::from_memory().await.unwrap();
    let deposits = DepositCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
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
                commitment_hash: BigUint::from(1_u64),
                hash_k: BigUint::from(11_u64),
                random_s: BigUint::from(111_u64),
                encrypted_note: String::from("encrypted_note 1"),
                asset_symbol: String::from("asset_symbol 1"),
                asset_decimals: 18,
                asset_address: Some(String::from("asset_address 1")),
                bridge_type: BridgeType::Axelar as i32,
                amount: 101_f64,
                decimal_amount: BigUint::from(1010000_u64),
                rollup_fee_amount: 102_f64,
                rollup_fee_decimal_amount: BigUint::from(1020000_u64),
                bridge_fee_amount: Some(103_f64),
                bridge_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
                bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 1")),
                bridge_fee_asset_symbol: Some(String::from("MTT")),
                bridge_fee_asset_decimals: Some(18),
                executor_fee_amount: Some(104_f64),
                executor_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
                executor_fee_asset_address: Some(String::from("executor_fee_asset_address 1")),
                executor_fee_asset_symbol: Some(String::from("STT")),
                executor_fee_asset_decimals: Some(18),
                shielded_address: String::from("shielded_address 1"),
                status: DepositStatus::Unspecified as i32,
                error_message: Some(String::from("error_message 1")),
                wallet_id: String::from("wallet_id 1"),
                dst_chain_id: 11,
                dst_chain_contract_address: String::from("dst_chain_contract_address 1"),
                dst_pool_address: String::from("dst_pool_address 1"),
                asset_approve_transaction_hash: Some(vec![String::from("asset_approve_transaction_hash 1")]),
                src_chain_transaction_hash: Some(String::from("transaction_hash 1")),
                queued_transaction_hash: Some(String::from("relay_transaction_hash 1")),
                included_transaction_hash: Some(String::from("rollup_transaction_hash 1")),
            })
            .await
            .unwrap(),
    );
    inserted_deposits.extend(
        deposits
            .insert_batch(&[
                Deposit {
                    chain_id: 2,
                    contract_address: String::from("contract_address 2"),
                    pool_address: String::from("pool_address 2"),
                    commitment_hash: BigUint::from(2_u64),
                    hash_k: BigUint::from(22_u64),
                    random_s: BigUint::from(222_u64),
                    encrypted_note: String::from("encrypted_note 2"),
                    asset_symbol: String::from("asset_symbol 2"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("asset_address 2")),
                    bridge_type: BridgeType::Celer as i32,
                    amount: 201_f64,
                    decimal_amount: BigUint::from(2010000_u64),
                    rollup_fee_amount: 202_f64,
                    rollup_fee_decimal_amount: BigUint::from(2020000_u64),
                    bridge_fee_amount: Some(203_f64),
                    bridge_fee_decimal_amount: Some(BigUint::from(2030000_u64)),
                    bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 2")),
                    bridge_fee_asset_symbol: Some(String::from("MTT")),
                    bridge_fee_asset_decimals: Some(18),
                    executor_fee_amount: Some(204_f64),
                    executor_fee_decimal_amount: Some(BigUint::from(2040000_u64)),
                    executor_fee_asset_address: Some(String::from("executor_fee_asset_address 2")),
                    executor_fee_asset_symbol: Some(String::from("STT")),
                    executor_fee_asset_decimals: Some(18),
                    shielded_address: String::from("shielded_address 2"),
                    status: DepositStatus::Included as i32,
                    error_message: Some(String::from("error_message 2")),
                    wallet_id: String::from("wallet_id 2"),
                    dst_chain_id: 22,
                    dst_chain_contract_address: String::from("dst_chain_contract_address 2"),
                    dst_pool_address: String::from("dst_pool_address 2"),
                    asset_approve_transaction_hash: Some(vec![String::from("asset_approve_transaction_hash 2")]),
                    src_chain_transaction_hash: Some(String::from("transaction_hash 2")),
                    queued_transaction_hash: Some(String::from("relay_transaction_hash 2")),
                    included_transaction_hash: Some(String::from("rollup_transaction_hash 2")),
                },
                Deposit {
                    chain_id: 3,
                    contract_address: String::from("contract_address 3"),
                    pool_address: String::from("pool_address 3"),
                    commitment_hash: BigUint::from(3_u64),
                    hash_k: BigUint::from(33_u64),
                    random_s: BigUint::from(333_u64),
                    encrypted_note: String::from("encrypted_note 3"),
                    asset_symbol: String::from("asset_symbol 3"),
                    asset_decimals: 18,
                    asset_address: Some(String::from("asset_address 3")),
                    bridge_type: BridgeType::LayerZero as i32,
                    amount: 301_f64,
                    decimal_amount: BigUint::from(3010000_u64),
                    rollup_fee_amount: 302_f64,
                    rollup_fee_decimal_amount: BigUint::from(3020000_u64),
                    bridge_fee_amount: Some(303_f64),
                    bridge_fee_decimal_amount: Some(BigUint::from(3030000_u64)),
                    bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 3")),
                    bridge_fee_asset_symbol: Some(String::from("MTT")),
                    bridge_fee_asset_decimals: Some(18),
                    executor_fee_amount: Some(304_f64),
                    executor_fee_decimal_amount: Some(BigUint::from(3040000_u64)),
                    executor_fee_asset_address: Some(String::from("executor_fee_asset_address 3")),
                    executor_fee_asset_symbol: Some(String::from("STT")),
                    executor_fee_asset_decimals: Some(18),
                    shielded_address: String::from("shielded_address 3"),
                    status: DepositStatus::Queued as i32,
                    error_message: Some(String::from("error_message 3")),
                    wallet_id: String::from("wallet_id 3"),
                    dst_chain_id: 33,
                    dst_chain_contract_address: String::from("dst_chain_contract_address 3"),
                    dst_pool_address: String::from("dst_pool_address 3"),
                    asset_approve_transaction_hash: Some(vec![String::from("asset_approve_transaction_hash 3")]),
                    src_chain_transaction_hash: Some(String::from("transaction_hash 3")),
                    queued_transaction_hash: Some(String::from("relay_transaction_hash 3")),
                    included_transaction_hash: Some(String::from("rollup_transaction_hash 3")),
                },
            ])
            .await
            .unwrap(),
    );

    // testing count/count_all
    assert_eq!(deposits.count_all().await.unwrap(), 3);
    assert_eq!(
        deposits
            .count(SubFilter::equal(DepositColumn::HashK, BigUint::from(22_u64)))
            .await
            .unwrap(),
        1
    );

    // testing find/find_all/find_one/find_by_id
    let mut found_deposits = deposits.find_all().await.unwrap();
    assert_eq!(found_deposits, inserted_deposits);
    let found_first_deposit = deposits
        .find_one(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(1)
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(found_first_deposit.data.status, DepositStatus::Unspecified as i32,);
    found_deposits = deposits
        .find(
            QueryFilter::builder()
                .conditions_operator(ConditionOperator::And)
                .limit(2)
                .offset(1)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(found_deposits, inserted_deposits[1..]);
    let mut found_deposit = deposits
        .find_one(SubFilter::equal(DepositColumn::RandomS, BigUint::from(222_u64)))
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
    deposits.delete_batch(&[inserted_deposits[1].clone()]).await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 1);
    deposits.insert(&inserted_deposits[0].data).await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 2);
    deposits
        .delete_by_filter(SubFilter::equal(DepositColumn::ShieldedAddress, "shielded_address 1"))
        .await
        .unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 1);
    deposits.delete_all().await.unwrap();
    assert_eq!(deposits.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_deposit_serde() {
    let deposits = create_deposits().await;
    let deposit = deposits
        .insert(&Deposit {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            pool_address: String::from("pool_address 1"),
            commitment_hash: BigUint::from(1_u64),
            hash_k: BigUint::from(11_u64),
            random_s: BigUint::from(111_u64),
            encrypted_note: String::from("encrypted_note 1"),
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 18,
            asset_address: Some(String::from("asset_address 1")),
            bridge_type: BridgeType::Axelar as i32,
            amount: 101_f64,
            decimal_amount: BigUint::from(1010000_u64),
            rollup_fee_amount: 102_f64,
            rollup_fee_decimal_amount: BigUint::from(1020000_u64),
            bridge_fee_amount: Some(103_f64),
            bridge_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
            bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 1")),
            bridge_fee_asset_symbol: Some(String::from("MTT")),
            bridge_fee_asset_decimals: Some(18),
            executor_fee_amount: Some(104_f64),
            executor_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
            executor_fee_asset_address: Some(String::from("executor_fee_asset_address 1")),
            executor_fee_asset_symbol: Some(String::from("STT")),
            executor_fee_asset_decimals: Some(18),
            shielded_address: String::from("shielded_address 1"),
            status: DepositStatus::Unspecified as i32,
            error_message: Some(String::from("error_message 1")),
            wallet_id: String::from("wallet_id 1"),
            dst_chain_id: 11,
            dst_chain_contract_address: String::from("dst_chain_contract_address 1"),
            dst_pool_address: String::from("dst_pool_address 1"),
            asset_approve_transaction_hash: Some(vec![String::from("asset_approve_transaction_hash 1")]),
            src_chain_transaction_hash: Some(String::from("transaction_hash 1")),
            queued_transaction_hash: Some(String::from("relay_transaction_hash 1")),
            included_transaction_hash: Some(String::from("rollup_transaction_hash 1")),
        })
        .await
        .unwrap();
    assert_eq!(
        deposit,
        serde_json::from_str(&serde_json::to_string(&deposit).unwrap()).unwrap()
    );
}

#[test]
fn test_from_proto() {
    let proto = ProtoDeposit::builder()
        .id("1234")
        .created_at(1234567890_u64)
        .updated_at(1234567891_u64)
        .chain_id(1_u64)
        .contract_address(String::from("contract_address 1"))
        .pool_address(String::from("pool_address 1"))
        .commitment_hash(String::from("1"))
        .hash_k(String::from("11"))
        .random_s(String::from("111"))
        .encrypted_note(String::from("encrypted_note 1"))
        .asset_symbol(String::from("asset_symbol 1"))
        .asset_decimals(18_u32)
        .asset_address(String::from("asset_address 1"))
        .bridge_type(BridgeType::Axelar as i32)
        .amount(101_f64)
        .decimal_amount(String::from("1010000"))
        .rollup_fee_amount(102_f64)
        .rollup_fee_decimal_amount(String::from("1020000"))
        .bridge_fee_amount(103_f64)
        .bridge_fee_decimal_amount(String::from("1030000"))
        .bridge_fee_asset_address(String::from("bridge_fee_asset_address 1"))
        .bridge_fee_asset_symbol(String::from("MTT"))
        .bridge_fee_asset_decimals(18_u32)
        .executor_fee_amount(104_f64)
        .executor_fee_decimal_amount(String::from("1040000"))
        .executor_fee_asset_address(String::from("executor_fee_asset_address 1"))
        .executor_fee_asset_symbol(String::from("STT"))
        .executor_fee_asset_decimals(18_u32)
        .shielded_address(String::from("shielded_address 1"))
        .status(DepositStatus::Unspecified as i32)
        .error_message(String::from("error_message 1"))
        .wallet_id(String::from("wallet_id 1"))
        .dst_chain_id(11_u64)
        .dst_chain_contract_address(String::from("dst_chain_contract_address 1"))
        .dst_pool_address(String::from("dst_pool_address 1"))
        .asset_approve_transaction_hash(vec![String::from("asset_approve_transaction_hash 1")])
        .src_chain_transaction_hash(String::from("transaction_hash 1"))
        .queued_transaction_hash(String::from("relay_transaction_hash 1"))
        .included_transaction_hash(String::from("rollup_transaction_hash 1"))
        .build();
    let deposit = Deposit::document_from_proto(proto).unwrap();
    assert_eq!(deposit.id, String::from("1234"));
    assert_eq!(deposit.created_at, 1234567890_u64);
    assert_eq!(deposit.updated_at, 1234567891_u64);
    assert_eq!(deposit.data.chain_id, 1);
    assert_eq!(deposit.data.contract_address, String::from("contract_address 1"));
    assert_eq!(deposit.data.pool_address, String::from("pool_address 1"));
    assert_eq!(deposit.data.commitment_hash, BigUint::from(1_u64));
    assert_eq!(deposit.data.hash_k, BigUint::from(11_u64));
    assert_eq!(deposit.data.random_s, BigUint::from(111_u64));
    assert_eq!(deposit.data.encrypted_note, String::from("encrypted_note 1"));
    assert_eq!(deposit.data.asset_symbol, String::from("asset_symbol 1"));
    assert_eq!(deposit.data.asset_decimals, 18);
    assert_eq!(deposit.data.asset_address, Some(String::from("asset_address 1")));
    assert_eq!(deposit.data.bridge_type, BridgeType::Axelar as i32);
    assert_eq!(deposit.data.amount, 101_f64);
    assert_eq!(deposit.data.decimal_amount, BigUint::from(1010000_u64));
    assert_eq!(deposit.data.rollup_fee_amount, 102_f64);
    assert_eq!(deposit.data.rollup_fee_decimal_amount, BigUint::from(1020000_u64));
    assert_eq!(deposit.data.bridge_fee_amount, Some(103_f64));
    assert_eq!(deposit.data.bridge_fee_decimal_amount, Some(BigUint::from(1030000_u64)));
    assert_eq!(
        deposit.data.bridge_fee_asset_address,
        Some(String::from("bridge_fee_asset_address 1"))
    );
    assert_eq!(deposit.data.bridge_fee_asset_symbol, Some(String::from("MTT")));
    assert_eq!(deposit.data.bridge_fee_asset_decimals, Some(18));
    assert_eq!(deposit.data.executor_fee_amount, Some(104_f64));
    assert_eq!(
        deposit.data.executor_fee_decimal_amount,
        Some(BigUint::from(1040000_u64))
    );
    assert_eq!(
        deposit.data.executor_fee_asset_address,
        Some(String::from("executor_fee_asset_address 1"))
    );
    assert_eq!(deposit.data.executor_fee_asset_symbol, Some(String::from("STT")));
    assert_eq!(deposit.data.executor_fee_asset_decimals, Some(18));
    assert_eq!(deposit.data.shielded_address, String::from("shielded_address 1"));
    assert_eq!(deposit.data.status, DepositStatus::Unspecified as i32);
    assert_eq!(deposit.data.error_message, Some(String::from("error_message 1")));
    assert_eq!(deposit.data.wallet_id, String::from("wallet_id 1"));
    assert_eq!(deposit.data.dst_chain_id, 11);
    assert_eq!(
        deposit.data.dst_chain_contract_address,
        String::from("dst_chain_contract_address 1")
    );
    assert_eq!(deposit.data.dst_pool_address, String::from("dst_pool_address 1"));
    assert_eq!(
        deposit.data.asset_approve_transaction_hash,
        Some(vec![String::from("asset_approve_transaction_hash 1")])
    );
    assert_eq!(
        deposit.data.src_chain_transaction_hash,
        Some(String::from("transaction_hash 1"))
    );
    assert_eq!(
        deposit.data.queued_transaction_hash,
        Some(String::from("relay_transaction_hash 1"))
    );
    assert_eq!(
        deposit.data.included_transaction_hash,
        Some(String::from("rollup_transaction_hash 1"))
    );
}

#[test]
fn test_into_proto() {
    let deposit = Document::new(
        String::from("1234"),
        1234567890_u64,
        1234567891_u64,
        Deposit {
            chain_id: 1,
            contract_address: String::from("contract_address 1"),
            pool_address: String::from("pool_address 1"),
            commitment_hash: BigUint::from(1_u64),
            hash_k: BigUint::from(11_u64),
            random_s: BigUint::from(111_u64),
            encrypted_note: String::from("encrypted_note 1"),
            asset_symbol: String::from("asset_symbol 1"),
            asset_decimals: 18,
            asset_address: Some(String::from("asset_address 1")),
            bridge_type: BridgeType::Axelar as i32,
            amount: 101_f64,
            decimal_amount: BigUint::from(1010000_u64),
            rollup_fee_amount: 102_f64,
            rollup_fee_decimal_amount: BigUint::from(1020000_u64),
            bridge_fee_amount: Some(103_f64),
            bridge_fee_decimal_amount: Some(BigUint::from(1030000_u64)),
            bridge_fee_asset_address: Some(String::from("bridge_fee_asset_address 1")),
            bridge_fee_asset_symbol: Some(String::from("MTT")),
            bridge_fee_asset_decimals: Some(18),
            executor_fee_amount: Some(104_f64),
            executor_fee_decimal_amount: Some(BigUint::from(1040000_u64)),
            executor_fee_asset_address: Some(String::from("executor_fee_asset_address 1")),
            executor_fee_asset_symbol: Some(String::from("STT")),
            executor_fee_asset_decimals: Some(18),
            shielded_address: String::from("shielded_address 1"),
            status: DepositStatus::Unspecified as i32,
            error_message: Some(String::from("error_message 1")),
            wallet_id: String::from("wallet_id 1"),
            dst_chain_id: 11,
            dst_chain_contract_address: String::from("dst_chain_contract_address 1"),
            dst_pool_address: String::from("dst_pool_address 1"),
            asset_approve_transaction_hash: Some(vec![String::from("asset_approve_transaction_hash 1")]),
            src_chain_transaction_hash: Some(String::from("transaction_hash 1")),
            queued_transaction_hash: Some(String::from("relay_transaction_hash 1")),
            included_transaction_hash: Some(String::from("rollup_transaction_hash 1")),
        },
    );
    let proto = Deposit::document_into_proto(deposit);
    assert_eq!(proto.id, String::from("1234"));
    assert_eq!(proto.created_at, 1234567890_u64);
    assert_eq!(proto.updated_at, 1234567891_u64);
    assert_eq!(proto.chain_id, 1);
    assert_eq!(proto.contract_address, String::from("contract_address 1"));
    assert_eq!(proto.pool_address, String::from("pool_address 1"));
    assert_eq!(proto.commitment_hash, String::from("1"));
    assert_eq!(proto.hash_k, String::from("11"));
    assert_eq!(proto.random_s, String::from("111"));
    assert_eq!(proto.encrypted_note, String::from("encrypted_note 1"));
    assert_eq!(proto.asset_symbol, String::from("asset_symbol 1"));
    assert_eq!(proto.asset_decimals, 18_u32);
    assert_eq!(proto.asset_address, Some(String::from("asset_address 1")));
    assert_eq!(proto.bridge_type, BridgeType::Axelar as i32);
    assert_eq!(proto.amount, 101_f64);
    assert_eq!(proto.decimal_amount, String::from("1010000"));
    assert_eq!(proto.rollup_fee_amount, 102_f64);
    assert_eq!(proto.rollup_fee_decimal_amount, String::from("1020000"));
    assert_eq!(proto.bridge_fee_amount, Some(103_f64));
    assert_eq!(proto.bridge_fee_decimal_amount, Some(String::from("1030000")));
    assert_eq!(
        proto.bridge_fee_asset_address,
        Some(String::from("bridge_fee_asset_address 1"))
    );
    assert_eq!(proto.bridge_fee_asset_symbol, Some(String::from("MTT")));
    assert_eq!(proto.bridge_fee_asset_decimals, Some(18_u32));
    assert_eq!(proto.executor_fee_amount, Some(104_f64));
    assert_eq!(proto.executor_fee_decimal_amount, Some(String::from("1040000")));
    assert_eq!(
        proto.executor_fee_asset_address,
        Some(String::from("executor_fee_asset_address 1"))
    );
    assert_eq!(proto.executor_fee_asset_symbol, Some(String::from("STT")));
    assert_eq!(proto.executor_fee_asset_decimals, Some(18_u32));
    assert_eq!(proto.shielded_address, String::from("shielded_address 1"));
    assert_eq!(proto.status, DepositStatus::Unspecified as i32);
    assert_eq!(proto.error_message, Some(String::from("error_message 1")));
    assert_eq!(proto.wallet_id, String::from("wallet_id 1"));
    assert_eq!(proto.dst_chain_id, 11);
    assert_eq!(
        proto.dst_chain_contract_address,
        String::from("dst_chain_contract_address 1")
    );
    assert_eq!(proto.dst_pool_address, String::from("dst_pool_address 1"));
    assert_eq!(
        proto.asset_approve_transaction_hash,
        vec![String::from("asset_approve_transaction_hash 1")]
    );
    assert_eq!(
        proto.src_chain_transaction_hash,
        Some(String::from("transaction_hash 1"))
    );
    assert_eq!(
        proto.queued_transaction_hash,
        Some(String::from("relay_transaction_hash 1"))
    );
    assert_eq!(
        proto.included_transaction_hash,
        Some(String::from("rollup_transaction_hash 1"))
    );
}
