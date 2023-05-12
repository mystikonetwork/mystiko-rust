use anyhow::Result;
use mystiko_config::wrapper::indexer::IndexerConfig;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment_queued::{CommitmentQueuedForContractRequest, CommitmentQueuedResponse};

pub struct IndexerInstance {
    client: IndexerClient,
}
impl IndexerInstance {
    pub fn new(cfg: &IndexerConfig) -> Self {
        let client = IndexerClient::builder(cfg.url()).build().unwrap();
        IndexerInstance { client }
    }

    pub async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        self.client
            .query_contract_sync_response(chain_id as u32, contract_address)
            .await
            .and_then(move |response| Ok(response.current_sync_block_num as u64))
    }

    pub async fn get_commitment_included_count(&self, chain_id: u64, contract_address: &str, end: u64) -> Result<u32> {
        self.client
            .count_commitment_included_for_contract(chain_id as u32, contract_address.to_string(), end as u32)
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
        self.client
            .find_commitment_queued_for_contract(&CommitmentQueuedForContractRequest {
                chain_id: chain_id as u32,
                address: contract_address.to_string(),
                start_block: Some(start as u32),
                end_block: Some(end as u32),
                where_filter: None,
            })
            .await
    }
}
