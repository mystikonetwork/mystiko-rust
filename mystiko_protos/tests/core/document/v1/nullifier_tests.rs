use mystiko_protos::core::document::v1::Nullifier;
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use prost::Message;

#[test]
fn test_wrappers() {
    let nullifier = Nullifier::builder()
        .nullifier(biguint_to_bytes(&BigUint::from(9999_u32)))
        .build();
    assert_eq!(nullifier.nullifier_as_biguint(), BigUint::from(9999_u32));
    let nullifier_bytes = nullifier.encode_to_vec();
    let nullifier_json = serde_json::to_string(&nullifier).unwrap();
    assert_eq!(
        nullifier,
        Nullifier::decode(std::io::Cursor::new(&nullifier_bytes)).unwrap()
    );
    assert_eq!(nullifier, serde_json::from_str(&nullifier_json).unwrap());
}
