use crate::{Commitment, Spend, SpendsError};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::GasRelayer;
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_storage::Document;
use num_bigint::BigUint;
use num_traits::identities::Zero;
use std::collections::HashSet;
use std::ops::{Add, Div, Mul, Sub};

pub fn select_commitments(
    commitments: &[Document<Commitment>],
    target_amount: Option<BigUint>,
    max_num: usize,
) -> Vec<&Document<Commitment>> {
    let amounts = sort_amounts(commitments);
    let selected = if let Some(target_amount) = target_amount {
        let selected = select_amounts(&amounts, 0, target_amount.clone(), max_num, true);
        if selected.is_empty() {
            select_amounts(&amounts, 0, target_amount, max_num, false)
        } else {
            selected
        }
    } else {
        amounts.into_iter().take(max_num).collect::<Vec<_>>()
    };
    selected
        .into_iter()
        .filter_map(|(index, _)| commitments.get(index))
        .collect::<Vec<_>>()
}

pub fn calc_commitments_fixed_amounts(
    commitments: &[Document<Commitment>],
    threshold: &BigUint,
    max_num: usize,
) -> Vec<BigUint> {
    let amounts = sort_amounts(commitments)
        .into_iter()
        .map(|(_, amount)| amount)
        .collect::<Vec<_>>();
    let mut fixed_amounts = HashSet::new();
    for num in 1..=max_num {
        let fixed_amounts_group = calc_fixed_amounts(&amounts, threshold, 0, num);
        for amounts_group in fixed_amounts_group.into_iter() {
            fixed_amounts.insert(amounts_group.into_iter().sum::<BigUint>());
        }
    }
    let mut fixed_amounts = fixed_amounts.into_iter().collect::<Vec<_>>();
    fixed_amounts.sort();
    fixed_amounts
}

pub(crate) fn spend_circuit_type(num_inputs: u64, num_outputs: u64) -> Result<mystiko_types::CircuitType, SpendsError> {
    match (num_inputs, num_outputs) {
        (1, 0) => Ok(mystiko_types::CircuitType::Transaction1x0),
        (1, 1) => Ok(mystiko_types::CircuitType::Transaction1x1),
        (1, 2) => Ok(mystiko_types::CircuitType::Transaction1x2),
        (2, 0) => Ok(mystiko_types::CircuitType::Transaction2x0),
        (2, 1) => Ok(mystiko_types::CircuitType::Transaction2x1),
        (2, 2) => Ok(mystiko_types::CircuitType::Transaction2x2),
        _ => Err(SpendsError::UnsupportedSpendJoinSplitTypeError(num_inputs, num_outputs)),
    }
}

pub(crate) fn calc_min_gas_relayer_fee(amount: &BigUint, gas_relayer: &GasRelayer) -> Result<BigUint, SpendsError> {
    let service_fee = amount
        .mul(BigUint::from(gas_relayer.service_fee_of_ten_thousandth))
        .div(BigUint::from(10000_u32));
    Ok(service_fee.add(gas_relayer.minimum_gas_fee_as_biguint()?))
}

pub(crate) fn format_spend_log(spend: &Document<Spend>) -> String {
    let spend_type = SpendType::from_i32(spend.data.spend_type).unwrap_or_default();
    let bridge_type = BridgeType::from_i32(spend.data.bridge_type).unwrap_or_default();
    let num_inputs = spend.data.input_commitments.len();
    let num_outputs = spend
        .data
        .output_commitments
        .as_ref()
        .map(|commitments| commitments.len())
        .unwrap_or_default();
    let spend_status = SpendStatus::from_i32(spend.data.status).unwrap_or_default();
    let mut keys = vec![
        format!("id={:?}", spend.id),
        format!("type={:?}{}x{}", spend_type, num_inputs, num_outputs,),
        format!("bridge_type={:?}", bridge_type),
        format!("chain_id={}", spend.data.chain_id),
        format!("asset_symbol={}", spend.data.asset_symbol),
        format!("amount={}", spend.data.amount),
        format!("status={:?}", spend_status),
    ];
    if let Some(rollup_fee_amount) = spend.data.rollup_fee_amount {
        keys.push(format!("rollup_fee_amount={}", rollup_fee_amount));
    }
    if let Some(gas_relayer_fee_amount) = spend.data.gas_relayer_fee_amount {
        keys.push(format!("gas_relayer_fee_amount={}", gas_relayer_fee_amount));
    }
    keys.push(format!("recipient={:?}", spend.data.recipient));
    format!("Spend({})", keys.join(", "))
}

fn sort_amounts(commitments: &[Document<Commitment>]) -> Vec<(usize, BigUint)> {
    let mut amounts = commitments
        .iter()
        .enumerate()
        .filter_map(|(index, commitment)| commitment.data.amount.clone().map(|amount| (index, amount)))
        .collect::<Vec<_>>();
    amounts.sort_by_key(|(_, amount)| amount.clone());
    amounts.reverse();
    amounts
}

fn select_amounts(
    amounts: &[(usize, BigUint)],
    start_index: usize,
    target_amount: BigUint,
    max_num: usize,
    exact_match: bool,
) -> Vec<(usize, BigUint)> {
    if target_amount.gt(&BigUint::zero()) && max_num > 0 && start_index < amounts.len() {
        for (index, amount) in amounts[start_index..].iter() {
            if (exact_match && amount.eq(&target_amount)) || (!exact_match && amount.ge(&target_amount)) {
                return vec![(*index, amount.clone())];
            }
        }
        for (index, (commitment_index, amount)) in amounts[start_index..].iter().enumerate() {
            if amount.lt(&target_amount) {
                let target_amount = target_amount.clone().sub(amount);
                let mut selected = select_amounts(
                    amounts,
                    start_index + index + 1,
                    target_amount,
                    max_num - 1,
                    exact_match,
                );
                if !selected.is_empty() {
                    selected.insert(0, (*commitment_index, amount.clone()));
                    return selected;
                }
            }
        }
    }
    vec![]
}

fn calc_fixed_amounts(amounts: &[BigUint], threshold: &BigUint, start_index: usize, num: usize) -> Vec<Vec<BigUint>> {
    let mut fixed_amounts = vec![];
    if threshold.gt(&BigUint::zero()) && start_index < amounts.len() && num > 0 {
        for (index, amount) in amounts[start_index..].iter().enumerate() {
            if amount.le(threshold) && amount.gt(&BigUint::zero()) {
                let mut next_fixed_amounts =
                    calc_fixed_amounts(amounts, &threshold.sub(amount), start_index + index + 1, num - 1);
                if num > 1 {
                    for next_fixed_amount in next_fixed_amounts.iter_mut() {
                        next_fixed_amount.insert(0, amount.clone());
                    }
                    fixed_amounts.extend(next_fixed_amounts);
                } else {
                    fixed_amounts.push(vec![amount.clone()]);
                }
            }
        }
    }
    fixed_amounts
}
