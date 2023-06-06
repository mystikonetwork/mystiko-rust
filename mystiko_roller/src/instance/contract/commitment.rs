use crate::common::error::{Result, RollerError};
use crate::db::document::commitment::CommitmentInfo;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, U64};
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_utils::convert::u256_to_big_int;
use std::str::FromStr;
use std::sync::Arc;
use tracing::debug;

pub struct CommitmentContractInstance {
    chain_id: u64,
    contract_address: String,
    pool: CommitmentPool<Provider>,
}

impl CommitmentContractInstance {
    pub fn new(chain_id: u64, contract_address: &str, provider: Arc<Provider>) -> Self {
        let address = Address::from_str(contract_address).expect("invalid contract address");
        let pool = CommitmentPool::new(address, provider);
        CommitmentContractInstance {
            chain_id,
            contract_address: contract_address.to_string(),
            pool,
        }
    }

    pub async fn query_queued_commitments(&self, start: u64, end: u64) -> Result<Vec<CommitmentInfo>> {
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
                    chain_id: self.chain_id,
                    contract_address: self.contract_address.clone(),
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
