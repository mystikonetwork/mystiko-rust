use num_bigint::BigUint;
use prost::Message;
use mystiko_protos::core::document::v1::Deposit;
use mystiko_protos::core::v1::{BridgeType, DepositStatus};
use mystiko_utils::convert::biguint_to_bytes;

#[test]
fn test_wrappers() {
    let deposit = Deposit::builder()
        .status(DepositStatus::Queued)
        .bridge_type(BridgeType::Loop)
        .commitment_hash(biguint_to_bytes(&BigUint::from(1234_u32)))
        .hash_k(biguint_to_bytes(&BigUint::from(5678_u32)))
        .random_s(biguint_to_bytes(&BigUint::from(9999_u32)))
        .amount(biguint_to_bytes(&BigUint::from(1000000_u32)))
        .rollup_fee_amount(biguint_to_bytes(&BigUint::from(10001_u32)))
        .bridge_fee_amount(Some(biguint_to_bytes(&BigUint::from(10002_u32))))
        .executor_fee_amount(Some(biguint_to_bytes(&BigUint::from(10003_u32))))
        .service_fee_amount(Some(biguint_to_bytes(&BigUint::from(10004_u32))))
        .build();
    assert_eq!(deposit.commitment_hash_as_bigint(), BigUint::from(1234_u32));
    assert_eq!(deposit.hash_k_as_bigint(), BigUint::from(5678_u32));
    assert_eq!(deposit.random_s_as_bigint(), BigUint::from(9999_u32));
    assert_eq!(deposit.amount_as_bigint(), BigUint::from(1000000_u32));
    assert_eq!(deposit.rollup_fee_amount_as_bigint(), BigUint::from(10001_u32));
    assert_eq!(deposit.bridge_fee_amount_as_bigint(), Some(BigUint::from(10002_u32)));
    assert_eq!(deposit.executor_fee_amount_as_bigint(), Some(BigUint::from(10003_u32)));
    assert_eq!(deposit.service_fee_amount_as_bigint(), Some(BigUint::from(10004_u32)));
    let deposit_bytes = deposit.encode_to_vec();
    let deposit_json = serde_json::to_string(&deposit).unwrap();
    assert_eq!(
        deposit,
        Deposit::decode(std::io::Cursor::new(&deposit_bytes)).unwrap()
    );
    assert_eq!(deposit, serde_json::from_str(&deposit_json).unwrap());
}