use mystiko_dataloader::handler::merge_commitments;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;

#[test]
fn test_merge_commitments() {
    let commitment1 = Commitment::builder()
        .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
        .block_number(100u64)
        .status(CommitmentStatus::Queued as i32)
        .leaf_index(10u64)
        .rollup_fee(biguint_to_bytes(&BigUint::from(100u64)))
        .encrypted_note(vec![1u8, 2u8, 3u8])
        .queued_transaction_hash(vec![4u8, 5u8, 6u8])
        .src_chain_transaction_hash(vec![7u8, 8u8, 9u8])
        .build();
    let commitment2 = Commitment::builder()
        .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
        .block_number(200u64)
        .status(CommitmentStatus::Included as i32)
        .included_block_number(200u64)
        .included_transaction_hash(vec![7u8, 8u8, 9u8])
        .build();
    let merged_commitment = merge_commitments(commitment1.clone(), commitment2.clone());
    assert_eq!(merged_commitment.status, CommitmentStatus::Included as i32);
    assert_eq!(merged_commitment.block_number, 100);
    assert_eq!(merged_commitment.included_block_number, Some(200));
    assert_eq!(merged_commitment.leaf_index, Some(10));
    assert_eq!(merged_commitment.rollup_fee_as_biguint(), Some(BigUint::from(100u64)));
    assert_eq!(merged_commitment.encrypted_note, Some(vec![1u8, 2u8, 3u8]));
    assert_eq!(merged_commitment.queued_transaction_hash, Some(vec![4u8, 5u8, 6u8]));
    assert_eq!(merged_commitment.included_transaction_hash, Some(vec![7u8, 8u8, 9u8]));
    assert_eq!(merged_commitment.src_chain_transaction_hash, Some(vec![7u8, 8u8, 9u8]));
    assert_eq!(merged_commitment, merge_commitments(commitment2, commitment1));
}
