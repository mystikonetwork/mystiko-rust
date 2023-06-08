pub mod chain_explorer;
pub mod indexer;
pub mod provider;

use crate::common::error::Result;
use crate::common::types::SyncType;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;

#[async_trait]
pub trait SyncTrait {
    fn sync_type(&self) -> SyncType;
    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64>;
    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<u32>;
    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>>;
}
