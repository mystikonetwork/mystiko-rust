use mystiko_protos::core::document::v1::Commitment;
use mystiko_protos::core::v1::CommitmentStatus;
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let commitment = Commitment::builder()
        .status(CommitmentStatus::Included)
        .commitment_hash(biguint_to_bytes(&BigUint::from(1234_u32)))
        .rollup_fee_amount(Some(biguint_to_bytes(&BigUint::from(5678_u32))))
        .leaf_index(Some(biguint_to_bytes(&BigUint::from(10001_u32))))
        .amount(Some(biguint_to_bytes(&BigUint::from(1000000_u32))))
        .nullifier(Some(biguint_to_bytes(&BigUint::from(9999_u32))))
        .build();
    assert_eq!(commitment.commitment_hash_as_bigint(), BigUint::from(1234_u32));
    assert_eq!(commitment.rollup_fee_amount_as_bigint(), Some(BigUint::from(5678_u32)));
    assert_eq!(commitment.leaf_index_as_bigint(), Some(BigUint::from(10001_u32)));
    assert_eq!(commitment.amount_as_bigint(), Some(BigUint::from(1000000_u32)));
    assert_eq!(commitment.nullifier_as_bigint(), Some(BigUint::from(9999_u32)));
    let commitment_bytes = commitment.encode_to_vec();
    let commitment_json = serde_json::to_string(&commitment).unwrap();
    assert_eq!(
        commitment,
        Commitment::decode(std::io::Cursor::new(&commitment_bytes)).unwrap()
    );
    assert_eq!(commitment, serde_json::from_str(&commitment_json).unwrap());
}
