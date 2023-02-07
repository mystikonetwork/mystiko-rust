use std::collections::HashMap;
use crate::common::BridgeType;
use crate::raw::contract::base::RawContractConfigTrait;
use crate::raw::contract::deposit::RawDepositContractConfig;
use crate::wrapper::asset::AssetConfig;
use crate::wrapper::contract::base::ContractConfig;
use crate::wrapper::contract::pool::PoolContractConfig;
use crate::wrapper::mystiko::MystikoConfig;

#[derive(Clone)]
pub struct AuxData {
    deposit_contract_getter: fn(&MystikoConfig, u32, String) -> Option<DepositContractConfig>,
    pool_contract_configs: HashMap<String, PoolContractConfig>,
    main_asset_config: AssetConfig,
    asset_configs: HashMap<String, AssetConfig>,
}

impl AuxData {
    pub fn new(
        deposit_contract_getter: fn(&MystikoConfig, u32, String) -> Option<DepositContractConfig>,
        pool_contract_configs: HashMap<String, PoolContractConfig>,
        main_asset_config: AssetConfig,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Self {
        Self { deposit_contract_getter, pool_contract_configs, main_asset_config, asset_configs }
    }

    fn pool_contract_getter(&self, address: &str) -> Option<&PoolContractConfig> {
        self.pool_contract_configs.get(address)
    }
}

#[derive(Clone)]
pub struct DepositContractConfig {
    base: ContractConfig<RawDepositContractConfig, AuxData>,
    bridge_fee_asset_config: Option<AssetConfig>,
    executor_fee_asset_config: Option<AssetConfig>,
}

impl DepositContractConfig {
    pub fn new(data: RawDepositContractConfig, aux_data: Option<AuxData>) -> Self {
        let contract_config = ContractConfig::new(data, aux_data);
        let bridge_fee_asset_config = DepositContractConfig::init_bridge_fee_asset_config(
            &contract_config,
            contract_config.base.aux_data_not_empty().asset_configs,
        );
        let executor_fee_asset_config = DepositContractConfig::init_executor_fee_asset_config(
            &contract_config,
            contract_config.base.aux_data_not_empty().asset_configs,
        );
        Self {
            base: contract_config,
            bridge_fee_asset_config,
            executor_fee_asset_config,
        }
    }

    pub fn bridge_type(&self) -> BridgeType {
        self.base.base.data.bridge_type.clone()
    }

    pub fn pool_contract(&self) -> PoolContractConfig {
        let aux_data = self.base.base.aux_data_not_empty();
        let pool_contract_config = aux_data.pool_contract_getter(
            &self.base.base.data.pool_address
        );
        pool_contract_config.expect(
            format!(
                "no poolContract definition found for deposit contract={:?}", self.base.base.data.address()
            ).as_str()
        ).clone()
    }

    pub fn disabled(&self) -> bool {
        self.base.base.data.disabled
    }

    pub fn peer_chain_id(&self) -> &Option<u32> {
        &self.base.base.data.peer_chain_id
    }

    pub fn peer_contract_address(&self) -> &Option<String> {
        &self.base.base.data.peer_contract_address
    }

    pub fn asset_symbol(&self) -> String {
        self.pool_contract().asset_symbol().to_string()
    }

    fn init_bridge_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        None
    }

    fn init_executor_fee_asset_config(
        base: &ContractConfig<RawDepositContractConfig, AuxData>,
        asset_configs: HashMap<String, AssetConfig>,
    ) -> Option<AssetConfig> {
        None
    }
}