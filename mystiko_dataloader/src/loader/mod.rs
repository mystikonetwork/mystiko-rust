mod chain;
mod config;

pub use chain::*;
pub use config::*;

use crate::DataLoaderError;
use async_trait::async_trait;
use typed_builder::TypedBuilder;

pub const DEFAULT_SCHEDULE_INTERVAL_MS: u64 = 120_000_u64;
pub const DEFAULT_DELAY_BLOCK: u64 = 10_u64;
pub const DEFAULT_VALIDATOR_CONCURRENCY: usize = 1_usize;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct LoadOption {
    #[builder(default = Some(DEFAULT_DELAY_BLOCK))]
    pub delay_block: Option<u64>,
    #[builder(default = DEFAULT_VALIDATOR_CONCURRENCY)]
    pub validator_concurrency: usize,
}

impl Default for LoadOption {
    fn default() -> Self {
        Self::builder().build()
    }
}

pub type DataLoaderResult<T> = anyhow::Result<T, DataLoaderError>;

#[async_trait]
pub trait DataLoader {
    async fn load(&self, options: Option<LoadOption>) -> DataLoaderResult<()>;
}
