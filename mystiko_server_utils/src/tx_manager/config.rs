use mehcode_config::{Config, Environment, File, ValueKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct TxManagerConfig {
    min_priority_fee_per_gas: u64,
    max_priority_fee_per_gas: u64,
    pub confirm_blocks: u32,
    pub max_confirm_count: u32,
    pub confirm_interval_secs: u64,
    pub gas_limit_reserve_percentage: u32,
    pub force_gas_price_chains: Vec<u64>,
    pub min_priority_fee_by_chain: HashMap<String, u64>,
}

impl TxManagerConfig {
    pub fn new(config_path: Option<&str>) -> anyhow::Result<Self> {
        let mut fee: HashMap<String, u64> = HashMap::new();
        fee.insert("137".to_string(), 30000000000);
        let mut s = Config::builder()
            .set_default("min_priority_fee_per_gas", "1000000000")?
            .set_default("max_priority_fee_per_gas", "50000000000")?
            .set_default("confirm_blocks", 2)?
            .set_default("max_confirm_count", 100)?
            .set_default("confirm_interval_secs", 10)?
            .set_default("gas_limit_reserve_percentage", 10)?
            .set_default("force_gas_price_chains", vec!["250", "4002"])?
            .set_default::<&str, ValueKind>("min_priority_fee_by_chain", fee.into())?;

        if let Some(path) = config_path {
            let run_config_path = format!("{}/tx_manager.json", path);
            if Path::exists(Path::new(&run_config_path)) {
                s = s.add_source(File::with_name(&run_config_path));
            }
        }

        let c = s
            .add_source(Environment::with_prefix("MYSTIKO_TX_MANAGER").separator("."))
            .build()?;

        let cfg: TxManagerConfig = c.try_deserialize()?;

        if cfg.max_priority_fee_per_gas < cfg.min_priority_fee_per_gas {
            return Err(anyhow::anyhow!(
                "max_priority_fee_per_gas must be greater than min_priority_fee_per_gas"
            ));
        }

        Ok(cfg)
    }

    pub fn get_min_priority_fee_per_gas(&self, chain_id: u64) -> u64 {
        match self.min_priority_fee_by_chain.get(&chain_id.to_string()) {
            Some(v) => *v,
            None => self.min_priority_fee_per_gas,
        }
    }
}
