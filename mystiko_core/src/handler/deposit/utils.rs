use crate::Deposit;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::v1::DepositStatus;
use mystiko_storage::Document;

pub(crate) fn format_deposit_log(deposit: &Document<Deposit>) -> String {
    if deposit.data.bridge_type == BridgeType::Loop as i32 {
        format!(
            "deposit(id={:?}, chain_id={}, asset_symbol={}, amount={}, rollup_fee_amount={}, status={:?})",
            deposit.id,
            deposit.data.chain_id,
            deposit.data.asset_symbol,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
        )
    } else {
        format!(
            "deposit(id={:?}, chain_id={}, dst_chain_id={}, bridge_type={:?}, asset_symbol={}, \
            amount={}, rollup_fee_amount={}, bridge_fee_amount={}, executor_fee_amount={}, status={:?})",
            deposit.id,
            deposit.data.chain_id,
            deposit.data.dst_chain_id,
            BridgeType::from_i32(deposit.data.bridge_type).unwrap_or_default(),
            deposit.data.asset_symbol,
            deposit.data.amount,
            deposit.data.rollup_fee_amount,
            deposit.data.bridge_fee_amount.unwrap_or_default(),
            deposit.data.executor_fee_amount.unwrap_or_default(),
            DepositStatus::from_i32(deposit.data.status).unwrap_or_default()
        )
    }
}
