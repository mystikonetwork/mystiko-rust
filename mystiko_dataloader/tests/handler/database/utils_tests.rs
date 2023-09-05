use mystiko_dataloader::handler::{dedup_commitments, dedup_nullifiers, merge_commitments};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;

#[test]
fn test_dedup_commitments() {
    let commitments: Vec<Commitment> = vec![
        Commitment::builder()
            .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
            .block_number(100u64)
            .status(CommitmentStatus::Queued as i32)
            .leaf_index(10u64)
            .rollup_fee(biguint_to_bytes(&BigUint::from(100u64)))
            .encrypted_note(vec![1u8, 2u8, 3u8])
            .queued_transaction_hash(vec![4u8, 5u8, 6u8])
            .src_chain_transaction_hash(vec![7u8, 8u8, 9u8])
            .build(),
        Commitment::builder()
            .commitment_hash(biguint_to_bytes(&BigUint::from(100000u64)))
            .block_number(200u64)
            .status(CommitmentStatus::Included as i32)
            .included_block_number(200u64)
            .included_transaction_hash(vec![7u8, 8u8, 9u8])
            .build(),
        Commitment::builder()
            .commitment_hash(biguint_to_bytes(&BigUint::from(200000u64)))
            .block_number(300u64)
            .status(CommitmentStatus::Unspecified as i32)
            .included_block_number(200u64)
            .included_transaction_hash(vec![7u8, 8u8, 9u8])
            .build(),
        Commitment::builder()
            .commitment_hash(biguint_to_bytes(&BigUint::from(300000u64)))
            .block_number(250u64)
            .status(CommitmentStatus::SrcSucceeded as i32)
            .src_chain_block_number(250u64)
            .src_chain_transaction_hash(vec![7u8, 8u8, 9u8])
            .build(),
    ];
    let deduped_commitments: Vec<Commitment> = dedup_commitments(commitments);
    assert_eq!(deduped_commitments.len(), 3);
    assert_eq!(deduped_commitments[0].status, CommitmentStatus::Included as i32);
    assert_eq!(deduped_commitments[0].included_block_number, Some(200u64));
    assert_eq!(deduped_commitments[0].block_number, 100u64);
    assert_eq!(
        deduped_commitments[0].commitment_hash,
        biguint_to_bytes(&BigUint::from(100000u64))
    );
    assert_eq!(
        deduped_commitments[0].included_transaction_hash,
        Some(vec![7u8, 8u8, 9u8])
    );
    assert_eq!(deduped_commitments[1].block_number, 250u64);
    assert_eq!(
        deduped_commitments[1].commitment_hash,
        biguint_to_bytes(&BigUint::from(300000u64))
    );
    assert_eq!(deduped_commitments[1].status, CommitmentStatus::SrcSucceeded as i32);
    assert_eq!(deduped_commitments[1].src_chain_block_number, Some(250u64));
    assert_eq!(
        deduped_commitments[1].src_chain_transaction_hash,
        Some(vec![7u8, 8u8, 9u8])
    );
    assert_eq!(deduped_commitments[2].block_number, 300u64);
    assert_eq!(
        deduped_commitments[2].commitment_hash,
        biguint_to_bytes(&BigUint::from(200000u64))
    );
    assert_eq!(deduped_commitments[2].status, CommitmentStatus::Unspecified as i32);
}

#[test]
fn test_dedup_nullifiers() {
    let deduped_nullifiers: Vec<Nullifier> = dedup_nullifiers(vec![
        Nullifier::builder()
            .nullifier(vec![1u8, 2u8, 3u8])
            .block_number(100u64)
            .transaction_hash(vec![1u8, 2u8, 3u8])
            .build(),
        Nullifier::builder()
            .nullifier(vec![1u8, 2u8, 3u8])
            .block_number(200u64)
            .transaction_hash(vec![11u8, 22u8, 33u8])
            .build(),
        Nullifier::builder()
            .nullifier(vec![4u8, 5u8, 6u8])
            .block_number(500u64)
            .transaction_hash(vec![1u8, 2u8, 3u8])
            .build(),
        Nullifier::builder()
            .nullifier(vec![7u8, 8u8, 9u8])
            .block_number(300u64)
            .transaction_hash(vec![1u8, 2u8, 3u8])
            .build(),
    ]);
    assert_eq!(deduped_nullifiers.len(), 3);
    assert_eq!(deduped_nullifiers[0].block_number, 200u64);
    assert_eq!(deduped_nullifiers[0].nullifier, vec![1u8, 2u8, 3u8]);
    assert_eq!(deduped_nullifiers[0].transaction_hash, vec![11u8, 22u8, 33u8]);
    assert_eq!(deduped_nullifiers[1].block_number, 300u64);
    assert_eq!(deduped_nullifiers[2].block_number, 500u64);
}

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
