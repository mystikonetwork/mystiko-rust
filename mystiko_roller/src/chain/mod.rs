pub mod event_log;
pub mod explorer;
pub mod indexer;
pub mod provider;

use crate::common::error::Result;
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;

#[async_trait]
pub trait ChainDataGiver: Send + Sync {
    fn data_source(&self) -> ChainDataSource;
    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64>;
    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<usize>;
    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>>;
}
