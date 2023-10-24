use mystiko_protos::core::document::v1::Spend;
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let spend = Spend::builder()
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
    assert_eq!(spend.root_hash_as_biguint(), BigUint::from(1234_u32));
    assert_eq!(
        spend.input_commitments_as_biguint(),
        vec![BigUint::from(5678_u32), BigUint::from(7890_u32)]
    );
    assert_eq!(
        spend.output_commitments_as_biguint().unwrap(),
        vec![BigUint::from(10001_u32), BigUint::from(10002_u32)]
    );
    assert_eq!(
        spend.nullifiers_as_biguint().unwrap(),
        vec![BigUint::from(9999_u32), BigUint::from(9998_u32)]
    );
    assert_eq!(spend.amount_as_biguint(), BigUint::from(1000000_u32));
    assert_eq!(spend.public_amount_as_biguint(), BigUint::from(1000001_u32));
    assert_eq!(spend.rollup_fee_amount_as_biguint(), Some(BigUint::from(1000002_u32)));
    assert_eq!(
        spend.gas_relayer_fee_amount_as_biguint(),
        Some(BigUint::from(1000003_u32))
    );
    assert_eq!(
        spend.random_auditing_public_key_as_biguint(),
        Some(BigUint::from(1000004_u32))
    );
    let transaction_bytes = spend.encode_to_vec();
    let transaction_json = serde_json::to_string(&spend).unwrap();
    assert_eq!(spend, Spend::try_from(&transaction_bytes).unwrap());
    assert_eq!(spend, serde_json::from_str(&transaction_json).unwrap());
}
