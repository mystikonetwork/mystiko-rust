use crate::common::error::Result;
use crate::common::error::RollerError;
use mystiko_config::wrapper::mystiko::{MystikoConfig, RemoteOptions};
use mystiko_fs::read_file_bytes;
use mystiko_server_utils::token_price::config::TokenPriceConfig;
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::{error, info};

// todo use rust config crate
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RollerConfig {
    #[serde(rename = "logLevel", default = "default_log_level")]
    pub log_level: String,
    pub core: CoreConfig,
    pub chain: Chain,
    pub pull: Pull,
    pub rollup: Rollup,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub name: String,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CoreConfig {
    pub is_testnet: bool,
    pub is_staging: bool,
    pub remote_base_url: String,
    pub git_revision: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pull {
    #[serde(default = "default_check_interval_secs")]
    pub check_interval_secs: u64,
    #[serde(default = "default_batch_block_from_indexer")]
    pub batch_block_from_indexer: u32,
    #[serde(default = "default_batch_block_from_provider")]
    pub batch_block_from_provider: u32,
    #[serde(default = "default_block_gap_to_delay")]
    pub block_gap_to_delay: u32,
    #[serde(default = "default_pull_timeout_secs")]
    pub pull_timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rollup {
    #[serde(default = "default_merkle_tree_height")]
    pub merkle_tree_height: u32,
    #[serde(default = "default_force_rollup_block_count")]
    pub force_rollup_block_count: u64,
    #[serde(default = "default_max_count_for_indexer_sync")]
    pub max_count_for_indexer_sync: u32,
    #[serde(default = "default_rollup1_gas_cost")]
    pub rollup1_gas_cost: u32,
    #[serde(default = "default_rollup2_gas_cost")]
    pub rollup2_gas_cost: u32,
    #[serde(default = "default_rollup4_gas_cost")]
    pub rollup4_gas_cost: u32,
    #[serde(default = "default_rollup8_gas_cost")]
    pub rollup8_gas_cost: u32,
    #[serde(default = "default_rollup16_gas_cost")]
    pub rollup16_gas_cost: u32,
}

fn default_chain_name() -> String {
    "goerli".to_string()
}

fn default_chain_id() -> u64 {
    5
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_check_interval_secs() -> u64 {
    60
}

fn default_batch_block_from_indexer() -> u32 {
    100_000
}

fn default_batch_block_from_provider() -> u32 {
    500
}

fn default_block_gap_to_delay() -> u32 {
    2
}

fn default_pull_timeout_secs() -> u64 {
    40
}

fn default_merkle_tree_height() -> u32 {
    20
}

fn default_force_rollup_block_count() -> u64 {
    10
}

fn default_max_count_for_indexer_sync() -> u32 {
    15
}

fn default_rollup1_gas_cost() -> u32 {
    331_000
}

fn default_rollup2_gas_cost() -> u32 {
    336_000
}

fn default_rollup4_gas_cost() -> u32 {
    340_000
}

fn default_rollup8_gas_cost() -> u32 {
    360_000
}

fn default_rollup16_gas_cost() -> u32 {
    410_000
}

impl RollerConfig {
    pub fn default() -> Self {
        RollerConfig {
            log_level: default_log_level(),
            core: CoreConfig {
                is_testnet: false,
                is_staging: false,
                remote_base_url: "".to_string(),
                git_revision: "".to_string(),
            },
            chain: Chain {
                name: default_chain_name(),
                chain_id: default_chain_id(),
            },
            pull: Pull {
                check_interval_secs: default_check_interval_secs(),
                batch_block_from_indexer: default_batch_block_from_indexer(),
                batch_block_from_provider: default_batch_block_from_provider(),
                block_gap_to_delay: default_block_gap_to_delay(),
                pull_timeout_secs: default_pull_timeout_secs(),
            },
            rollup: Rollup {
                merkle_tree_height: default_merkle_tree_height(),
                force_rollup_block_count: default_force_rollup_block_count(),
                max_count_for_indexer_sync: default_max_count_for_indexer_sync(),
                rollup1_gas_cost: default_rollup1_gas_cost(),
                rollup2_gas_cost: default_rollup2_gas_cost(),
                rollup4_gas_cost: default_rollup4_gas_cost(),
                rollup8_gas_cost: default_rollup8_gas_cost(),
                rollup16_gas_cost: default_rollup16_gas_cost(),
            },
        }
    }
}

impl RollerConfig {
    pub async fn from_json_file(file_path_str: &str) -> Result<RollerConfig> {
        let file = read_file_bytes(file_path_str)
            .await
            .map_err(|why| RollerError::FileError(why.to_string()))?;
        let config: RollerConfig = serde_json::from_slice(&file)?;
        Ok(config)
    }
}

impl Rollup {
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

pub async fn create_roller_config(cfg_path: &str) -> Result<RollerConfig> {
    RollerConfig::from_json_file(&(cfg_path.to_owned() + "/roller.json")).await
}

pub async fn create_mystiko_config(config_path: &str, core_config: &CoreConfig) -> Result<Arc<MystikoConfig>> {
    let config_file = config_path.to_owned() + "/core.json";
    if Path::new(&config_file).exists() {
        let cfg = MystikoConfig::from_json_file(&config_file).await;
        if cfg.is_ok() {
            info!("load core configure from local file");
            return Ok(Arc::new(cfg.unwrap()));
        } else {
            error!("core config file is invalid {:?}", cfg.err());
            return Err(RollerError::LoadConfigError("core".to_string()));
        }
    }

    info!("load core configure from remote url");
    let mut remote_options = RemoteOptions::builder()
        .is_testnet(core_config.is_testnet)
        .is_staging(core_config.is_staging)
        .build();
    remote_options.base_url = Some(core_config.remote_base_url.to_string());
    remote_options.git_revision = Some(core_config.git_revision.to_string());
    let cfg = MystikoConfig::from_remote(&remote_options)
        .await
        .map_err(|e| RollerError::LoadConfigError(e.to_string()))?;
    Ok(Arc::new(cfg))
}

pub async fn create_token_price_config(config_path: &str) -> Result<TokenPriceConfig> {
    let config_file = config_path.to_owned() + "/token_price.json";
    if Path::new(&config_file).exists() {
        let cfg = TokenPriceConfig::from_json_file(&config_file).await;
        if cfg.is_ok() {
            Ok(cfg.unwrap())
        } else {
            error!("token price config file is invalid {:?}", cfg.err());
            Err(RollerError::LoadConfigError("token price".to_string()))
        }
    } else {
        Ok(TokenPriceConfig::default())
    }
}

pub async fn create_tx_manager_config(config_path: &str) -> Result<TxManagerConfig> {
    let config_file = config_path.to_owned() + "/tx_manager.json";
    if Path::new(&config_file).exists() {
        let cfg = TxManagerConfig::from_json_file(&config_file).await;
        if cfg.is_ok() {
            Ok(cfg.unwrap())
        } else {
            error!("tx manager config file is invalid {:?}", cfg.err());
            Err(RollerError::LoadConfigError("tx manager".to_string()))
        }
    } else {
        Ok(TxManagerConfig::default())
    }
}
