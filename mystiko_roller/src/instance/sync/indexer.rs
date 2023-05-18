use anyhow::Result;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment_queued::{CommitmentQueuedForContractRequest, CommitmentQueuedResponse};
use std::time::Duration;
use tracing::debug;

pub struct IndexerInstance {
    client: IndexerClient,
}
impl IndexerInstance {
    pub fn new(cfg: &IndexerConfig) -> Self {
        let client = IndexerClient::builder(cfg.url())
            .timeout(Duration::from_millis(cfg.timeout_ms()))
            .build()
            .unwrap();
        IndexerInstance { client }
    }

    pub async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        debug!("get indexer latest block number");

        self.client
            .query_contract_sync_response(chain_id, contract_address)
            .await
            .and_then(move |response| Ok(response.current_sync_block_num as u64))
    }

    pub async fn get_commitment_included_count(&self, chain_id: u64, contract_address: &str, end: u64) -> Result<u32> {
        debug!("get indexer commitment included count at block number {:?} ", end);

        self.client
            .count_commitment_included_for_contract(chain_id, contract_address, end)
            .await
            .and_then(move |response| Ok(response))
    }

    pub async fn get_queued_commitment(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentQueuedResponse>> {
        debug!("get indexer queued commitment start: {:}, end: {:} ", start, end);

        self.client
            .find_commitment_queued_for_contract(&CommitmentQueuedForContractRequest {
                chain_id: chain_id,
                address: contract_address.to_string(),
                start_block: Some(start),
                end_block: Some(end),
                where_filter: None,
            })
            .await
    }
}
