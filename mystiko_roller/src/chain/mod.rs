pub mod explorer;
pub mod indexer;
pub mod provider;

use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;
use ethers_contract::parse_log;
use ethers_core::types::Log;
use mystiko_abi::commitment_pool::CommitmentQueuedFilter;
use mystiko_utils::convert::u256_to_big_int;

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

pub fn parse_event_logs(chain_id: u64, contract_address: &str, logs: Vec<Log>) -> Result<Vec<CommitmentInfo>> {
    let cms = logs
        .into_iter()
        .map(|log| {
            let block_number = log
                .block_number
                .ok_or_else(|| RollerError::InvalidEventLogs("block number".to_string()))?
                .as_u64();
            let tx_hash = log
                .transaction_hash
                .ok_or_else(|| RollerError::InvalidEventLogs("transaction hash".to_string()))?;
            let tx_hash = format!("{:#x}", tx_hash);
            let cm = parse_log::<CommitmentQueuedFilter>(log)?;
            Ok(CommitmentInfo {
                chain_id,
                contract_address: contract_address.to_string(),
                commitment_hash: u256_to_big_int(&cm.commitment),
                block_number,
                rollup_fee: cm.rollup_fee.to_string(),
                leaf_index: cm.leaf_index.as_u32(),
                tx_hash,
            })
        })
        .collect::<Result<Vec<CommitmentInfo>>>()?;
    Ok(cms)
}
