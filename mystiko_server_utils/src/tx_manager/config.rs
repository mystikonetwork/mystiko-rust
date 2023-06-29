use ethers_core::types::{U256, U64};
use mehcode_config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct TxManagerConfig {
    pub max_gas_price: U256,
    pub min_priority_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
    pub confirm_blocks: U64,
    pub max_confirm_count: u32,
    pub confirm_interval_secs: u64,
    pub gas_limit_reserve_percentage: u32,
    pub force_gas_price_chains: Vec<U64>,
}

impl TxManagerConfig {
    pub fn new(run_mod: &str, config_path: Option<&str>) -> anyhow::Result<Self> {
        let mut s = Config::builder()
            .set_default("min_priority_fee_per_gas", "0x3b9aca00")?
            .set_default("max_priority_fee_per_gas", "0xba43b7400")?
            .set_default("confirm_blocks", 2)?
            .set_default("max_confirm_count", 100)?
            .set_default("confirm_interval_secs", 10)?
            .set_default("gas_limit_reserve_percentage", 10)?
            .set_default("force_gas_price_chains", vec!["0xfa", "0xfa2"])?;

        if run_mod == "testnet" {
            s = s.set_default("max_gas_price", "0x2e90edd000")?;
        } else {
            s = s.set_default("max_gas_price", "0x174876e800")?;
        }

        if config_path.is_some() {
            let run_config_path = format!("{}/tx_manager.json", config_path.unwrap());
            if Path::exists(Path::new(&run_config_path)) {
                s = s.add_source(File::with_name(&run_config_path));
            }
        }

        let c = s
            .add_source(Environment::with_prefix("MYSTIKO_TX_MANAGER").separator("."))
            .build()?;

        let cfg: TxManagerConfig = c.try_deserialize()?;

        if cfg.max_gas_price < cfg.max_priority_fee_per_gas {
            return Err(anyhow::anyhow!(
                "max_gas_price must be greater than min_priority_fee_per_gas"
            ));
        }

        if cfg.max_priority_fee_per_gas < cfg.min_priority_fee_per_gas {
            return Err(anyhow::anyhow!(
                "max_priority_fee_per_gas must be greater than min_priority_fee_per_gas"
            ));
        }

        Ok(cfg)
    }
}
