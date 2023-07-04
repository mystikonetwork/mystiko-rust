use crate::chain::ChainDataGiver;
use crate::common::env::load_chain_explorer_api_key;
use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;
use std::time::Duration;

#[derive(Clone)]
pub struct ExplorerStub {
    url: String,
    key: String,
    client: reqwest::Client,
}

impl ExplorerStub {
    pub fn new(url: &str) -> Self {
        let key = load_chain_explorer_api_key().unwrap();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        ExplorerStub {
            url: url.to_string(),
            key,
            client,
        }
    }
}

#[async_trait]
impl ChainDataGiver for ExplorerStub {
    fn data_source(&self) -> ChainDataSource {
        ChainDataSource::Explorer
    }

    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        // todo!()
        let _ = self.url.clone();
        let _ = self.key.clone();
        let _ = self.client.clone();
        let _ = chain_id;
        let _ = contract_address;
        Err(RollerError::NoChainExplorer)
    }

    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<usize> {
        // todo!()
        let _ = chain_id;
        let _ = contract_address;
        Err(RollerError::NoChainExplorer)
    }

    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>> {
        // todo!()
        let _ = chain_id;
        let _ = contract_address;
        let _ = start;
        let _ = end;
        Err(RollerError::NoChainExplorer)
    }
}
