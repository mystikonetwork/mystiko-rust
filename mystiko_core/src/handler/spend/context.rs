use crate::{Spend, SpendsError};
use mystiko_config::{MystikoConfig, PoolContractConfig};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateSpendOptions, QuoteSpendOptions};
use mystiko_storage::Document;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct SpendContext {
    pub(crate) chain_id: u64,
    pub(crate) contract_config: PoolContractConfig,
}

impl SpendContext {
    pub(crate) fn from_quote_options(
        config: Arc<MystikoConfig>,
        options: &QuoteSpendOptions,
    ) -> Result<Self, SpendsError> {
        let contract_config = find_pool_contract(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.bridge_type,
            options.version,
        )?;
        Ok(Self::builder()
            .chain_id(options.chain_id)
            .contract_config(contract_config)
            .build())
    }

    pub(crate) fn from_create_options(
        config: Arc<MystikoConfig>,
        options: &CreateSpendOptions,
    ) -> Result<Self, SpendsError> {
        let contract_config = find_pool_contract(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.bridge_type,
            options.version,
        )?;
        Ok(Self::builder()
            .chain_id(options.chain_id)
            .contract_config(contract_config)
            .build())
    }

    pub(crate) fn from_spend(config: Arc<MystikoConfig>, spend: &Document<Spend>) -> Result<Self, SpendsError> {
        let contract_config = config
            .find_pool_contract_by_address(spend.data.chain_id, &spend.data.contract_address)
            .cloned()
            .ok_or(SpendsError::NoPoolContractAddressFoundError(
                spend.data.chain_id,
                spend.data.contract_address.clone(),
            ))?;
        Ok(Self::builder()
            .chain_id(spend.data.chain_id)
            .contract_config(contract_config)
            .build())
    }
}

fn find_pool_contract(
    config: Arc<MystikoConfig>,
    chain_id: u64,
    asset_symbol: &str,
    bridge_type: Option<i32>,
    version: Option<u32>,
) -> Result<PoolContractConfig, SpendsError> {
    let bridge_type = bridge_type.unwrap_or(BridgeType::Loop as i32);
    let bridge_type = BridgeType::from_i32(bridge_type).unwrap_or_default();
    let bridge_type: mystiko_types::BridgeType = bridge_type.into();
    if let Some(version) = version {
        config
            .find_pool_contract(chain_id, asset_symbol, &bridge_type, version)
            .cloned()
            .ok_or(SpendsError::NoPoolContractFoundError(
                chain_id,
                asset_symbol.to_string(),
                bridge_type,
                Some(version),
            ))
    } else {
        let mut pool_contracts = config.find_pool_contracts(chain_id, asset_symbol, &bridge_type);
        pool_contracts.sort_by_key(|contract| contract.version());
        pool_contracts
            .pop()
            .cloned()
            .ok_or(SpendsError::NoPoolContractFoundError(
                chain_id,
                asset_symbol.to_string(),
                bridge_type,
                None,
            ))
    }
}
