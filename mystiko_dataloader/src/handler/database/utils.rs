use mystiko_protos::data::v1::{Commitment, Nullifier};
use mystiko_utils::convert::bytes_to_biguint;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn dedup_commitments(commitments: Vec<Commitment>) -> Vec<Commitment> {
    let mut commitments_map = HashMap::new();
    for commitment in commitments.into_iter() {
        let commitment_hash = bytes_to_biguint(&commitment.commitment_hash);
        let commitment = if let Some(existing_commitment) = commitments_map.remove(&commitment_hash) {
            merge_commitments(existing_commitment, commitment)
        } else {
            commitment
        };
        commitments_map.insert(commitment_hash, commitment);
    }
    let mut commitments = commitments_map.into_values().collect::<Vec<_>>();
    commitments.sort_by(|c1, c2| {
        if c1.block_number == c2.block_number {
            c1.leaf_index.cmp(&c2.leaf_index)
        } else {
            c1.block_number.cmp(&c2.block_number)
        }
    });
    commitments
}

pub fn dedup_nullifiers(nullifiers: Vec<Nullifier>) -> Vec<Nullifier> {
    let nullifiers = nullifiers
        .into_iter()
        .map(|n| (n.nullifier_as_biguint(), n))
        .collect::<HashMap<_, _>>();
    let mut nullifiers = nullifiers.into_values().collect::<Vec<_>>();
    nullifiers.sort_by_key(|n| n.block_number);
    nullifiers
}

pub fn merge_commitments(commitment1: Commitment, commitment2: Commitment) -> Commitment {
    let (first, last) = match commitment1.block_number.cmp(&commitment2.block_number) {
        Ordering::Less => (commitment1, commitment2),
        Ordering::Equal => match commitment1.status.cmp(&commitment2.status) {
            Ordering::Less => (commitment1, commitment2),
            _ => (commitment2, commitment1),
        },
        Ordering::Greater => (commitment2, commitment1),
    };
    Commitment::builder()
        .commitment_hash(first.commitment_hash)
        .status(last.status)
        .block_number(first.block_number)
        .leaf_index(first.leaf_index.or(last.leaf_index))
        .encrypted_note(first.encrypted_note.or(last.encrypted_note))
        .rollup_fee(first.rollup_fee.or(last.rollup_fee))
        .included_block_number(first.included_block_number.or(last.included_block_number))
        .src_chain_block_number(first.src_chain_block_number.or(last.src_chain_block_number))
        .queued_transaction_hash(first.queued_transaction_hash.or(last.queued_transaction_hash))
        .included_transaction_hash(first.included_transaction_hash.or(last.included_transaction_hash))
        .src_chain_transaction_hash(first.src_chain_transaction_hash.or(last.src_chain_transaction_hash))
        .build()
}
