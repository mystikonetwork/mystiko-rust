use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let commitment = Commitment::builder()
        .status(CommitmentStatus::Included)
        .commitment_hash(biguint_to_bytes(&BigUint::from(1234u32)))
        .rollup_fee(Some(biguint_to_bytes(&BigUint::from(5678u32))))
        .leaf_index(Some(10001))
        .encrypted_note(Some(decode_hex("0x6c0c").unwrap()))
        .creation_transaction_hash(Some(decode_hex("0xdead").unwrap()))
        .rollup_transaction_hash(Some(decode_hex("0xbeef").unwrap()))
        .build();
    assert_eq!(commitment.commitment_hash_as_biguint(), BigUint::from(1234u32));
    assert_eq!(commitment.rollup_fee_as_biguint(), Some(BigUint::from(5678u32)));
    assert_eq!(
        commitment.creation_transaction_hash_as_hex(),
        Some("0xdead".to_string())
    );
    assert_eq!(commitment.rollup_transaction_hash_as_hex(), Some("0xbeef".to_string()));
    let commitment_bytes = commitment.encode_to_vec();
    let commitment_json = serde_json::to_string(&commitment).unwrap();
    assert_eq!(
        commitment,
        Commitment::decode(std::io::Cursor::new(&commitment_bytes)).unwrap()
    );
    assert_eq!(commitment, serde_json::from_str(&commitment_json).unwrap());
}
