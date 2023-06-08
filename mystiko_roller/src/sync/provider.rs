use crate::common::error::{Result, RollerError};
use crate::db::document::commitment::CommitmentInfo;
use crate::sync::SyncType;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, U64};
use ethers_providers::Middleware;
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_utils::convert::u256_to_big_int;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;

pub struct SyncProvider {
    provider: Arc<Provider>,
    pool: CommitmentPool<Provider>,
}

impl SyncProvider {
    pub fn new(contract_address: &str, provider: Arc<Provider>) -> Self {
        let address = Address::from_str(contract_address).expect("invalid contract address");
        let pool = CommitmentPool::new(address, provider.clone());

        SyncProvider { provider, pool }
    }

    pub fn sync_type(&self) -> SyncType {
        SyncType::Provider
    }

    pub async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        let _ = chain_id;
        let _ = contract_address;

        self.provider
            .get_block_number()
            .await
            .map_err(|e| e.into())
            .map(|n| n.as_u64())
    }

    pub async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<u32> {
        let _ = chain_id;
        let _ = contract_address;

        self.pool
            .get_commitment_included_count()
            .await
            .map_err(|e| RollerError::ContractCallError(e.to_string()))
            .map(|c| c.as_u32())
    }

    pub async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>> {
        debug!("query queued commitments from {} to {}", start, end);

        let event = self
            .pool
            .commitment_queued_filter()
            .from_block(BlockNumber::Number(U64::from(start)))
            .to_block(BlockNumber::Number(U64::from(end)));

        let cms = event
            .query()
            .await
            .map_err(|e| RollerError::ContractCallError(e.to_string()))?;

        let info_cms = cms
            .iter()
            .map(|cm| {
                Ok(CommitmentInfo {
                    chain_id,
                    contract_address: contract_address.to_string(),
                    commitment_hash: u256_to_big_int(&cm.commitment),
                    block_number: end,
                    rollup_fee: cm.rollup_fee.to_string(),
                    leaf_index: cm.leaf_index.as_u32(),
                    tx_hash: "".to_string(),
                })
            })
            .collect::<Result<Vec<CommitmentInfo>>>()?;
        Ok(info_cms)
    }
}
