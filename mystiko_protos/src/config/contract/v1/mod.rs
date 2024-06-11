use crate::common::v1::{BridgeType, ContractType};
use crate::config::v1::CircuitConfig;
pub use crate::gen::mystiko::config::contract::v1::*;

impl TryFrom<&mystiko_config::DepositContractConfig> for DepositContractConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::DepositContractConfig) -> anyhow::Result<Self> {
        Ok(DepositContractConfig::builder()
            .version(config.version())
            .name(config.name().to_string())
            .address(config.address().to_string())
            .start_block(config.start_block())
            .disabled(config.disabled())
            .min_amount(config.min_amount()?.to_string())
            .max_amount(config.max_amount()?.to_string())
            .pool_contract_config(Some(config.pool_contract().try_into()?))
            .bridge_type(Into::<BridgeType>::into(config.bridge_type()))
            .contract_type(Into::<ContractType>::into(config.contract_type()))
            .min_bridge_fee(Some(config.min_bridge_fee()?.to_string()))
            .min_executor_fee(Some(config.min_executor_fee()?.to_string()))
            .service_fee(Some(config.service_fee()))
            .service_fee_divider(Some(config.service_fee_divider()))
            .bridge_fee_asset_config(Some(config.bridge_fee_asset().try_into()?))
            .executor_fee_asset_config(Some(config.executor_fee_asset().try_into()?))
            .peer_chain_id(*config.peer_chain_id())
            .peer_contract_address(config.peer_contract_address().map(|s| s.to_string()))
            .build())
    }
}

impl TryFrom<&mystiko_config::PoolContractConfig> for PoolContractConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::PoolContractConfig) -> anyhow::Result<Self> {
        Ok(PoolContractConfig::builder()
            .version(config.version())
            .name(config.name().to_string())
            .address(config.address().to_string())
            .start_block(config.start_block())
            .pool_name(config.pool_name().to_string())
            .min_rollup_fee(config.min_rollup_fee()?.to_string())
            .contract_type(Into::<ContractType>::into(config.contract_type()))
            .bridge_type(Into::<BridgeType>::into(config.bridge_type()))
            .asset_config(Some(config.asset().try_into()?))
            .circuit_configs(
                config
                    .circuits()
                    .into_iter()
                    .map(|c| c.try_into())
                    .collect::<anyhow::Result<Vec<CircuitConfig>>>()?,
            )
            .circuits(config.circuits_names().clone())
            .build())
    }
}

impl TryFrom<&mystiko_config::ContractConfig> for ContractConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::ContractConfig) -> anyhow::Result<Self> {
        Ok(ContractConfig::builder()
            .version(config.version())
            .name(config.name().to_string())
            .address(config.address().to_string())
            .start_block(config.start_block())
            .disabled(config.disabled())
            .min_rollup_fee(config.min_rollup_fee()?.to_string())
            .asset_config(Some(config.asset().try_into()?))
            .bridge_type(Into::<BridgeType>::into(config.bridge_type()))
            .contract_type(Into::<ContractType>::into(config.contract_type()))
            .build())
    }
}
