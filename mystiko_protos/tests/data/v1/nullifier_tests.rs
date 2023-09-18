use mystiko_protos::data::v1::Nullifier;
use mystiko_utils::convert::biguint_to_bytes;
use mystiko_utils::hex::decode_hex;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let nullifier = Nullifier::builder()
        .nullifier(biguint_to_bytes(&BigUint::from(1234_u32)))
        .transaction_hash(decode_hex("0xdead").unwrap())
        .build();
    assert_eq!(nullifier.nullifier_as_biguint(), BigUint::from(1234_u32));
    assert_eq!(nullifier.transaction_hash_as_hex(), "0xdead".to_string());
    let nullifier_bytes = nullifier.encode_to_vec();
    let nullifier_json = serde_json::to_string(&nullifier).unwrap();
    assert_eq!(nullifier, Nullifier::try_from(&nullifier_bytes).unwrap());
    assert_eq!(nullifier, serde_json::from_str(&nullifier_json).unwrap());
}
