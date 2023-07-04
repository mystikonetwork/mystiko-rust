use crate::common::env::{load_roller_config_path, load_roller_run_mod};
use crate::common::error::{Result, RollerError};
use mehcode_config::{Config, Environment, File};
use mystiko_config::wrapper::mystiko::{MystikoConfig, RemoteOptions};
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use tracing::{error, info};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct RollerConfig {
    pub log_level: String,
    pub core: CoreConfig,
    pub chain: ChainConfig,
    pub pull: PullConfig,
    pub rollup: RollupConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub data_source_order: String,
}

impl ChainConfig {
    pub fn get_data_source_order(&self) -> Vec<ChainDataSource> {
        let mut data_sources = Vec::new();
        let data_source_order = self.data_source_order.split(',');
        for source in data_source_order {
            let source = source.trim();
            match source {
                "explorer" => data_sources.push(ChainDataSource::Explorer),
                "indexer" => data_sources.push(ChainDataSource::Indexer),
                "provider" => data_sources.push(ChainDataSource::Provider),
                _ => {
                    panic!("invalid data source: {}", source);
                }
            }
        }
        data_sources
    }

    pub fn is_data_source_enable(&self, source: ChainDataSource) -> bool {
        let data_sources = self.get_data_source_order();
        data_sources.iter().any(|s| *s == source)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct CoreConfig {
    pub is_staging: bool,
    pub remote_base_url: Option<String>,
    pub git_revision: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct PullConfig {
    pub check_interval_secs: u64,
    pub max_empty_queue_count: u64,
    pub batch_block_from_indexer: u32,
    pub batch_block_from_provider: u32,
    pub batch_block_from_explorer: u32,
    pub explorer_request_timeout_secs: u64,
}

impl PullConfig {
    pub fn batch_block(&self, data_source: ChainDataSource) -> usize {
        match data_source {
            ChainDataSource::Indexer => self.batch_block_from_indexer as usize,
            ChainDataSource::Explorer => self.batch_block_from_explorer as usize,
            ChainDataSource::Provider => self.batch_block_from_provider as usize,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[allow(unused)]
pub struct RollupConfig {
    pub merkle_tree_height: u32,
    pub force_rollup_block_count: u64,
    pub rollup1_gas_cost: u32,
    pub rollup2_gas_cost: u32,
    pub rollup4_gas_cost: u32,
    pub rollup8_gas_cost: u32,
    pub rollup16_gas_cost: u32,
}

impl RollupConfig {
    pub fn get_rollup_cost(&self, rollup_size: usize) -> u32 {
        match rollup_size {
            1 => self.rollup1_gas_cost,
            2 => self.rollup2_gas_cost,
            4 => self.rollup4_gas_cost,
            8 => self.rollup8_gas_cost,
            16 => self.rollup16_gas_cost,
            _ => panic!("unsupported rollup size {}", rollup_size),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChainDataSource {
    Indexer,
    Provider,
    Explorer,
}

impl FromStr for ChainDataSource {
    type Err = RollerError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "indexer" => Ok(ChainDataSource::Indexer),
            "explorer" => Ok(ChainDataSource::Explorer),
            "provider" => Ok(ChainDataSource::Provider),
            _ => Err(RollerError::LoadConfigError("chain data source invalid".to_string())),
        }
    }
}

impl RollerConfig {
    pub fn new() -> Result<Self> {
        let run_mod = load_roller_run_mod();
        let cfg_path = load_roller_config_path();
        let mut s = Config::builder()
            .add_source(File::with_name(&format!("{}/default", cfg_path)))
            .add_source(File::with_name(&format!("{}/{}", cfg_path, run_mod)));

        let roller_cfg = format!("{}/roller.json", cfg_path);
        if Path::exists(Path::new(&roller_cfg)) {
            s = s.add_source(File::with_name(&roller_cfg));
        }

        let cfg = s
            .add_source(Environment::with_prefix("MYSTIKO_ROLLER").separator("."))
            .build()?;
        Ok(cfg.try_deserialize()?)
    }
}

pub fn create_roller_config() -> RollerConfig {
    RollerConfig::new().unwrap_or_else(|_| panic!("load roller config failed"))
}

pub async fn create_mystiko_config(core_config: &CoreConfig) -> MystikoConfig {
    let cfg_path = load_roller_config_path();
    let config_file = cfg_path + "/mystiko.json";
    if Path::exists(Path::new(&config_file)) {
        return MystikoConfig::from_json_file(&config_file).await.unwrap_or_else(|e| {
            error!("error occurred: {:?}", e);
            panic!("failed load core config from local file")
        });
    }

    info!("load core configure from remote url");
    let remote_options = create_remote_options(core_config);
    MystikoConfig::from_remote(&remote_options).await.unwrap_or_else(|e| {
        error!("error occurred: {:?}", e);
        panic!("failed load core config from remote")
    })
}

fn create_remote_options(core_config: &CoreConfig) -> RemoteOptions {
    let run_mod = load_roller_run_mod();
    let is_testnet = run_mod == "testnet";
    let mut remote_options = RemoteOptions::builder()
        .is_testnet(is_testnet)
        .is_staging(core_config.is_staging)
        .build();
    remote_options.base_url = core_config.remote_base_url.clone();
    remote_options.git_revision = core_config.git_revision.clone();

    remote_options
}

pub fn create_token_price_config() -> TokenPriceConfig {
    let cfg_path = load_roller_config_path();
    let run_mod = load_roller_run_mod();

    TokenPriceConfig::new(&run_mod, Some(&cfg_path)).unwrap_or_else(|e| {
        error!("error occurred: {:?}", e);
        panic!("load token price config failed")
    })
}

pub fn create_tx_manager_config() -> TxManagerConfig {
    let cfg_path = load_roller_config_path();
    let run_mod = load_roller_run_mod();

    TxManagerConfig::new(&run_mod, Some(&cfg_path)).unwrap_or_else(|e| {
        error!("error occurred: {:?}", e);
        panic!("load tx manager config failed")
    })
}
