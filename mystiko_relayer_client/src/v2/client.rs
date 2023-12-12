use crate::error::RelayerClientError;
use crate::request::{build_request_builder, get_data, post_data};
use crate::types::register::RegisterInfo;
use crate::types::result;
use crate::RelayerClient;
use async_trait::async_trait;
use ethers_core::types::Address;
use futures::future::try_join_all;
use log::debug;
use mystiko_ethers::Providers;
use mystiko_protos::relayer::v1::RelayerClientOptions;
use mystiko_relayer_abi::mystiko_gas_relayer::MystikoGasRelayer;
use mystiko_relayer_config::wrapper::relayer::{RelayerConfig, RemoteOptions};
use mystiko_relayer_types::{
    HandshakeResponse, RegisterInfoRequest, RegisterInfoResponse, RelayTransactRequest, RelayTransactResponse,
    RelayTransactStatusRequest, RelayTransactStatusResponse, TransactStatus, WaitingTransactionRequest,
};
use mystiko_types::NetworkType;
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use validator::Validate;

pub const SUPPORTED_API_VERSION: &str = "v2";
pub const HANDSHAKE_URL_PATH: &str = "handshake";
pub const INFO_URL_PATH: &str = "api/v2/info";
pub const TRANSACT_URL_PATH: &str = "api/v2/transact";
pub const TRANSACTION_STATUS_URL_PATH: &str = "api/v2/transaction/status";

const DEFAULT_TIME_OUT_MS: u64 = 60000;

pub struct RelayerClientV2<P: Providers = Box<dyn Providers>> {
    pub reqwest_client: Arc<Client>,
    pub network_type: NetworkType,
    pub relayer_config: Arc<RelayerConfig>,
    pub providers: Arc<P>,
}

impl<P> RelayerClientV2<P>
where
    P: 'static + Providers,
{
    pub async fn new<O: Into<RelayerClientOptions>>(providers: Arc<P>, options: O) -> result::Result<Self> {
        let options: RelayerClientOptions = options.into();
        let relayer_config = create_relayer_config(&options).await?;
        let network_type = options.is_testnet.map_or(NetworkType::Testnet, |is_testnet| {
            if is_testnet {
                NetworkType::Testnet
            } else {
                NetworkType::Mainnet
            }
        });
        let timeout = options
            .timeout_ms
            .map_or(Duration::from_millis(DEFAULT_TIME_OUT_MS), |timeout_ms| {
                Duration::from_millis(timeout_ms)
            });
        let reqwest_client = Arc::new(
            Client::builder()
                .timeout(timeout)
                .build()
                .map_err(RelayerClientError::ReqwestError)?,
        );
        Ok(Self {
            reqwest_client,
            network_type,
            relayer_config,
            providers,
        })
    }
}

