use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::common::{BridgeType, ContractType};
use crate::raw::contract::base::RawContractConfig;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct RawDepositContractConfig {
    contract: RawContractConfig,
    contract_type: ContractType,
    bridge_type: BridgeType,
    pool_address: String,
    disabled: bool,
    peer_chain_id: u32,
    peer_contract_address: String,
    min_amount: String,
    max_amount: String,
    min_bridge_fee: String,
    min_executor_fee: String,
    bridge_fee_asset_address: String,
    executor_fee_asset_address: String,
    service_fee: u32,
    service_fee_divider: u32,
}