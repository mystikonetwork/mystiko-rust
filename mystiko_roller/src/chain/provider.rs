use crate::chain::ChainDataGiver;
use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, U64};
use ethers_providers::Middleware;
use mystiko_abi::commitment_pool::CommitmentPool;
use mystiko_ethers::provider::factory::Provider;
use mystiko_utils::convert::u256_to_big_int;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error};

pub struct ProviderStub {
    provider: Arc<Provider>,
    pool: CommitmentPool<Provider>,
}

impl ProviderStub {
    pub fn new(contract_address: &str, provider: Arc<Provider>) -> Self {
        let address = Address::from_str(contract_address).expect("invalid contract address");
        let pool = CommitmentPool::new(address, provider.clone());

        ProviderStub { provider, pool }
    }

    pub async fn get_queued_commitments_inner(
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

        // todo get commitment block number
        let info_cms = cms
            .iter()
            .map(|cm| {
                Ok(CommitmentInfo {
                    chain_id,
                    contract_address: contract_address.to_string(),
                    commitment_hash: u256_to_big_int(&cm.commitment),
                    block_number: start,
                    rollup_fee: cm.rollup_fee.to_string(),
                    leaf_index: cm.leaf_index.as_u32(),
                    tx_hash: "".to_string(),
                })
            })
            .collect::<Result<Vec<CommitmentInfo>>>()?;
        Ok(info_cms)
    }
}

#[async_trait]
impl ChainDataGiver for ProviderStub {
    fn data_source(&self) -> ChainDataSource {
        ChainDataSource::Provider
    }

    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        let _ = chain_id;
        let _ = contract_address;

        self.provider
            .get_block_number()
            .await
            .map_err(|e| e.into())
            .map(|n| n.as_u64())
    }

    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<usize> {
        let _ = chain_id;
        let _ = contract_address;

        self.pool
            .get_commitment_included_count()
            .await
            .map_err(|e| RollerError::ContractCallError(e.to_string()))
            .map(|c| c.as_usize())
    }

    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>> {
        // create new tokio runtime because ethers event filter not support Send
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                error!("create tokio runtime error: {}", e);
                RollerError::RuntimeError
            })?;
        rt.block_on(self.get_queued_commitments_inner(chain_id, contract_address, start, end))
    }
}