#[async_trait]
impl<P> RelayerClient for RelayerClientV2<P>
where
    P: 'static + Providers,
{
    type Error = RelayerClientError;

    async fn register_info(&self, request: RegisterInfoRequest) -> Result<Vec<RegisterInfo>, Self::Error> {
        let chain_id = request.chain_id;

        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        // get provider by chain id
        let provider = self
            .providers
            .get_provider(chain_id)
            .await
            .map_err(|err| RelayerClientError::GetOrCreateProviderError(err.to_string()))?;

        // found chain config
        if let Some(chain_config) = self.relayer_config.find_chain_config(chain_id) {
            let contract_address = ethers_address_from_string(chain_config.relayer_contract_address())
                .map_err(RelayerClientError::FromHexError)?;
            let contract = MystikoGasRelayer::new(contract_address, provider.clone());
            let relayer_info = contract
                .get_all_relayer_info()
                .await
                .map_err(|err| RelayerClientError::CallRelayerContractError(err.to_string()))?;

            let urls = Arc::new(relayer_info.0);
            let names = Arc::new(relayer_info.1);
            let relayers = Arc::new(relayer_info.2);

            let request = Arc::new(request);
            let mut registers: Vec<RegisterInfo> = Vec::new();
            let mut handlers = Vec::new();

            if let Some(name) = &request.name {
                if let Some(index) = names.iter().position(|x| x == name) {
                    let url = urls[index].clone();
                    let name = names[index].clone();
                    let relayer = relayers[index];
                    let request = Arc::clone(&request);
                    let reqwest_client = self.reqwest_client.clone();
                    if self.handshake(&url).await? {
                        registers.push(get_register_info(reqwest_client, &url, &name, &relayer, &request).await?);
                    }
                } else {
                    return Err(RelayerClientError::RelayerNameNotFoundError(name.clone()));
                }
            } else {
                for i in 0..urls.len() {
                    let url = urls[i].clone();
                    let name = names[i].clone();
                    let relayer = relayers[i];
                    let reqwest_client = self.reqwest_client.clone();
                    let request = Arc::clone(&request);
                    if self.handshake(&url).await? {
                        let handler = tokio::spawn(async move {
                            get_register_info(reqwest_client, &url, &name, &relayer, &request).await
                        });
                        handlers.push(handler);
                    }
                }

                let all_response = try_join_all(handlers).await.map_err(RelayerClientError::JoinError)?;

                registers.extend(all_response.into_iter().filter_map(Result::ok));
            }

            if let Some(options) = request.options.as_ref() {
                if !options.show_unavailable {
                    registers.retain(|register| register.available);
                }
            }

            Ok(registers)
        } else {
            Err(RelayerClientError::RelayerConfigNotFoundError(chain_id))
        }
    }

    async fn relay_transact(&self, request: RelayTransactRequest) -> Result<RelayTransactResponse, Self::Error> {
        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        if !self.handshake(&request.relayer_url).await? {
            return Err(RelayerClientError::UnsupportedApiVersion(
                SUPPORTED_API_VERSION.to_string(),
            ));
        }

        let mut request_builder = self
            .reqwest_client
            .post(format!("{}/{}", request.relayer_url, TRANSACT_URL_PATH));
        request_builder = build_request_builder(request_builder, None, &request.data);
        let response = post_data::<RelayTransactResponse>(request_builder).await?;

        Ok(response)
    }

    async fn relay_transaction_status(
        &self,
        request: RelayTransactStatusRequest,
    ) -> Result<RelayTransactStatusResponse, Self::Error> {
        // validate request
        request.validate().map_err(RelayerClientError::ValidationErrors)?;

        if !self.handshake(&request.relayer_url).await? {
            return Err(RelayerClientError::UnsupportedApiVersion(
                SUPPORTED_API_VERSION.to_string(),
            ));
        }

        let response = get_data::<RelayTransactStatusResponse>(
            self.reqwest_client.clone(),
            &format!(
                "{}/{}/{}",
                request.relayer_url, TRANSACTION_STATUS_URL_PATH, request.uuid
            ),
        )
        .await?;
        Ok(response)
    }

    async fn wait_transaction(
        &self,
        request: WaitingTransactionRequest,
    ) -> Result<RelayTransactStatusResponse, Self::Error> {
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

            let timeout = request.timeout.unwrap_or(Duration::from_secs(120));
            let interval = request.interval.unwrap_or(Duration::from_millis(500));

            if start_time.elapsed() > timeout {
                return Err(RelayerClientError::WaitTransactionTimeout(request.uuid));
            }
            sleep(interval).await;
        }
    }

    async fn handshake(&self, url: &str) -> Result<bool, Self::Error> {
        let response =
            get_data::<HandshakeResponse>(self.reqwest_client.clone(), &format!("{}/{}", url, HANDSHAKE_URL_PATH))
                .await?;
        Ok(response
            .api_version
            .iter()
            .any(|version| version.to_lowercase() == SUPPORTED_API_VERSION.to_lowercase()))
    }
}

async fn get_register_info(
    reqwest_client: Arc<Client>,
    url: &str,
    name: &str,
    relayer: &Address,
    request: &RegisterInfoRequest,
) -> result::Result<RegisterInfo> {
    let mut request_builder = reqwest_client.post(format!("{}/{}", url, INFO_URL_PATH));
    request_builder = build_request_builder(request_builder, None, &request);
    let response = post_data::<RegisterInfoResponse>(request_builder).await?;

    let mut register_info = RegisterInfo {
        support: false,
        available: false,
        chain_id: 0,
        url: url.to_string(),
        name: name.to_string(),
        relayer_address: ethers_address_to_string(relayer),
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

async fn create_relayer_config(options: &RelayerClientOptions) -> result::Result<Arc<RelayerConfig>> {
    #[cfg(feature = "fs")]
    let result = if let Some(config_file_path) = &options.relayer_config_file_path {
        RelayerConfig::from_json_file(config_file_path).await
    } else {
        create_relayer_config_from_remote(options).await
    };
    #[cfg(not(feature = "fs"))]
    let result = create_relayer_config_from_remote(options).await;

    match result {
        Ok(config) => Ok(Arc::new(config)),
        Err(err) => Err(RelayerClientError::CreateRelayerConfigError(err.to_string())),
    }
}

async fn create_relayer_config_from_remote(options: &RelayerClientOptions) -> anyhow::Result<RelayerConfig> {
    let is_testnet = options.is_testnet.unwrap_or(false);
    let is_staging = options.is_staging.unwrap_or(false);
    let mut remote_options = RemoteOptions::builder()
        .is_testnet(is_testnet)
        .is_staging(is_staging)
        .build();
    remote_options.base_url = options.relayer_config_remote_base_url.clone();
    remote_options.git_revision = options.relayer_config_git_revision.clone();
    RelayerConfig::from_remote(&remote_options).await
}
