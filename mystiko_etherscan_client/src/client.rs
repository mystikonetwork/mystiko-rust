use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderValue, ACCEPT},
    {Client, IntoUrl},
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use ethers::types::U64;
use ethers_contract::{EthEvent, LogMeta};
use ethers_core::types::{Block, Log, Transaction, TransactionReceipt};
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};
use typed_builder::TypedBuilder;

use crate::config::get_default_base_url;
use crate::errors::EtherFetcherError;
use crate::response::{EtherScanResponse, JsonRpcResponse};
use crate::retry::RetryPolicy;

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct GetLogsOptions {
    pub address: String,
    pub from_block: u64,
    pub to_block: u64,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct GetOptions<U> {
    #[builder(setter(strip_option), default = None)]
    pub current_retry_time: Option<u64>,
    pub url: U,
    pub module: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event<R> {
    pub raw: R,
    pub metadata: LogMeta,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
pub struct EtherScanClientOptions {
    pub chain_id: u64,
    pub api_key: String,
    #[builder(setter(strip_option), default = None)]
    base_url: Option<String>,
    #[builder(setter(strip_option), default = None)]
    offset: Option<u64>,
    #[builder(setter(strip_option), default = None)]
    max_requests_per_second: Option<u128>,
    #[builder(setter(strip_option), default = None)]
    retry_policy: Option<RetryPolicy>,
}

#[derive(Debug, TypedBuilder)]
pub struct EtherScanClient {
    pub chain_id: u64,
    pub offset: u64,
    pub base_url: String,
    pub api_key: String,
    pub client: reqwest::Client,
    pub max_requests_per_second: u128,
    pub retry_policy: RetryPolicy,
    pub last_request_time: Mutex<Option<Instant>>,
}

impl EtherScanClient {
    pub fn new(options: EtherScanClientOptions) -> EtherScanClient {
        let chain_id = options.chain_id;
        let offset = options.offset.unwrap_or(1000);
        let api_key = options.api_key;
        let base_url = options.base_url.unwrap_or(get_default_base_url(chain_id).unwrap());
        let axios_instance = Client::new();
        let max_requests_per_second = options.max_requests_per_second.unwrap_or(5);
        let retry_policy = options.retry_policy.unwrap_or(RetryPolicy::default());
        Self {
            chain_id,
            offset,
            base_url,
            api_key,
            client: axios_instance,
            max_requests_per_second,
            last_request_time: None.into(),
            retry_policy,
        }
    }

    pub async fn eth_call(&mut self, to: &str, function_encoded_data: &str, block_tag: Option<&str>) -> Result<String> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_call".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("apikey={}", &self.api_key));
        params.push(format!("to={}", to));
        params.push(format!("data={}", function_encoded_data));
        params.push(format!("tag={}", block_tag.unwrap_or("latest")));
        let url = format!("{}/api?{}", self.base_url, params.join("&"));
        let options = GetOptions::<String>::builder()
            .url(url)
            .module("proxy".to_string())
            .build();
        return match self.get::<String, _>(options).await? {
            Some(result) => return Ok(result),
            None => Err(anyhow!("eth call error")),
        };
    }

    pub async fn get_block_number(&mut self) -> Result<U64> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_blockNumber".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("apikey={}", &self.api_key));
        let url = format!("{}/api?{}", self.base_url, params.join("&"));
        let options = GetOptions::<String>::builder()
            .url(url)
            .module("proxy".to_string())
            .build();
        return match self.get::<U64, _>(options).await? {
            Some(result) => Ok(result),
            None => Err(anyhow!(EtherFetcherError::CustomError(
                "get block number error".to_string()
            ))),
        };
    }

    pub async fn get_block_by_number(&mut self, block_number: u64) -> Result<Block<Transaction>> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getBlockByNumber".to_string());
        params.push("module=proxy".to_string());
        params.push("boolean=false".to_string());
        params.push(format!("apikey={}", &self.api_key));
        params.push(format!("tag={:x}", block_number));
        let url = format!("{}/api?{}", self.base_url, params.join("&"));
        let options = GetOptions::<String>::builder()
            .url(url)
            .module("proxy".to_string())
            .build();
        return match self.get::<Block<Transaction>, _>(options).await? {
            Some(result) => return Ok(result),
            None => Err(anyhow!(EtherFetcherError::CustomError(
                "get block by number error".to_string()
            ))),
        };
    }
    pub async fn get_transaction_by_hash(&mut self, transaction_hash: &str) -> Result<Transaction> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getTransactionByHash".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("txhash={}", transaction_hash));
        params.push(format!("apikey={}", &self.api_key));
        let url = format!("{}/api?{}", self.base_url, params.join("&"));
        let options = GetOptions::<String>::builder()
            .url(url)
            .module("proxy".to_string())
            .build();
        return match self.get::<Transaction, _>(options).await? {
            Some(result) => return Ok(result),
            None => Err(anyhow!(EtherFetcherError::CustomError(
                "get transaction by hash error".to_string()
            ))),
        };
    }

    pub async fn get_transaction_receipt(&mut self, transaction_hash: &str) -> Result<TransactionReceipt> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getTransactionReceipt".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("txhash={}", transaction_hash));
        params.push(format!("apikey={}", &self.api_key));
        let url = format!("{}/api?{}", self.base_url, params.join("&"));
        let options = GetOptions::<String>::builder()
            .url(url)
            .module("proxy".to_string())
            .build();
        return match self.get::<TransactionReceipt, _>(options).await? {
            Some(result) => return Ok(result),
            None => Err(anyhow!(EtherFetcherError::CustomError(
                "get transaction receipt error".to_string()
            ))),
        };
    }

    pub async fn fetch_event_logs<R>(&mut self, options: GetLogsOptions) -> Result<Vec<Event<R>>>
    where
        R: EthEvent + std::fmt::Debug,
    {
        let mut params: Vec<String> = vec![];
        params.push("action=getLogs".to_string());
        params.push("module=logs".to_string());
        params.push(format!("fromBlock={}", options.from_block));
        params.push(format!("toBlock={}", options.to_block));
        params.push(format!("offset={}", &self.offset));
        params.push(format!("address={}", options.address));
        params.push(format!("topic0=0x{:x}", R::signature()));
        params.push(format!("apikey={}", &self.api_key));
        let mut events: Vec<Event<R>> = Vec::new();
        let mut page = 1;
        loop {
            let url = format!("{}/api?{}&page={}", self.base_url, params.join("&"), page);
            let options = GetOptions::<String>::builder()
                .module("logs".to_string())
                .url(url)
                .build();
            if let Some(logs) = self.get::<Vec<Log>, _>(options).await? {
                if logs.is_empty() {
                    break;
                }
                let len = logs.len();
                for log in logs.into_iter() {
                    let metadata: LogMeta = (&log).into();
                    let raw_log = log.into();
                    let event = R::decode_log(&raw_log)?;
                    events.push(Event { raw: event, metadata });
                    page += 1;
                }
                if len < self.offset as usize {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(events)
    }

    pub async fn get<R, U>(&mut self, options: GetOptions<U>) -> Result<Option<R>>
    where
        R: DeserializeOwned + Send + Serialize,
        U: IntoUrl + Clone,
    {
        self.get_with_retry::<R, U>(options).await
    }

    pub async fn get_with_retry<R, U>(&self, options: GetOptions<U>) -> Result<Option<R>>
    where
        R: DeserializeOwned + Send + Serialize,
        U: IntoUrl + Clone,
    {
        let mut current_retry_time = options.current_retry_time.unwrap_or(1);
        loop {
            self.throttle().await;
            match self.handle_response(options.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    if self.retry_policy.is_retryable(&err, current_retry_time) {
                        current_retry_time += 1;
                        continue;
                    } else {
                        return Err(anyhow!("request error: {}", err));
                    }
                }
            };
        }
    }

    pub async fn handle_response<R, U>(&self, options: GetOptions<U>) -> Result<Option<R>>
    where
        R: DeserializeOwned + Serialize,
        U: IntoUrl,
    {
        let response = self
            .client
            .get(options.url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        let response = response.error_for_status()?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok());
        if let Some(s) = content_type {
            if s.contains("application/json") {
                if options.module == "proxy" {
                    return self.handle_response_with_proxy::<R>(response).await;
                }
                return self.handle_response_with_logs::<R>(response).await;
            }
        }
        Err(anyhow!(EtherFetcherError::CustomError(
            content_type.unwrap_or("").to_string()
        )))
    }

    async fn handle_response_with_logs<R>(&self, response: reqwest::Response) -> Result<Option<R>>
    where
        R: DeserializeOwned + Serialize,
    {
        let response_text = response.text().await?;
        let response = serde_json::from_str::<EtherScanResponse<Value>>(&response_text)?;
        if response.status.ne("1") {
            Err(anyhow!("request failed: {}", response_text))
        } else if let Some(result) = response.result {
            Ok(Some(serde_json::from_value::<R>(result)?))
        } else {
            Ok(None)
        }
    }

    async fn handle_response_with_proxy<R>(&self, response: reqwest::Response) -> Result<Option<R>>
    where
        R: DeserializeOwned + Serialize,
    {
        let response = response.text().await?;
        return match serde_json::from_str::<JsonRpcResponse<Value>>(&response) {
            Ok(json_rpc_response) => {
                if let Some(error) = json_rpc_response.error {
                    Err(error.into())
                } else if let Some(result) = json_rpc_response.result {
                    Ok(Some(serde_json::from_value::<R>(result)?))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Err(anyhow!("request failed with message: {}", response)),
        };
    }

    async fn throttle(&self) {
        let now: Instant = Instant::now();
        let mut last_request_time = self.last_request_time.lock().await;
        if let Some(last_instant) = last_request_time.as_ref() {
            let elapsed = now.duration_since(*last_instant).as_millis();
            let wait_ms = 1000 / self.max_requests_per_second;
            if elapsed < wait_ms {
                sleep(Duration::from_millis((wait_ms - elapsed) as u64)).await;
            }
        }
        *last_request_time = Some(now);
    }
}
