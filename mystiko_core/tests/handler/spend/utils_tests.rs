use mystiko_core::{calc_commitments_fixed_amounts, select_commitments, Commitment};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_storage::Document;
use mystiko_utils::time::current_timestamp;
use num_bigint::BigUint;

#[test]
fn test_select_commitments() {
    let commitments = generate_commitments(vec![4, 2, 3, 5, 1]);

    assert!(select_commitments(&[], None, 1).is_empty());
    assert!(select_commitments(&[], Some(BigUint::from(10_u64)), 1).is_empty());
    assert!(select_commitments(&commitments, None, 0).is_empty());
    assert!(select_commitments(&commitments, Some(BigUint::from(0_u64)), 1).is_empty());

    let selected = select_commitments(&commitments, None, 2);
    assert_eq!(selected, vec![commitments.get(3).unwrap(), commitments.get(0).unwrap()]);

    let selected = select_commitments(&commitments, Some(BigUint::from(4_u64)), 2);
    assert_eq!(selected, vec![commitments.get(0).unwrap()]);

    let selected = select_commitments(&commitments, Some(BigUint::from(5_u64)), 2);
    assert_eq!(selected, vec![commitments.get(3).unwrap()]);

    let selected = select_commitments(&commitments, Some(BigUint::from(8_u64)), 2);
    assert_eq!(selected, vec![commitments.get(3).unwrap(), commitments.get(2).unwrap()]);

    let selected = select_commitments(&commitments, Some(BigUint::from(10_u64)), 2);
    assert!(selected.is_empty());

    let commitments = generate_commitments(vec![4, 8, 1, 10, 2]);
    let selected = select_commitments(&commitments, Some(BigUint::from(15_u64)), 2);
    assert_eq!(selected, vec![commitments.get(3).unwrap(), commitments.get(1).unwrap()]);

    let selected = select_commitments(&commitments, Some(BigUint::from(5_u64)), 1);
    assert_eq!(selected, vec![commitments.get(3).unwrap()]);

    let commitments = generate_commitments(vec![3, 4, 10]);
    let selected = select_commitments(&commitments, Some(BigUint::from(8_u64)), 2);
    assert_eq!(selected, vec![commitments.get(2).unwrap()]);
}

#[test]
fn test_calc_commitments_fixed_amount() {
    let commitments = generate_commitments(vec![4, 2, 3, 5, 1, 11]);
    assert!(calc_commitments_fixed_amounts(&[], &BigUint::from(10_u64), 1).is_empty());
    assert!(calc_commitments_fixed_amounts(&commitments, &BigUint::from(10_u64), 0).is_empty());
    let fixed_amounts = calc_commitments_fixed_amounts(&commitments, &BigUint::from(10_u64), 1);
    assert_eq!(
        fixed_amounts,
        vec![
            BigUint::from(1_u64),
            BigUint::from(2_u64),
            BigUint::from(3_u64),
            BigUint::from(4_u64),
            BigUint::from(5_u64)
        ]
    );
    let fixed_amounts = calc_commitments_fixed_amounts(&commitments, &BigUint::from(10_u64), 2);
    assert_eq!(
        fixed_amounts,
        vec![
            BigUint::from(1_u64),
            BigUint::from(2_u64),
            BigUint::from(3_u64),
            BigUint::from(4_u64),
            BigUint::from(5_u64),
            BigUint::from(6_u64),
            BigUint::from(7_u64),
            BigUint::from(8_u64),
            BigUint::from(9_u64),
        ]
    );
    let fixed_amounts = calc_commitments_fixed_amounts(&commitments, &BigUint::from(10_u64), 3);
    assert_eq!(
        fixed_amounts,
        vec![
            BigUint::from(1_u64),
            BigUint::from(2_u64),
            BigUint::from(3_u64),
            BigUint::from(4_u64),
            BigUint::from(5_u64),
            BigUint::from(6_u64),
            BigUint::from(7_u64),
            BigUint::from(8_u64),
            BigUint::from(9_u64),
            BigUint::from(10_u64),
        ]
    );
}

fn generate_commitments(amounts: Vec<u64>) -> Vec<Document<Commitment>> {
    amounts
        .into_iter()
        .enumerate()
        .map(|(index, amount)| Document {
            id: index.to_string(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            data: Commitment {
                chain_id: 1_u64,
                contract_address: "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string(),
                bridge_type: BridgeType::Loop as i32,
                commitment_hash: BigUint::from(index),
                asset_symbol: "ETH".to_string(),
                asset_decimals: 18,
                asset_address: None,
                status: CommitmentStatus::Included as i32,
                spent: false,
                block_number: 10000000 + index as u64,
                src_chain_block_number: None,
                included_block_number: Some(10000100 + index as u64),
                rollup_fee_amount: None,
                encrypted_note: None,
                leaf_index: None,
                amount: Some(BigUint::from(amount)),
                nullifier: None,
                shielded_address: None,
                queued_transaction_hash: None,
                included_transaction_hash: None,
                src_chain_transaction_hash: None,
            },
        })
        .collect()
}
