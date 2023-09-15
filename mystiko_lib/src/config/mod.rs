use crate::runtime;
use anyhow::Result;
use mystiko_protos::common::v1::{BridgeType, CircuitType};
use mystiko_protos::config::bridge::v1::BridgeConfig;
use mystiko_protos::config::contract::v1::{ContractConfig, DepositContractConfig, PoolContractConfig};
use mystiko_protos::config::v1::{ChainConfig, CircuitConfig, MystikoConfig};

pub fn get() -> Result<MystikoConfig> {
    runtime().block_on(internal::get())
}

pub fn find_default_circuit(circuit_type: CircuitType) -> Result<Option<CircuitConfig>> {
    runtime().block_on(internal::find_default_circuit(circuit_type))
}

pub fn find_circuit(circuit_name: String) -> Result<Option<CircuitConfig>> {
    runtime().block_on(internal::find_circuit(circuit_name))
}

pub fn find_chain(chain_id: u64) -> Result<Option<ChainConfig>> {
    runtime().block_on(internal::find_chain(chain_id))
}

pub fn find_peer_chains(chain_id: u64) -> Result<Vec<ChainConfig>> {
    runtime().block_on(internal::find_peer_chains(chain_id))
}

pub fn find_asset_symbols(chain_id: u64, peer_chain_id: u64) -> Result<Vec<String>> {
    runtime().block_on(internal::find_asset_symbols(chain_id, peer_chain_id))
}

pub fn find_bridges(chain_id: u64, peer_chain_id: u64, asset_symbol: String) -> Result<Vec<BridgeType>> {
    runtime().block_on(internal::find_bridges(chain_id, peer_chain_id, asset_symbol))
}

pub fn find_bridge(bridge_type: BridgeType) -> Result<Option<BridgeConfig>> {
    runtime().block_on(internal::find_bridge(bridge_type))
}

pub fn find_deposit_contract(
    chain_id: u64,
    peer_chain_id: u64,
    asset_symbol: String,
    bridge_type: BridgeType,
) -> Result<Option<DepositContractConfig>> {
    runtime().block_on(internal::find_deposit_contract(
        chain_id,
        peer_chain_id,
        asset_symbol,
        bridge_type,
    ))
}

pub fn find_deposit_contract_by_address(chain_id: u64, address: String) -> Result<DepositContractConfig> {
    todo!()
}

pub fn find_pool_contract(
    chain_id: u64,
    asset_symbol: String,
    bridge_type: BridgeType,
    version: u32,
) -> Result<Option<PoolContractConfig>> {
    runtime().block_on(internal::find_pool_contract(
        chain_id,
        asset_symbol,
        bridge_type,
        version,
    ))
}

pub fn find_pool_contracts(
    chain_id: u64,
    asset_symbol: String,
    bridge_type: BridgeType,
) -> Result<Vec<PoolContractConfig>> {
    runtime().block_on(internal::find_pool_contracts(chain_id, asset_symbol, bridge_type))
}

pub fn find_pool_contract_by_address(chain_id: u64, address: String) -> Result<Option<PoolContractConfig>> {
    runtime().block_on(internal::find_pool_contract_by_address(chain_id, address))
}

pub fn find_contract_by_address(chain_id: u64, address: String) -> Result<Option<ContractConfig>> {
    runtime().block_on(internal::find_contract_by_address(chain_id, address))
}

pub fn get_transaction_url(chain_id: u64, tx_hash: String) -> Result<Option<String>> {
    runtime().block_on(internal::get_transaction_url(chain_id, tx_hash))
}

mod internal {
    use super::*;
    use crate::instance;

    pub(crate) async fn get() -> Result<MystikoConfig> {
        let mystiko_guard = instance().read().await;
        mystiko_guard.get()?.config.to_proto()
    }

    pub(crate) async fn find_default_circuit(circuit_type: CircuitType) -> Result<Option<CircuitConfig>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard
            .get()?
            .config
            .find_default_circuit(&circuit_type.into())
            .map(|c| c.to_proto()))
    }

    pub(crate) async fn find_circuit(circuit_name: String) -> Result<Option<CircuitConfig>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard
            .get()?
            .config
            .find_circuit(&circuit_name)
            .map(|c| c.to_proto()))
    }

    pub(crate) async fn find_chain(chain_id: u64) -> Result<Option<ChainConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard.get()?.config.find_chain(chain_id).map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn find_peer_chains(chain_id: u64) -> Result<Vec<ChainConfig>> {
        let mystiko_guard = instance().read().await;
        mystiko_guard
            .get()?
            .config
            .find_peer_chains(chain_id)
            .into_iter()
            .map(|c| c.to_proto())
            .collect()
    }

    pub(crate) async fn find_asset_symbols(chain_id: u64, peer_chain_id: u64) -> Result<Vec<String>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard
            .get()?
            .config
            .find_asset_symbols(chain_id, peer_chain_id)
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }

    pub(crate) async fn find_bridges(
        chain_id: u64,
        peer_chain_id: u64,
        asset_symbol: String,
    ) -> Result<Vec<BridgeType>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard
            .get()?
            .config
            .find_bridges(chain_id, peer_chain_id, &asset_symbol)
            .into_iter()
            .map(|b| b.into())
            .collect::<Vec<BridgeType>>())
    }

    pub(crate) async fn find_bridge(bridge_type: BridgeType) -> Result<Option<BridgeConfig>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard
            .get()?
            .config
            .find_bridge(&bridge_type.into())
            .map(|c| c.to_proto()))
    }

    pub(crate) async fn find_deposit_contract(
        chain_id: u64,
        peer_chain_id: u64,
        asset_symbol: String,
        bridge_type: BridgeType,
    ) -> Result<Option<DepositContractConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard
            .get()?
            .config
            .find_deposit_contract(chain_id, peer_chain_id, &asset_symbol, &bridge_type.into())
            .map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn find_deposit_contract_by_address(
        chain_id: u64,
        address: String,
    ) -> Result<Option<DepositContractConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard
            .get()?
            .config
            .find_deposit_contract_by_address(chain_id, &address)
            .map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn find_pool_contract(
        chain_id: u64,
        asset_symbol: String,
        bridge_type: BridgeType,
        version: u32,
    ) -> Result<Option<PoolContractConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard
            .get()?
            .config
            .find_pool_contract(chain_id, &asset_symbol, &bridge_type.into(), version)
            .map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn find_pool_contracts(
        chain_id: u64,
        asset_symbol: String,
        bridge_type: BridgeType,
    ) -> Result<Vec<PoolContractConfig>> {
        let mystiko_guard = instance().read().await;
        mystiko_guard
            .get()?
            .config
            .find_pool_contracts(chain_id, &asset_symbol, &bridge_type.into())
            .into_iter()
            .map(|c| c.to_proto())
            .collect()
    }

    pub(crate) async fn find_pool_contract_by_address(
        chain_id: u64,
        address: String,
    ) -> Result<Option<PoolContractConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard
            .get()?
            .config
            .find_pool_contract_by_address(chain_id, &address)
            .map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn find_contract_by_address(chain_id: u64, address: String) -> Result<Option<ContractConfig>> {
        let mystiko_guard = instance().read().await;
        let option = mystiko_guard
            .get()?
            .config
            .find_contract_by_address(chain_id, &address)
            .map(|c| c.to_proto());
        Ok(option.transpose()?)
    }

    pub(crate) async fn get_transaction_url(chain_id: u64, tx_hash: String) -> Result<Option<String>> {
        let mystiko_guard = instance().read().await;
        Ok(mystiko_guard.get()?.config.transaction_url(chain_id, &tx_hash))
    }
}
