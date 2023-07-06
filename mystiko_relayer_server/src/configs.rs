use anyhow::{bail, Result};
use config::Config;
use lazy_static::lazy_static;
use log::debug;
use mystiko_relayer_config::wrapper::relayer::RelayerConfig;
use mystiko_types::NetworkType;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref REGEX_API_VERSION: Regex = Regex::new(r"^v\d+$").unwrap();
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    #[validate(regex(path = "REGEX_API_VERSION"))]
    pub api_version: String,
    pub network_type: NetworkType,
    #[validate(contains = ".sqlite")]
    pub sqlite_db_path: String,
    #[validate(length(min = 1))]
    pub coin_market_cap_api_key: String,
    #[validate]
    pub accounts: Vec<AccountConfig>,
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct AccountConfig {
    #[validate(range(min = 1))]
    pub chain_id: u64,
    pub available: bool,
    pub private_key: String,
    pub supported_erc20_tokens: Vec<String>,
    #[validate(range(min = 0.0001))]
    pub balance_alarm_threshold: f64,
    #[validate(range(min = 20000))]
    pub balance_check_interval_ms: u64,
}

impl ServerConfig {
    pub fn find_accounts(&self, chain_id: u64) -> Option<Vec<&AccountConfig>> {
        let mut accounts = Vec::new();
        for account in &self.accounts {
            if account.chain_id == chain_id {
                accounts.push(account);
            }
        }

        if accounts.is_empty() {
            None
        } else {
            Some(accounts)
        }
    }

    pub fn find_accounts_available(&self, chain_id: u64) -> Option<Vec<&AccountConfig>> {
        self.find_accounts(chain_id)
            .map(|accounts| accounts.into_iter().filter(|account| account.available).collect())
    }

    pub fn validation(&self, relayer_config: &RelayerConfig) -> Result<()> {
        for account in &self.accounts {
            // validate account supported erc20 tokens
            let chain_config_opt = relayer_config.find_chain_config(account.chain_id);
            if chain_config_opt.is_none() {
                bail!("chain id {} not found in relayer config", account.chain_id);
            }
            let chain_config = chain_config_opt.unwrap();
            let symbols = chain_config
                .contracts()
                .iter()
                .map(|contract| contract.asset_symbol().to_lowercase())
                .collect::<Vec<String>>();
            debug!("chain id {} symbols {:?}", account.chain_id, symbols);
            debug!(
                "server config supported erc20 tokens {:?}",
                &account.supported_erc20_tokens
            );
            for tokens in &account.supported_erc20_tokens {
                if !symbols.contains(&tokens.to_lowercase()) {
                    bail!(
                        "chain_id {} token {} not found in relayer chain config",
                        account.chain_id,
                        tokens
                    );
                }
            }
        }
        Ok(self.validate()?)
    }
}

pub fn load_config(path: &str) -> Result<ServerConfig> {
    // use config crate to load config
    let settings = Config::builder().add_source(config::File::with_name(path)).build()?;
    // settings deserialize to ServerConfig
    let config = settings.try_deserialize::<ServerConfig>()?;
    Ok(config)
}
