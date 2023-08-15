use mystiko_protos::core::document::v1::Transaction;
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let transaction = Transaction::builder()
        .root_hash(biguint_to_bytes(&BigUint::from(1234_u32)))
        .input_commitments(vec![
            biguint_to_bytes(&BigUint::from(5678_u32)),
            biguint_to_bytes(&BigUint::from(7890_u32)),
        ])
        .output_commitments(vec![
            biguint_to_bytes(&BigUint::from(10001_u32)),
            biguint_to_bytes(&BigUint::from(10002_u32)),
        ])
        .nullifiers(vec![
            biguint_to_bytes(&BigUint::from(9999_u32)),
            biguint_to_bytes(&BigUint::from(9998_u32)),
        ])
        .amount(biguint_to_bytes(&BigUint::from(1000000_u32)))
        .public_amount(biguint_to_bytes(&BigUint::from(1000001_u32)))
        .rollup_fee_amount(Some(biguint_to_bytes(&BigUint::from(1000002_u32))))
        .gas_relayer_fee_amount(Some(biguint_to_bytes(&BigUint::from(1000003_u32))))
        .random_auditing_public_key(Some(biguint_to_bytes(&BigUint::from(1000004_u32))))
        .build();
    assert_eq!(transaction.root_hash_as_bigint(), BigUint::from(1234_u32));
    assert_eq!(
        transaction.input_commitments_as_bigint(),
        vec![BigUint::from(5678_u32), BigUint::from(7890_u32)]
    );
    assert_eq!(
        transaction.output_commitments_as_bigint().unwrap(),
        vec![BigUint::from(10001_u32), BigUint::from(10002_u32)]
    );
    assert_eq!(
        transaction.nullifiers_as_bigint().unwrap(),
        vec![BigUint::from(9999_u32), BigUint::from(9998_u32)]
    );
    assert_eq!(transaction.amount_as_bigint(), BigUint::from(1000000_u32));
    assert_eq!(transaction.public_amount_as_bigint(), BigUint::from(1000001_u32));
    assert_eq!(
        transaction.rollup_fee_amount_as_bigint(),
        Some(BigUint::from(1000002_u32))
    );
    assert_eq!(
        transaction.gas_relayer_fee_amount_as_bigint(),
        Some(BigUint::from(1000003_u32))
    );
    assert_eq!(
        transaction.random_auditing_public_key_as_bigint(),
        Some(BigUint::from(1000004_u32))
    );
    let transaction_bytes = transaction.encode_to_vec();
    let transaction_json = serde_json::to_string(&transaction).unwrap();
    assert_eq!(
        transaction,
        Transaction::decode(std::io::Cursor::new(&transaction_bytes)).unwrap()
    );
    assert_eq!(transaction, serde_json::from_str(&transaction_json).unwrap());
}
