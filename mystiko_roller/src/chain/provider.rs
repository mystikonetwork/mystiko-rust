use crate::chain::event_log::parse_event_logs;
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
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProviderStub {
    provider: Arc<Provider>,
    commitment_contract: CommitmentPool<Provider>,
}

impl ProviderStub {
    pub fn new(contract_address: &str, provider: Arc<Provider>) -> Self {
        let address = Address::from_str(contract_address).expect("invalid contract address");
        let commitment_contract = CommitmentPool::new(address, provider.clone());
        ProviderStub {
            provider,
            commitment_contract,
        }
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

        self.commitment_contract
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
        let event = self
            .commitment_contract
            .commitment_queued_filter()
            .from_block(BlockNumber::Number(U64::from(start)))
            .to_block(BlockNumber::Number(U64::from(end)));
        let logs = self.provider.get_logs(&event.filter).await?;
        let cms = parse_event_logs(chain_id, contract_address, logs)?;
        Ok(cms)
    }
}
