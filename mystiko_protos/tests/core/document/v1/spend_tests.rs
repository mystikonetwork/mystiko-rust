use mystiko_protos::core::document::v1::Spend;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let spend = Spend::builder().build();
    assert!(spend.rollup_fee_decimal_amount_as_biguint().unwrap().is_none());
    assert!(spend.gas_relayer_fee_decimal_amount_as_biguint().unwrap().is_none());
    assert!(spend.output_commitments_as_biguint().unwrap().is_none());
    assert!(spend.nullifiers_as_biguint().unwrap().is_none());
    assert!(spend.signature_public_key_hashes_as_biguint().unwrap().is_none());
    assert!(spend.encrypted_auditor_notes_as_biguint().unwrap().is_none());
    assert!(spend.random_auditing_public_key_as_biguint().unwrap().is_none());
    let spend = Spend::builder()
        .decimal_amount("10000".to_string())
        .rollup_fee_decimal_amount("20000".to_string())
        .gas_relayer_fee_decimal_amount("30000".to_string())
        .root_hash("1234".to_string())
        .input_commitments(vec!["5678".to_string(), "7890".to_string()])
        .output_commitments(vec!["10001".to_string(), "10002".to_string()])
        .nullifiers(vec!["9999".to_string(), "9998".to_string()])
        .signature_public_key_hashes(vec!["40000".to_string(), "50000".to_string()])
        .encrypted_auditor_notes(vec!["60000".to_string(), "70000".to_string()])
        .random_auditing_public_key(Some("1000004".to_string()))
        .build();
    assert_eq!(
        spend.nullifiers_as_biguint().unwrap().unwrap(),
        vec![BigUint::from(9999_u32), BigUint::from(9998_u32)]
    );
    assert_eq!(spend.decimal_amount_as_biguint().unwrap(), BigUint::from(10000_u64));
    assert_eq!(
        spend.rollup_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(20000_u64))
    );
    assert_eq!(
        spend.gas_relayer_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(30000_u64))
    );
    assert_eq!(spend.root_hash_as_biguint().unwrap(), BigUint::from(1234_u32));
    assert_eq!(
        spend.input_commitments_as_biguint().unwrap(),
        vec![BigUint::from(5678_u32), BigUint::from(7890_u32)]
    );
    assert_eq!(
        spend.output_commitments_as_biguint().unwrap().unwrap(),
        vec![BigUint::from(10001_u32), BigUint::from(10002_u32)]
    );
    assert_eq!(
        spend.gas_relayer_fee_decimal_amount_as_biguint().unwrap(),
        Some(BigUint::from(30000_u64))
    );
    assert_eq!(
        spend.signature_public_key_hashes_as_biguint().unwrap().unwrap(),
        vec![BigUint::from(40000_u64), BigUint::from(50000_u64)]
    );
    assert_eq!(
        spend.encrypted_auditor_notes_as_biguint().unwrap().unwrap(),
        vec![BigUint::from(60000_u64), BigUint::from(70000_u64)]
    );
    let transaction_bytes = spend.encode_to_vec();
    let transaction_json = serde_json::to_string(&spend).unwrap();
    assert_eq!(spend, Spend::try_from(&transaction_bytes).unwrap());
    assert_eq!(spend, serde_json::from_str(&transaction_json).unwrap());
}
