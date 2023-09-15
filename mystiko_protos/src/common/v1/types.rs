use crate::common::v1::{AssetType, BridgeType, CircuitType, ContractType, ProviderType};

impl From<&mystiko_types::BridgeType> for BridgeType {
    fn from(value: &mystiko_types::BridgeType) -> Self {
        match value {
            mystiko_types::BridgeType::Loop => BridgeType::Loop,
            mystiko_types::BridgeType::Poly => BridgeType::Poly,
            mystiko_types::BridgeType::Tbridge => BridgeType::Tbridge,
            mystiko_types::BridgeType::Celer => BridgeType::Celer,
            mystiko_types::BridgeType::LayerZero => BridgeType::LayerZero,
            mystiko_types::BridgeType::Axelar => BridgeType::Axelar,
        }
    }
}

impl From<&mystiko_types::AssetType> for AssetType {
    fn from(value: &mystiko_types::AssetType) -> Self {
        match value {
            mystiko_types::AssetType::Erc20 => AssetType::Erc20,
            mystiko_types::AssetType::Main => AssetType::Main,
        }
    }
}

impl From<&mystiko_types::CircuitType> for CircuitType {
    fn from(value: &mystiko_types::CircuitType) -> Self {
        match value {
            mystiko_types::CircuitType::Rollup1 => CircuitType::Rollup1,
            mystiko_types::CircuitType::Rollup2 => CircuitType::Rollup2,
            mystiko_types::CircuitType::Rollup4 => CircuitType::Rollup4,
            mystiko_types::CircuitType::Rollup8 => CircuitType::Rollup8,
            mystiko_types::CircuitType::Rollup16 => CircuitType::Rollup16,
            mystiko_types::CircuitType::Transaction1x0 => CircuitType::Transaction1x0,
            mystiko_types::CircuitType::Transaction1x1 => CircuitType::Transaction1x1,
            mystiko_types::CircuitType::Transaction1x2 => CircuitType::Transaction1x2,
            mystiko_types::CircuitType::Transaction2x0 => CircuitType::Transaction2x0,
            mystiko_types::CircuitType::Transaction2x1 => CircuitType::Transaction2x1,
            mystiko_types::CircuitType::Transaction2x2 => CircuitType::Transaction2x2,
        }
    }
}

impl From<&mystiko_types::ContractType> for ContractType {
    fn from(value: &mystiko_types::ContractType) -> Self {
        match value {
            mystiko_types::ContractType::Deposit => ContractType::Deposit,
            mystiko_types::ContractType::Pool => ContractType::Pool,
        }
    }
}

impl From<&mystiko_types::ProviderType> for ProviderType {
    fn from(value: &mystiko_types::ProviderType) -> Self {
        match value {
            mystiko_types::ProviderType::Failover => ProviderType::Failover,
            mystiko_types::ProviderType::Quorum => ProviderType::Quorum,
        }
    }
}

impl From<ContractType> for mystiko_types::ContractType {
    fn from(value: ContractType) -> Self {
        match value {
            ContractType::Unspecified => mystiko_types::ContractType::Deposit,
            ContractType::Deposit => mystiko_types::ContractType::Deposit,
            ContractType::Pool => mystiko_types::ContractType::Pool,
        }
    }
}
