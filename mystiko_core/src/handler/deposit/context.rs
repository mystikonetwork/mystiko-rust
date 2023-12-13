use crate::{Deposit, DepositsError};
use mystiko_config::{ChainConfig, DepositContractConfig, MystikoConfig};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateDepositOptions, QuoteDepositOptions};
use mystiko_storage::Document;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub(crate) struct DepositContext {
    pub(crate) chain_config: ChainConfig,
    pub(crate) contract_config: DepositContractConfig,
    #[builder(default)]
    pub(crate) peer_contract_config: Option<DepositContractConfig>,
}

impl DepositContext {
    pub(crate) fn new(
        config: Arc<MystikoConfig>,
        chain_id: u64,
        contract_config: DepositContractConfig,
    ) -> Result<Self, DepositsError> {
        let chain_config = config
            .find_chain(chain_id)
            .ok_or(DepositsError::UnsupportedChainIdError(chain_id))?
            .clone();
        let peer_contract_config = if let (Some(peer_chain_id), Some(peer_contract_address)) =
            (contract_config.peer_chain_id(), contract_config.peer_contract_address())
        {
            config.find_deposit_contract_by_address(*peer_chain_id, peer_contract_address)
        } else {
            None
        };
        Ok(Self::builder()
            .chain_config(chain_config)
            .contract_config(contract_config)
            .peer_contract_config(peer_contract_config.cloned())
            .build())
    }

    pub(crate) fn from_quote_options(
        config: Arc<MystikoConfig>,
        options: &QuoteDepositOptions,
    ) -> Result<Self, DepositsError> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        Self::new(config, options.chain_id, contract_config)
    }

    pub(crate) fn from_create_options(
        config: Arc<MystikoConfig>,
        options: &CreateDepositOptions,
    ) -> Result<Self, DepositsError> {
        let contract_config = find_deposit_config(
            config.clone(),
            options.chain_id,
            &options.asset_symbol,
            options.dst_chain_id,
            options.bridge_type,
        )?;
        Self::new(config, options.chain_id, contract_config)
    }

    pub(crate) fn from_deposit(config: Arc<MystikoConfig>, deposit: &Document<Deposit>) -> Result<Self, DepositsError> {
        let contract_config = find_deposit_config(
            config.clone(),
            deposit.data.chain_id,
            &deposit.data.asset_symbol,
            Some(deposit.data.dst_chain_id),
            Some(deposit.data.bridge_type),
        )?;
        Self::new(config, deposit.data.chain_id, contract_config)
    }
}

fn find_deposit_config(
    config: Arc<MystikoConfig>,
    chain_id: u64,
    asset_symbol: &str,
    dst_chain_id: Option<u64>,
    bridge_type: Option<i32>,
) -> Result<DepositContractConfig, DepositsError> {
    let dst_chain_id = dst_chain_id.unwrap_or(chain_id);
    let bridge_type: mystiko_types::BridgeType = bridge_type
        .and_then(BridgeType::from_i32)
        .unwrap_or(BridgeType::Loop)
        .into();
    config
        .find_deposit_contract(chain_id, dst_chain_id, asset_symbol, &bridge_type)
        .cloned()
        .ok_or(DepositsError::NoDepositContractFoundError(
            chain_id,
            asset_symbol.to_string(),
            dst_chain_id,
            bridge_type,
        ))
}
