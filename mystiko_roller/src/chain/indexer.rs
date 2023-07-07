use crate::chain::ChainDataGiver;
use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment_queued::CommitmentQueuedForContractRequest;
use num_bigint::BigInt;
use std::time::Duration;
use tracing::info;

pub struct IndexerStub {
    client: IndexerClient,
}

impl IndexerStub {
    pub fn new(cfg: &IndexerConfig) -> Result<Self> {
        let client = IndexerClient::builder(cfg.url())
            .timeout(Duration::from_millis(cfg.timeout_ms()))
            .build()?;

        Ok(IndexerStub { client })
    }
}

#[async_trait]
impl ChainDataGiver for IndexerStub {
    fn data_source(&self) -> ChainDataSource {
        ChainDataSource::Indexer
    }

    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        self.client
            .query_contract_sync_response(chain_id, contract_address)
            .await
            .map(move |response| response.current_sync_block_num)
            .map_err(|e| e.into())
    }

    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<usize> {
        self.client
            .count_commitment_included_for_contract(chain_id, contract_address, None)
            .await
            .map(|c| c as usize)
            .map_err(RollerError::from)
    }

    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>> {
        info!("get indexer queued commitment start: {:}, end: {:} ", start, end);

        self.client
            .find_commitment_queued_for_contract(&CommitmentQueuedForContractRequest {
                chain_id,
                address: contract_address.to_string(),
                start_block: Some(start),
                end_block: Some(end),
                where_filter: None,
            })
            .await?
            .iter()
            .map(|cm| {
                let commitment_hash =
                    BigInt::parse_bytes(cm.commit_hash.as_bytes(), 10).ok_or(RollerError::InvalidCommitmentHash)?;
                Ok(CommitmentInfo {
                    chain_id,
                    contract_address: contract_address.to_string(),
                    commitment_hash,
                    block_number: cm.block_num,
                    rollup_fee: cm.rollup_fee.clone(),
                    leaf_index: cm.leaf_index,
                    tx_hash: cm.tx_hash.clone(),
                })
            })
            .collect::<Result<Vec<CommitmentInfo>>>()
    }
}
