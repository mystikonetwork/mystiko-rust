use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Deposit;
use mystiko_protos::core::v1::DepositStatus;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let deposit = Deposit::builder().build();
    assert!(deposit.bridge_fee_decimal_amount_as_biguint().unwrap().is_none());
    assert!(deposit.executor_fee_decimal_amount_as_biguint().unwrap().is_none());
    let deposit = Deposit::builder()
        .status(DepositStatus::Queued)
        .bridge_type(BridgeType::Loop)
        .commitment_hash("1234".to_string())
        .hash_k("5678".to_string())
        .random_s("9999".to_string())
        .decimal_amount("1000000000000000000".to_string())
        .rollup_fee_decimal_amount("2000000000000000000".to_string())
        .bridge_fee_decimal_amount("3000000000000000000".to_string())
        .executor_fee_decimal_amount("4000000000000000000".to_string())
        .build();
    assert_eq!(deposit.commitment_hash_as_biguint().unwrap(), BigUint::from(1234_u32));
    assert_eq!(deposit.hash_k_as_biguint().unwrap(), BigUint::from(5678_u32));
    assert_eq!(deposit.random_s_as_biguint().unwrap(), BigUint::from(9999_u32));
    assert_eq!(
        deposit.decimal_amount_as_biguint().unwrap(),
        BigUint::from(1000000000000000000_u64)
    );
    assert_eq!(
        deposit.rollup_fee_decimal_amount_as_biguint().unwrap(),
        BigUint::from(2000000000000000000_u64)
    );
    assert_eq!(
        deposit.bridge_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(3000000000000000000_u64))
    );
    assert_eq!(
        deposit.executor_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(4000000000000000000_u64))
    );
    let deposit_bytes = deposit.encode_to_vec();
    let deposit_json = serde_json::to_string(&deposit).unwrap();
    assert_eq!(deposit, Deposit::try_from(&deposit_bytes).unwrap());
    assert_eq!(deposit, serde_json::from_str(&deposit_json).unwrap());
}
