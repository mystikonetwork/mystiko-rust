pub use crate::gen::mystiko::config::v1::*;

use crate::common::v1::{AssetType, CircuitType, ProviderType, TransactionType};
use crate::config::bridge::v1::BridgeConfig;
use crate::config::contract::v1::{DepositContractConfig, PoolContractConfig};
use crate::core::v1::{PackerChecksum, PackerCompression};
use std::collections::HashMap;

impl TryFrom<&mystiko_config::AssetConfig> for AssetConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::AssetConfig) -> anyhow::Result<Self> {
        Ok(AssetConfig::builder()
            .asset_type(Into::<AssetType>::into(config.asset_type()))
            .asset_symbol(config.asset_symbol().to_string())
            .asset_decimals(config.asset_decimals())
            .asset_address(Some(config.asset_address().to_string()))
            .recommended_amounts(
                config
                    .recommended_amounts()?
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>(),
            )
            .build())
    }
}

impl TryFrom<&mystiko_config::ChainConfig> for ChainConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::ChainConfig) -> anyhow::Result<Self> {
        let asset_configs = config
            .assets()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<AssetConfig>>>()?
            .into_iter()
            .map(|c| Ok((c.asset_address().to_string(), c)))
            .collect::<anyhow::Result<HashMap<String, AssetConfig>>>()?;
        let deposit_contract_configs = config
            .deposit_contracts()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<DepositContractConfig>>>()?
            .into_iter()
            .map(|c| Ok((c.address.to_string(), c)))
            .collect::<anyhow::Result<HashMap<String, DepositContractConfig>>>()?;
        let pool_contract_configs = config
            .pool_contracts()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<PoolContractConfig>>>()?
            .into_iter()
            .map(|c| Ok((c.address.to_string(), c)))
            .collect::<anyhow::Result<HashMap<String, PoolContractConfig>>>()?;
        let recommended_amounts = config
            .recommended_amounts()?
            .into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let provider_configs = config
            .providers()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<ProviderConfig>>>()?;
        Ok(ChainConfig::builder()
            .chain_id(config.chain_id())
            .name(config.name().to_string())
            .asset_symbol(config.asset_symbol().to_string())
            .asset_decimals(config.asset_decimals())
            .explorer_url(config.explorer_url().to_string())
            .explorer_api_url(config.explorer_api_url().to_string())
            .explorer_prefix(config.explorer_prefix().to_string())
            .provider_quorum_percentage(config.provider_quorum_percentage() as u32)
            .signer_endpoint(config.signer_endpoint().to_string())
            .event_delay_blocks(config.event_delay_blocks())
            .event_filter_size(config.event_filter_size())
            .sequencer_fetch_size(config.sequencer_fetch_size())
            .main_asset_config(Some(config.main_asset().try_into()?))
            .provider_type(Into::<ProviderType>::into(config.provider_type()))
            .transaction_type(Into::<TransactionType>::into(config.transaction_type()))
            .asset_configs(asset_configs)
            .deposit_contract_configs(deposit_contract_configs)
            .pool_contract_configs(pool_contract_configs)
            .recommended_amounts(recommended_amounts)
            .provider_configs(provider_configs)
            .granularities(config.granularities().to_vec())
            .build())
    }
}

impl TryFrom<&mystiko_config::CircuitConfig> for CircuitConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::CircuitConfig) -> anyhow::Result<Self> {
        Ok(CircuitConfig::builder()
            .name(config.name().to_string())
            .circuit_type(Into::<CircuitType>::into(config.circuit_type()))
            .is_default(config.is_default())
            .program_file(config.program_file().clone())
            .abi_file(config.abi_file().clone())
            .proving_key_file(config.proving_key_file().clone())
            .verifying_key_file(config.verifying_key_file().clone())
            .build())
    }
}

impl TryFrom<&mystiko_config::PackerConfig> for PackerConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::PackerConfig) -> anyhow::Result<Self> {
        Ok(PackerConfig::builder()
            .url(config.url().to_string())
            .client_timeout_ms(config.client_timeout_ms())
            .version(config.version())
            .checksum(Into::<PackerChecksum>::into(config.checksum()))
            .compression(Into::<PackerCompression>::into(config.compression()))
            .build())
    }
}

impl TryFrom<&mystiko_config::ProviderConfig> for ProviderConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::ProviderConfig) -> anyhow::Result<Self> {
        Ok(ProviderConfig::builder()
            .url(config.url().to_string())
            .timeout_ms(config.timeout_ms())
            .max_try_count(config.max_try_count())
            .quorum_weight(config.quorum_weight())
            .build())
    }
}

impl TryFrom<&mystiko_config::MystikoConfig> for MystikoConfig {
    type Error = anyhow::Error;

    fn try_from(config: &mystiko_config::MystikoConfig) -> anyhow::Result<Self> {
        let chain_configs = config
            .chains()
            .into_iter()
            .map(|c| Ok((c.chain_id(), c.try_into()?)))
            .collect::<anyhow::Result<HashMap<u64, ChainConfig>>>()?;
        let bridge_configs = config
            .bridges()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<BridgeConfig>>>()?
            .into_iter()
            .map(|c| (c.bridge_type, c))
            .collect::<HashMap<i32, BridgeConfig>>();
        let country_blacklist = config
            .country_blacklist()
            .into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let circuit_configs = config
            .circuits()
            .into_iter()
            .map(|c| c.try_into())
            .collect::<anyhow::Result<Vec<CircuitConfig>>>()?;
        let packer_config = config.packer().map(|c| c.try_into()).transpose()?;
        let sequencer_config = config.sequencer().map(|c| c.into());
        Ok(MystikoConfig::builder()
            .version(config.version().to_string())
            .chain_configs(chain_configs)
            .bridge_configs(bridge_configs)
            .git_revision(config.git_revision().map(|s| s.to_string()))
            .sequencer_config(sequencer_config)
            .packer_config(packer_config)
            .country_blacklist(country_blacklist)
            .circuit_configs(circuit_configs)
            .build())
    }
}
