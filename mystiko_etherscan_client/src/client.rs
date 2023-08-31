use std::collections::HashSet;
use std::time::Duration;

use reqwest::{
    header::{HeaderValue, ACCEPT},
    {Client, IntoUrl},
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use ethers::types::U64;
use ethers_contract::EthEvent;
use ethers_core::types::{Block, Transaction, TransactionReceipt};
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};
use typed_builder::TypedBuilder;

use crate::config::{
    get_default_base_url, DEFAULT_MAX_REQUESTS_PER_SECOND, DEFAULT_PAGE_OFFSET, DEFAULT_URL_PREFIX, MAX_OFFSET,
};
use crate::errors::EtherScanError;
use crate::log::{Log, LogMeta};
use crate::response::{EtherScanResponse, JsonRpcResponse, Result};
use crate::retry::{DefaultRetryPolicy, RetryPolicy};

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GetLogsOptions {
    pub address: String,
    pub from_block: u64,
    pub to_block: u64,
}

#[derive(Debug, Clone, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct GetOptions<U> {
    pub url: U,
    pub module: EtherScanModule,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum EtherScanModule {
    #[default]
    Normal = 1,
    JsonRpcProxy = 2,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event<R> {
    pub raw: R,
    pub metadata: LogMeta,
}

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EtherScanClientOptions {
    pub chain_id: u64,
    pub api_key: String,
    #[builder(default = None)]
    pub base_url: Option<String>,
    #[builder(default = None)]
    pub client: Option<Client>,
    #[builder(default = None)]
    pub max_requests_per_second: Option<u128>,
    #[builder(default = None)]
    pub offset: Option<u64>,
    #[builder(default = None)]
    pub retry_policy: Option<Box<dyn RetryPolicy>>,
    #[builder(default = None)]
    pub url_prefix: Option<String>,
}

#[derive(Debug, TypedBuilder)]
pub struct EtherScanClient {
    pub chain_id: u64,
    pub offset: u64,
    pub base_url: String,
    api_key: String,
    client: reqwest::Client,
    last_request_time: Mutex<Option<Instant>>,
    max_requests_per_second: u128,
    retry_policy: Box<dyn RetryPolicy>,
    url_prefix: String,
}

impl EtherScanClient {
    pub fn new(options: EtherScanClientOptions) -> Result<Self> {
        let chain_id = options.chain_id;
        let offset = options.offset.unwrap_or(DEFAULT_PAGE_OFFSET);
        if offset > MAX_OFFSET {
            return Err(EtherScanError::ParamCheckError(format!(
                "offset must be less than or equal to {}. offset = {}",
                MAX_OFFSET, offset
            )));
        }
        let api_key = options.api_key;
        let base_url = options.base_url.unwrap_or(get_default_base_url(chain_id)?);
        let client = options.client.unwrap_or(Client::new());
        let max_requests_per_second = options
            .max_requests_per_second
            .unwrap_or(DEFAULT_MAX_REQUESTS_PER_SECOND);
        let retry_policy = options.retry_policy.unwrap_or(Box::<DefaultRetryPolicy>::default());
        let url_prefix = options.url_prefix.unwrap_or(DEFAULT_URL_PREFIX.into());
        Ok(Self {
            chain_id,
            offset,
            base_url,
            api_key,
            client,
            max_requests_per_second,
            last_request_time: None.into(),
            retry_policy,
            url_prefix,
        })
    }

    pub async fn eth_call(&self, to: &str, function_encoded_data: &str, block_tag: Option<&str>) -> Result<String> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_call".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("apikey={}", &self.api_key));
        params.push(format!("to={}", to));
        params.push(format!("data={}", function_encoded_data));
        params.push(format!("tag={}", block_tag.unwrap_or("latest")));
        let url = self.format_url(&params);
        let options = GetOptions::<String>::builder()
            .url(url)
            .module(EtherScanModule::JsonRpcProxy)
            .build();
        match self.get::<String, _>(options).await? {
            Some(result) => Ok(result),
            None => Err(EtherScanError::UnknownError("eth call error".to_string())),
        }
    }

    pub async fn get_block_number(&self) -> Result<U64> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_blockNumber".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("apikey={}", &self.api_key));
        let url = self.format_url(&params);
        let options = GetOptions::<String>::builder()
            .url(url)
            .module(EtherScanModule::JsonRpcProxy)
            .build();
        match self.get::<U64, _>(options).await? {
            Some(result) => Ok(result),
            None => Err(EtherScanError::MissingCurrentBlock(
                "get block number error".to_string(),
            )),
        }
    }

    pub async fn get_block_by_number(&self, block_number: u64) -> Result<Option<Block<Transaction>>> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getBlockByNumber".to_string());
        params.push("module=proxy".to_string());
        params.push("boolean=false".to_string());
        params.push(format!("apikey={}", &self.api_key));
        params.push(format!("tag={:x}", block_number));
        let url = self.format_url(&params);
        let options = GetOptions::<String>::builder()
            .url(url)
            .module(EtherScanModule::JsonRpcProxy)
            .build();
        self.get::<Block<Transaction>, _>(options).await
    }

    pub async fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<Option<Transaction>> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getTransactionByHash".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("txhash={}", transaction_hash));
        params.push(format!("apikey={}", &self.api_key));
        let url = self.format_url(&params);
        let options = GetOptions::<String>::builder()
            .url(url)
            .module(EtherScanModule::JsonRpcProxy)
            .build();
        self.get::<Transaction, _>(options).await
    }

    pub async fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<Option<TransactionReceipt>> {
        let mut params: Vec<String> = vec![];
        params.push("action=eth_getTransactionReceipt".to_string());
        params.push("module=proxy".to_string());
        params.push(format!("txhash={}", transaction_hash));
        params.push(format!("apikey={}", &self.api_key));
        let url = self.format_url(&params);
        let options = GetOptions::<String>::builder()
            .url(url)
            .module(EtherScanModule::JsonRpcProxy)
            .build();
        self.get::<TransactionReceipt, _>(options).await
    }

    pub async fn fetch_event_logs<R>(&self, options: GetLogsOptions) -> Result<Vec<Event<R>>>
    where
        R: EthEvent + std::fmt::Debug,
    {
        if self.offset > MAX_OFFSET {
            return Err(EtherScanError::ParamCheckError(format!(
                "offset must be less than or equal to {}. offset={}",
                MAX_OFFSET, self.offset
            )));
        }
        let mut params: Vec<String> = vec![];
        params.push("action=getLogs".to_string());
        params.push("module=logs".to_string());
        params.push("page=1".to_string());
        params.push(format!("toBlock={}", options.to_block));
        params.push(format!("offset={}", &self.offset));
        params.push(format!("address={}", options.address));
        params.push(format!("topic0=0x{:x}", R::signature()));
        params.push(format!("apikey={}", &self.api_key));
        let mut raw_logs = HashSet::new();
        let mut events: Vec<Event<R>> = Vec::new();
        let mut from_block = options.from_block;
        loop {
            let url = format!("{}&fromBlock={}", self.format_url(&params), from_block);
            let options = GetOptions::<String>::builder()
                .module(EtherScanModule::Normal)
                .url(url)
                .build();
            if let Some(logs) = self.get::<Vec<Log>, _>(options).await? {
                if logs.is_empty() {
                    break;
                }
                let len = logs.len();
                for log in logs.into_iter() {
                    if raw_logs.insert(log.clone()) {
                        let metadata: LogMeta = (&log).into();
                        let event = R::decode_log(&log.into_raw())?;
                        events.push(Event { raw: event, metadata });
                    }
                }
                if let Some(max_block_number) = events.iter().map(|e| e.metadata.block_number.as_u64()).max() {
                    from_block = max_block_number;
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

    pub async fn get<R, U>(&self, options: GetOptions<U>) -> Result<Option<R>>
    where
        R: DeserializeOwned + Send + Serialize,
        U: IntoUrl + Clone,
    {
        self.get_with_retry::<R, U>(options).await
    }

    async fn get_with_retry<R, U>(&self, options: GetOptions<U>) -> Result<Option<R>>
    where
        R: DeserializeOwned + Send + Serialize,
        U: IntoUrl + Clone,
    {
        let mut current_retry_time = 1;
        loop {
            self.throttle().await;
            match self.handle_response(options.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    if self.retry_policy.is_retryable(&err, current_retry_time) {
                        current_retry_time += 1;
                        continue;
                    } else {
                        return Err(err);
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
                if options.module == EtherScanModule::JsonRpcProxy {
                    return self.handle_response_with_proxy::<R>(response).await;
                }
                return self.handle_response_with_logs::<R>(response).await;
            }
        }
        Err(EtherScanError::UnexpectedContentTypeError(
            content_type.unwrap_or_default().to_string(),
        ))
    }

    async fn handle_response_with_logs<R>(&self, response: reqwest::Response) -> Result<Option<R>>
    where
        R: DeserializeOwned + Serialize,
    {
        let response_text = response.text().await?;
        let response = serde_json::from_str::<EtherScanResponse<Value>>(&response_text)?;
        if response.status.ne("1") {
            if response.message.to_lowercase().contains("no records found") {
                return Ok(None);
            }
            Err(EtherScanError::ResponseError(format!(
                "request failed: {}",
                response_text
            )))
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
        match serde_json::from_str::<JsonRpcResponse<Value>>(&response) {
            Ok(json_rpc_response) => {
                if let Some(error) = json_rpc_response.error {
                    Err(EtherScanError::JsonRpcError(error))
                } else if let Some(result) = json_rpc_response.result {
                    Ok(Some(serde_json::from_value::<R>(result)?))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Err(EtherScanError::ResponseError(format!(
                "request failed with message: {}",
                response
            ))),
        }
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

    fn format_url(&self, params: &[String]) -> String {
        format!("{}{}?{}", self.base_url, self.url_prefix, params.join("&"))
    }
}
