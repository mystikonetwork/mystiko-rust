use crate::error::RelayerClientError;
use crate::types::register::RegisterInfo;
use crate::types::result::Result;
use ethers_core::types::Address;
use futures::future::try_join_all;
use log::{debug, LevelFilter};
use mystiko_ethers::provider::pool::ProviderPool;
use mystiko_relayer_abi::mystiko_gas_relayer::MystikoGasRelayer;
use mystiko_relayer_config::wrapper::relayer::{RelayerConfig, RemoteOptions};
use mystiko_relayer_types::response::ApiResponse;
use mystiko_relayer_types::{
    RegisterInfoRequest, RegisterInfoResponse, RelayTransactRequest, RelayTransactResponse, RelayTransactStatusRequest,
    RelayTransactStatusResponse, TransactStatus, WaitingTransactionRequest,
};
use mystiko_types::NetworkType;
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::{Client, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::{sleep, Instant};
use typed_builder::TypedBuilder;
use validator::Validate;

pub const INFO_URL_PATH: &str = "info";
pub const TRANSACT_URL_PATH: &str = "transact";
pub const TRANSACTION_STATUS_URL_PATH: &str = "transaction/status";

#[derive(Debug)]
pub struct RelayerClient {
    pub reqwest_client: Client,
    pub network_type: NetworkType,
    pub relayer_config: Arc<RelayerConfig>,
    pub providers: Arc<RwLock<ProviderPool>>,
}

#[derive(TypedBuilder, Debug)]
pub struct RelayerClientOptions {
    #[builder(default, setter(strip_option))]
    pub relayer_config_file_path: Option<String>,
    #[builder(default, setter(strip_option))]
    pub relayer_config_remote_base_url: Option<String>,
    #[builder(default, setter(strip_option))]
    pub relayer_config_git_revision: Option<String>,
    #[builder(default = false)]
    pub is_testnet: bool,
    #[builder(default = false)]
    pub is_staging: bool,
    #[builder(default = Duration::from_millis(60000))]
    pub timeout: Duration,
    #[builder(default = LevelFilter::Info)]
    pub log_level: LevelFilter,
}

impl RelayerClient {
    pub async fn new(providers: Arc<RwLock<ProviderPool>>, options: Option<RelayerClientOptions>) -> Result<Self> {
        let relayer_options = options.unwrap_or(RelayerClientOptions::builder().build());
        let relayer_config = create_relayer_config(&relayer_options).await?;
        let network_type = if relayer_options.is_testnet {
            NetworkType::Testnet
        } else {
            NetworkType::Mainnet
        };
        let reqwest_client = Client::builder()
            .timeout(relayer_options.timeout)
            .build()
            .map_err(RelayerClientError::ReqwestError)?;
        log::set_max_level(relayer_options.log_level);
        Ok(Self {
            reqwest_client,
            network_type,
            relayer_config,
            providers,
        })
    }

    pub async fn all_register_info(self, request: RegisterInfoRequest) -> Result<Vec<RegisterInfo>> {
        debug!("all register info {:?}", request);

        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        let chain_id = request.chain_id;

        let mut provider_pool = self.providers.write().await;
        let provider = provider_pool
            .get_or_create_provider(chain_id)
            .await
            .map_err(|err| RelayerClientError::GetOrCreateProviderError(err.to_string()))?;
        drop(provider_pool);

        let chain_config_option = self.relayer_config.find_chain_config(chain_id);
        if chain_config_option.is_none() {
            return Err(RelayerClientError::RelayerConfigNotFoundError(chain_id));
        }
        let chain_config = chain_config_option.unwrap();

        let contract_address =
            Address::from_str(chain_config.relayer_contract_address()).map_err(RelayerClientError::FromHexError)?;

        let contract = MystikoGasRelayer::new(contract_address, provider);

        let relayer_info = contract
            .get_all_relayer_info()
            .await
            .map_err(|err| RelayerClientError::CallRelayerContractError(err.to_string()))?;
        let urls = Arc::new(relayer_info.0);
        let names = Arc::new(relayer_info.1);
        let relayers = Arc::new(relayer_info.2);

        let mut registers: Vec<RegisterInfo> = Vec::new();
        let mut handlers = Vec::new();

        let self_arc = Arc::new(self);
        let request = Arc::new(request);

        for i in 0..urls.len() {
            let url = urls[i].clone();
            let name = names[i].clone();
            let relayer = relayers[i];
            let self_clone = Arc::clone(&self_arc);
            let request = Arc::clone(&request);

            let handler = tokio::spawn(async move { self_clone.register_info(&url, &name, &relayer, &request).await });
            handlers.push(handler);
        }

        let all_response = try_join_all(handlers).await.map_err(RelayerClientError::JoinError)?;

        for result in all_response {
            let response = result?;
            registers.push(response);
        }

        if let Some(options) = request.options.as_ref() {
            if !options.show_unavailable {
                registers.retain(|register| register.available);
            }
        }

        Ok(registers)
    }

    pub async fn relay_transact(&self, request: RelayTransactRequest) -> Result<RelayTransactResponse> {
        debug!("gas relayer send transact: {:?}", request);

        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        let mut request_builder = self
            .reqwest_client
            .post(format!("{}/{}", request.relayer_url, TRANSACT_URL_PATH));
        request_builder = self.build_request_builder(request_builder, None, &request.data);
        let response = self.post_data::<RelayTransactResponse>(request_builder).await?;

        Ok(response)
    }

    pub async fn relay_transaction_status(
        &self,
        request: RelayTransactStatusRequest,
    ) -> Result<RelayTransactStatusResponse> {
        debug!("relay transaction status {:?}", request);

        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        let response = self
            .get_data::<RelayTransactStatusResponse>(&format!(
                "{}/{}/{}",
                request.relayer_url, TRANSACTION_STATUS_URL_PATH, request.uuid
            ))
            .await?;
        Ok(response)
    }

    pub async fn wait_transaction(&self, request: WaitingTransactionRequest) -> Result<RelayTransactStatusResponse> {
        debug!("wait transaction {:?}", request);

        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        let start_time = Instant::now();
        loop {
            let response = self
                .relay_transaction_status(
                    RelayTransactStatusRequest::builder()
                        .relayer_url(request.relayer_url.clone())
                        .uuid(request.uuid.clone())
                        .build(),
                )
                .await?;

            match response.status {
                TransactStatus::Queued => {
                    if request.waiting_status == TransactStatus::Queued {
                        return Ok(response);
                    }
                }
                TransactStatus::Pending => {
                    if request.waiting_status == TransactStatus::Queued
                        || request.waiting_status == TransactStatus::Pending
                    {
                        return Ok(response);
                    }
                }
                TransactStatus::Succeeded => {
                    if request.waiting_status == TransactStatus::Queued
                        || request.waiting_status == TransactStatus::Pending
                        || request.waiting_status == TransactStatus::Succeeded
                    {
                        return Ok(response);
                    }
                }
                TransactStatus::Failed => {
                    return Err(RelayerClientError::TransactionFailed(
                        request.relayer_url,
                        request.uuid.clone(),
                    ));
                }
            }

            debug!(
                "uuid {:?} transaction status {:?}, keep waiting...",
                response.uuid, response.status
            );

            if start_time.elapsed() > request.timeout {
                return Err(RelayerClientError::WaitTransactionTimeout(request.uuid));
            }
            let interval = request.interval.unwrap_or(Duration::from_millis(500));
            sleep(interval).await;
        }
    }

    async fn handle_response<T>(&self, response: Response) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let response = response.error_for_status()?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok());
        if let Some(s) = content_type {
            if s.contains("application/json") {
                let parsed_resp = response.json::<ApiResponse<T>>().await?;
                let handled_resp = match parsed_resp.code {
                    0 => Ok(parsed_resp),
                    _ => Err(RelayerClientError::ApiResponseError {
                        code: parsed_resp.code,
                        message: parsed_resp.error.unwrap_or_default(),
                    }),
                };
                let res_body = handled_resp?;
                return Ok(res_body.result.unwrap());
            }
        }
        Err(RelayerClientError::UnsupportedContentTypeError(
            content_type.unwrap_or("").to_string(),
        ))
    }

    async fn get_data<T>(&self, url: &str) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let response = self
            .reqwest_client
            .get(url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .send()
            .await?;
        self.handle_response::<T>(response).await
    }

    async fn post_data<T>(&self, request_builder: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let response = request_builder.send().await?;
        self.handle_response::<T>(response).await
    }

    fn build_request_builder<T>(
        &self,
        mut request_builder: RequestBuilder,
        params: Option<HashMap<String, String>>,
        body: &T,
    ) -> RequestBuilder
    where
        T: Serialize,
    {
        match params {
            None => {}
            Some(params) => {
                for (key, value) in params.iter() {
                    request_builder = request_builder.query(&[(key, value)]);
                }
            }
        }
        request_builder = request_builder.json(body);
        request_builder
    }

    async fn register_info(
        &self,
        url: &str,
        name: &str,
        relayer: &Address,
        request: &RegisterInfoRequest,
    ) -> Result<RegisterInfo> {
        let mut request_builder = self.reqwest_client.post(format!("{}/{}", url, INFO_URL_PATH));
        request_builder = self.build_request_builder(request_builder, None, &request);
        let response = self.post_data::<RegisterInfoResponse>(request_builder).await?;

        let mut register_info = RegisterInfo {
            support: false,
            available: false,
            chain_id: 0,
            url: url.to_string(),
            name: name.to_string(),
            relayer_address: relayer.to_string(),
            relayer_contract_address: "".to_string(),
            contracts: vec![],
        };
        register_info.chain_id = response.chain_id;
        if response.support {
            register_info.support = response.support;
            register_info.available = response.available.unwrap_or(false);
            register_info.relayer_contract_address = response.relayer_contract_address.unwrap_or_default();
            register_info.contracts = response.contracts.unwrap_or_default();
        }

        Ok(register_info)
    }
}

async fn create_relayer_config(options: &RelayerClientOptions) -> Result<Arc<RelayerConfig>> {
    let result = if let Some(config_file_path) = &options.relayer_config_file_path {
        RelayerConfig::from_json_file(config_file_path).await
    } else {
        let mut remote_options = RemoteOptions::builder()
            .is_testnet(options.is_testnet)
            .is_staging(options.is_staging)
            .build();
        remote_options.base_url = options.relayer_config_remote_base_url.clone();
        remote_options.git_revision = options.relayer_config_git_revision.clone();
        RelayerConfig::from_remote(&remote_options).await
    };

    if let Ok(config) = result {
        Ok(Arc::new(config))
    } else {
        Err(RelayerClientError::CreateRelayerConfigError)
    }
}
