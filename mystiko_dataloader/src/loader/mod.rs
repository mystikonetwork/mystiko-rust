mod chain;
mod config;

pub use chain::*;
pub use config::*;

use crate::DataLoaderError;
use async_trait::async_trait;
use serde::Serialize;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

pub const DEFAULT_FETCHER_QUERY_LOADED_BLOCK_TIMEOUT_MS: u64 = 5_000_u64;
pub const DEFAULT_FETCHER_FETCH_TIMEOUT_MS: u64 = 300_000_u64;
pub const DEFAULT_VALIDATOR_CONCURRENCY: usize = 1_usize;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadOption {
    pub fetcher: LoadFetcherOption,
    pub validator: LoadValidatorOption,
}

impl From<Option<LoadOption>> for LoadOption {
    fn from(opt: Option<LoadOption>) -> LoadOption {
        opt.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadFetcherOption {
    #[builder(default = DEFAULT_FETCHER_QUERY_LOADED_BLOCK_TIMEOUT_MS)]
    pub query_loaded_block_timeout_ms: u64,
    #[builder(default = DEFAULT_FETCHER_FETCH_TIMEOUT_MS)]
    pub fetch_timeout_ms: u64,
    pub skips: HashMap<&'static str, LoadFetcherSkipOption>,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadFetcherSkipOption {
    pub skip_fetch: Option<bool>,
    pub skip_validation: Option<bool>,
}

impl Default for LoadFetcherOption {
    fn default() -> Self {
        LoadFetcherOption::builder().build()
    }
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadValidatorOption {
    #[builder(default = DEFAULT_VALIDATOR_CONCURRENCY)]
    pub concurrency: usize,
    pub skips: HashMap<String, LoadValidatorSkipOption>,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadValidatorSkipOption {
    pub skip_validation: Option<bool>,
    pub skip_checkers: HashMap<&'static str, bool>,
}

impl Default for LoadValidatorOption {
    fn default() -> Self {
        LoadValidatorOption::builder().build()
    }
}

impl Default for LoadOption {
    fn default() -> Self {
        Self::builder().build()
    }
}

pub type DataLoaderResult<T> = anyhow::Result<T, DataLoaderError>;

#[async_trait]
pub trait FromConfig<T>: Sized {
    async fn from_config(item: &T) -> DataLoaderConfigResult<Self>;
}

#[async_trait]
pub trait DataLoader: Send + Sync {
    async fn chain_loaded_block(&self, chain_id: u64) -> DataLoaderResult<Option<u64>>;

    async fn contract_loaded_block(&self, chain_id: u64, contract_address: &str) -> DataLoaderResult<Option<u64>>;

    async fn load<O>(&self, options: O) -> DataLoaderResult<()>
    where
        O: Into<LoadOption> + Send + Sync;
}
