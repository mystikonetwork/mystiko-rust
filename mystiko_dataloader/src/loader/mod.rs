mod chain;
mod config;

pub use chain::*;
pub use config::*;

use crate::DataLoaderError;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

pub const DEFAULT_SCHEDULE_INTERVAL_MS: u64 = 120_000_u64;
pub const DEFAULT_FETCHER_QUERY_LOADED_BLOCK_TIMEOUT_MS: u64 = 5_000_u64;
pub const DEFAULT_FETCHER_FETCH_TIMEOUT_MS: u64 = 120_000_u64;

pub const DEFAULT_VALIDATOR_CONCURRENCY: usize = 1_usize;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadOption {
    #[builder(default = LoadFetcherOption::builder().build())]
    pub fetcher: LoadFetcherOption,
    #[builder(default = LoadValidatorOption::builder().build())]
    pub validator: LoadValidatorOption,
}

impl From<Option<LoadOption>> for LoadOption {
    fn from(opt: Option<LoadOption>) -> LoadOption {
        opt.unwrap_or_default()
    }
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadFetcherOption {
    #[builder(default = DEFAULT_FETCHER_QUERY_LOADED_BLOCK_TIMEOUT_MS)]
    pub query_loaded_block_timeout_ms: u64,
    #[builder(default = DEFAULT_FETCHER_FETCH_TIMEOUT_MS)]
    pub fetch_timeout_ms: u64,
}

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadValidatorOption {
    #[builder(default = DEFAULT_VALIDATOR_CONCURRENCY)]
    pub concurrency: usize,
}

impl Default for LoadOption {
    fn default() -> Self {
        Self::builder().build()
    }
}

pub type DataLoaderResult<T> = anyhow::Result<T, DataLoaderError>;

#[async_trait]
pub trait DataLoader {
    async fn load<O>(&self, options: O) -> DataLoaderResult<()>
    where
        O: Into<LoadOption> + Send + Sync;
}
