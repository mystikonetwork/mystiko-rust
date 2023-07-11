use crate::chain::{parse_event_logs, ChainDataGiver};
use crate::common::env::load_chain_explorer_api_key;
use crate::common::error::{Result, RollerError};
use crate::config::roller::ChainDataSource;
use crate::db::document::commitment::CommitmentInfo;
use async_trait::async_trait;
use ethers_core::types::Log;
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExplorerRpcResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExplorerApiResponse {
    pub status: String,
    pub message: String,
    pub result: Value,
}

trait ExplorerResponse<T>: Debug + Clone + DeserializeOwned + Serialize {
    fn process_response_data(&self) -> Result<T>;
}

impl<T> ExplorerResponse<T> for ExplorerRpcResponse
where
    T: DeserializeOwned + Serialize,
{
    fn process_response_data(&self) -> Result<T> {
        let result = serde_json::from_value::<T>(self.result.clone())?;
        Ok(result)
    }
}

impl<T> ExplorerResponse<T> for ExplorerApiResponse
where
    T: DeserializeOwned + Serialize,
{
    fn process_response_data(&self) -> Result<T> {
        if self.status == "1" && self.message == "OK" {
            let result = serde_json::from_value::<T>(self.result.clone())?;
            Ok(result)
        } else {
            Err(RollerError::StubExplorerError(
                self.result.to_string(),
                self.status.parse::<i32>().unwrap_or(-1),
            ))
        }
    }
}

#[derive(Clone)]
pub struct ExplorerStub {
    url: String,
    key: String,
    client: reqwest::Client,
}

impl ExplorerStub {
    pub fn new(url: &str, timeout_secs: u64) -> Result<Self> {
        let key = load_chain_explorer_api_key()?;
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;
        Ok(ExplorerStub {
            url: url.to_string(),
            key,
            client,
        })
    }
}

impl ExplorerStub {
    async fn get<R, T>(&self, url: &str) -> Result<T>
    where
        R: ExplorerResponse<T>,
        T: DeserializeOwned + Serialize,
    {
        let response = self
            .client
            .get(url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        self.process_response::<R, T>(response).await
    }

    async fn process_response<R, T>(&self, response: Response) -> Result<T>
    where
        R: ExplorerResponse<T>,
        T: DeserializeOwned + Serialize,
    {
        let response = response.error_for_status()?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| RollerError::StubExplorerError("content type not found".to_string(), -1))?;

        if !content_type.contains("application/json") {
            return Err(RollerError::StubExplorerError(
                "content type not supported".to_string(),
                -1,
            ));
        }

        let data = response.bytes().await?;
        let response = serde_json::from_slice::<R>(data.as_ref());
        match response {
            Ok(r) => r.process_response_data(),
            Err(err) => Err(RollerError::StubExplorerError(err.to_string(), -1)),
        }
    }
}

#[async_trait]
impl ChainDataGiver for ExplorerStub {
    fn data_source(&self) -> ChainDataSource {
        ChainDataSource::Explorer
    }

    async fn get_latest_block_number(&self, chain_id: u64, contract_address: &str) -> Result<u64> {
        let _ = chain_id;
        let _ = contract_address;
        let action = "/api?module=proxy&action=eth_blockNumber";
        let url = format!("{}{}&apikey={}", self.url, action, self.key);

        let rsp = self.get::<ExplorerRpcResponse, String>(&url).await?;
        let block_number = u64::from_str_radix(rsp.trim_start_matches("0x"), 16)
            .map_err(|_| RollerError::StubExplorerError(format!("invalid block number response: {}", rsp), -1))?;
        Ok(block_number)
    }

    async fn get_included_count(&self, chain_id: u64, contract_address: &str) -> Result<usize> {
        let _ = chain_id;
        let action = format!(
            "/api?module=proxy&action=eth_call&to={}&data=0xe500f504&tag=latest",
            contract_address
        );
        let url = format!("{}{}&apikey={}", self.url, action, self.key);
        let rsp = self.get::<ExplorerRpcResponse, String>(&url).await?;
        let included_count = u64::from_str_radix(rsp.trim_start_matches("0x"), 16)
            .map_err(|_| RollerError::StubExplorerError(format!("invalid included count response: {}", rsp), -1))?;
        Ok(included_count as usize)
    }

    async fn get_queued_commitments(
        &self,
        chain_id: u64,
        contract_address: &str,
        start: u64,
        end: u64,
    ) -> Result<Vec<CommitmentInfo>> {
        let _ = chain_id;
        let action = format!(
            "/api?module=logs&action=getLogs&fromBlock={}&toBlock={}&address={}&topic0=0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
            start, end, contract_address,
        );
        let url = format!("{}{}&apikey={}", self.url, action, self.key);
        let logs = self.get::<ExplorerApiResponse, Vec<Log>>(&url).await?;
        let cms = parse_event_logs(chain_id, contract_address, logs)?;
        Ok(cms)
    }
}
