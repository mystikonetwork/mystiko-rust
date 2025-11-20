use crate::data::ChainResult;
use crate::data::ContractData;
use crate::data::ContractResult;
use crate::data::LoadedData;
use crate::data::{Data, DataType};
use crate::data::{FullData, LiteData};
use crate::fetcher::error::FetcherError;
use crate::fetcher::types::{DataFetcher, FetchOptions, FetchResult};
use crate::fetcher::{ChainLoadedBlockOptions, FetcherLogOptions};
use anyhow::Result;
use async_trait::async_trait;
use ethers_contract::EthEvent;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, Filter, U64};
use ethers_providers::Middleware;
use ethers_providers::ProviderError;
use ethers_providers::Quorum;
use log::info;
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_config::MystikoConfig;
use mystiko_ethers::{
    ChainProvidersOptions, Provider, ProviderOptions, ProviderPool, Providers, ProvidersOptions, QuorumProviderOptions,
};
use mystiko_etherscan_client::{Log, LogMeta};
use mystiko_protos::common::v1::ProviderType;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::loader::v1::ProviderFetcherConfig;
use mystiko_types::{BridgeType, ContractType};
use mystiko_utils::convert::u256_to_bytes;
use rustc_hex::FromHexError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::time::{sleep, Instant};
use typed_builder::TypedBuilder;

pub const PROVIDER_FETCHER_NAME: &str = "provider";

#[derive(Error, Debug)]
pub enum ProviderFetcherError {
    #[error("provider type error: {0}")]
    ProviderTypeError(i32),
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
    #[error("unsupported chain id: {0}")]
    UnsupportedChainError(u64),
}

pub const DEFAULT_MAX_REQUESTS_PER_SECOND: u32 = 5;
pub const DEFAULT_MAX_RETRY_TIMES: u32 = 20;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProviderRetryConfig {
    #[builder(default = DEFAULT_MAX_REQUESTS_PER_SECOND)]
    max_requests_per_second: u32,
    #[builder(default = DEFAULT_MAX_RETRY_TIMES)]
    max_retry_times: u32,
}

impl Default for ProviderRetryConfig {
    fn default() -> Self {
        Self::builder()
            .max_requests_per_second(DEFAULT_MAX_REQUESTS_PER_SECOND)
            .max_retry_times(DEFAULT_MAX_RETRY_TIMES)
            .build()
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProviderFetcher<R: LoadedData, P = Box<dyn Providers>> {
    providers: Arc<P>,
    #[builder(default = Some(1))]
    concurrency: Option<u32>,
    #[builder(default)]
    chain_delay_num_blocks: HashMap<u64, u64>,
    #[builder(default)]
    retry_config: ProviderRetryConfig,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FetcherChainProvidersOptions {
    fetcher_config: ProviderFetcherConfig,
    mystiko_config: Arc<MystikoConfig>,
}

#[derive(Debug, Clone, TypedBuilder)]
struct Event<R> {
    pub(crate) raw: R,
    pub(crate) metadata: LogMeta,
}

#[derive(Debug, Clone, TypedBuilder)]
struct CommitmentDataEvent {
    pub(crate) crosschain_events: Vec<Event<CommitmentCrossChainFilter>>,
    pub(crate) queued_events: Vec<Event<CommitmentQueuedFilter>>,
    pub(crate) included_events: Vec<Event<CommitmentIncludedFilter>>,
}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct ProviderContractFetchOptions {
    pub(crate) chain_id: u64,
    pub(crate) contract_address: String,
    pub(crate) contract_type: ContractType,
    pub(crate) bridge_type: BridgeType,
    pub(crate) start_block: u64,
    pub(crate) actual_target_block: u64,
    pub(crate) disabled_at: bool,
    pub(crate) event_filter_size: u64,
    pub(crate) provider: Arc<Provider>,
}

#[async_trait]
impl<R, P> DataFetcher<R> for ProviderFetcher<R, P>
where
    R: LoadedData,
    P: Providers,
{
    fn name(&self) -> &'static str {
        PROVIDER_FETCHER_NAME
    }

    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let (current_block, provider) = self
            .chain_safe_current_block(option.chain_id, option.config.clone())
            .await?;
        let fetch_options = to_options(option, current_block, provider.clone())?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(
                fetch_contracts::<R>(
                    fetch_options,
                    &self.retry_config,
                    self.concurrency.unwrap_or(1) as usize,
                )
                .await
                .map_err(FetcherError::AnyhowError)?,
            )
            .build())
    }

    async fn chain_loaded_block(&self, options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        let (current_block, _) = self
            .chain_safe_current_block(options.chain_id, options.config.clone())
            .await?;
        Ok(current_block)
    }
}

impl<R, P> ProviderFetcher<R, P>
where
    R: LoadedData,
    P: Providers,
{
    async fn chain_safe_current_block(
        &self,
        chain_id: u64,
        config: Arc<MystikoConfig>,
    ) -> Result<(u64, Arc<Provider>), FetcherError> {
        let provider = self
            .providers
            .get_provider(chain_id)
            .await
            .map_err(FetcherError::AnyhowError)?;
        let delay_num_blocks = if let Some(delay_blocks) = self.chain_delay_num_blocks.get(&chain_id) {
            *delay_blocks
        } else {
            config
                .find_chain(chain_id)
                .map(|c| c.event_delay_blocks())
                .unwrap_or_default()
        };
        let current_block = get_block_num(provider.clone())
            .await
            .map_err(FetcherError::AnyhowError)?
            .as_u64();
        let current_safe_block = current_block.saturating_sub(delay_num_blocks);
        Ok((current_safe_block, provider))
    }
}

impl<R, P> From<Arc<P>> for ProviderFetcher<R, P>
where
    R: LoadedData,
    P: Providers,
{
    fn from(providers: Arc<P>) -> Self {
        Self::builder()
            .providers(providers)
            .retry_config(ProviderRetryConfig::default())
            .build()
    }
}

impl<R, P> ProviderFetcher<R, P>
where
    R: LoadedData,
    P: Providers,
{
    pub fn from_config<C: Into<ProviderFetcherConfig>>(config: C, providers: Arc<P>) -> Self {
        let config = config.into();
        let delay_blocks: HashMap<_, _> = config
            .chains
            .iter()
            .filter_map(|(k, v)| v.delay_num_blocks.map(|delay| (*k, delay)))
            .collect();
        // todo save filter size in map
        ProviderFetcher::builder()
            .providers(providers)
            .chain_delay_num_blocks(delay_blocks)
            .concurrency(config.concurrency)
            .retry_config(
                ProviderRetryConfig::builder()
                    .max_requests_per_second(
                        config
                            .max_requests_per_second
                            .unwrap_or(DEFAULT_MAX_REQUESTS_PER_SECOND),
                    )
                    .max_retry_times(config.max_retry_times.unwrap_or(DEFAULT_MAX_RETRY_TIMES))
                    .build(),
            )
            .build()
    }
}

pub fn create_provider_pool_from_config<C>(
    cfg: C,
    mystiko_config: Arc<MystikoConfig>,
) -> ProviderPool<FetcherChainProvidersOptions>
where
    C: Into<ProviderFetcherConfig>,
{
    let cfg = cfg.into();
    let cfg_option = FetcherChainProvidersOptions::builder()
        .fetcher_config(cfg)
        .mystiko_config(mystiko_config.clone())
        .build();
    let providers: ProviderPool<FetcherChainProvidersOptions> =
        ProviderPool::builder().chain_providers_options(cfg_option).build();
    providers
}

#[async_trait]
impl ChainProvidersOptions for FetcherChainProvidersOptions {
    async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>> {
        if let Some(chain_cfg) = self.mystiko_config.find_chain(chain_id) {
            let mut providers_options: Vec<ProviderOptions> = vec![];
            let mut provider_type = ProviderType::Quorum;

            if let Some(fetcher_cfg) = self.fetcher_config.chains.get(&chain_id) {
                if let Some(t) = fetcher_cfg.provider_type {
                    provider_type = ProviderType::from_i32(t).ok_or(ProviderFetcherError::ProviderTypeError(t))?;
                }

                for url in &fetcher_cfg.urls() {
                    let provider_options = match self.fetcher_config.timeout_ms {
                        None => ProviderOptions::builder().url(url.clone()).build(),
                        Some(t) => ProviderOptions::builder()
                            .url(url.clone())
                            .request_timeout(Duration::from_millis(t))
                            .build(),
                    };
                    providers_options.push(provider_options);
                }
            }

            if providers_options.is_empty() {
                for provider_config in chain_cfg.providers() {
                    let timeout_ms = match self.fetcher_config.timeout_ms {
                        None => provider_config.timeout_ms() as u64,
                        Some(t) => t,
                    };
                    let provider_options = ProviderOptions::builder()
                        .url(provider_config.url().to_string())
                        .quorum_weight(provider_config.quorum_weight() as u64)
                        .timeout_retries(provider_config.max_try_count() - 1)
                        .rate_limit_retries(provider_config.max_try_count() - 1)
                        .request_timeout(Duration::from_millis(timeout_ms))
                        .build();
                    providers_options.push(provider_options);
                }
            }

            match provider_type {
                ProviderType::Unspecified => Err(ProviderFetcherError::ProviderTypeError(
                    ProviderType::Unspecified as i32,
                ))?,
                ProviderType::Failover => Ok(Some(ProvidersOptions::Failover(providers_options))),
                ProviderType::Quorum => {
                    let quorum_options = QuorumProviderOptions::builder()
                        .quorum(Quorum::Percentage(chain_cfg.provider_quorum_percentage()))
                        .build();
                    Ok(Some(ProvidersOptions::Quorum(providers_options, quorum_options)))
                }
            }
        } else {
            Ok(None)
        }
    }
}

async fn get_block_num(provider: Arc<Provider>) -> Result<U64> {
    Ok(provider
        .get_block_number()
        .await
        .map_err(ProviderFetcherError::ProviderError)?)
}

fn to_options(
    option: &FetchOptions,
    current_block_num: u64,
    provider: Arc<Provider>,
) -> Result<Vec<ProviderContractFetchOptions>> {
    let chain_config = option
        .config
        .find_chain(option.chain_id)
        .ok_or_else(|| ProviderFetcherError::UnsupportedChainError(option.chain_id))?;
    option.contract_options.as_ref().map_or_else(
        || {
            Ok(chain_config
                .contracts()
                .into_iter()
                .map(|contract_config| {
                    ProviderContractFetchOptions::builder()
                        .chain_id(option.chain_id)
                        .contract_address(contract_config.address().to_string())
                        .contract_type(contract_config.contract_type().clone())
                        .bridge_type(contract_config.bridge_type().clone())
                        .start_block(option.start_block)
                        .actual_target_block(option.target_block.min(current_block_num))
                        .disabled_at(option.start_block > contract_config.disabled_at().unwrap_or(option.start_block))
                        .event_filter_size(chain_config.event_filter_size().max(1))
                        .provider(Arc::clone(&provider))
                        .build()
                })
                .collect::<Vec<ProviderContractFetchOptions>>())
        },
        |contract_options| {
            Ok(contract_options
                .iter()
                .map(|contract_option| {
                    let target_block = contract_option.target_block.unwrap_or(option.target_block);
                    let start_block = contract_option.start_block.unwrap_or(option.start_block);
                    ProviderContractFetchOptions::builder()
                        .chain_id(option.chain_id)
                        .contract_address(contract_option.contract_config.address().to_string())
                        .contract_type(contract_option.contract_config.contract_type().clone())
                        .bridge_type(contract_option.contract_config.bridge_type().clone())
                        .start_block(start_block)
                        .actual_target_block(target_block.min(current_block_num))
                        .disabled_at(start_block > contract_option.contract_config.disabled_at().unwrap_or(start_block))
                        .event_filter_size(chain_config.event_filter_size().max(1))
                        .provider(Arc::clone(&provider))
                        .build()
                })
                .collect::<Vec<ProviderContractFetchOptions>>())
        },
    )
}

async fn fetch_contracts<R: LoadedData>(
    options: Vec<ProviderContractFetchOptions>,
    retry_config: &ProviderRetryConfig,
    concurrency: usize,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let concurrency = if concurrency == 0 { 1 } else { concurrency };
    let chunk_nums = options.len().div_ceil(concurrency);
    let chunks = options.chunks(chunk_nums);
    let mut group_tasks = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        group_tasks.push(group_fetch_contracts::<R>(chunk.to_vec(), retry_config))
    }
    let group_results = futures::future::try_join_all(group_tasks).await?;
    Ok(group_results
        .into_iter()
        .flatten()
        .collect::<Vec<ContractResult<ContractData<R>>>>())
}

async fn group_fetch_contracts<R: LoadedData>(
    options: Vec<ProviderContractFetchOptions>,
    retry_config: &ProviderRetryConfig,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut group_result = Vec::with_capacity(options.len());
    for option in options {
        let contract_result = fetch_contract(option, retry_config).await?;
        group_result.push(contract_result);
    }
    Ok(group_result)
}

async fn fetch_contract<R: LoadedData>(
    option: ProviderContractFetchOptions,
    retry_config: &ProviderRetryConfig,
) -> Result<ContractResult<ContractData<R>>> {
    Ok(ContractResult::builder()
        .address(option.contract_address.to_string())
        .result(fetch_contract_result(&option, retry_config).await)
        .build())
}

async fn fetch_contract_result<R: LoadedData>(
    option: &ProviderContractFetchOptions,
    retry_config: &ProviderRetryConfig,
) -> Result<ContractData<R>> {
    let commitments = if option.disabled_at {
        Vec::new()
    } else {
        fetch_commitments(option, retry_config).await?
    };

    let data = match R::data_type() {
        DataType::Full => {
            let fulldata = FullData::builder()
                .commitments(commitments)
                .nullifiers(fetch_nullifiers(option, retry_config).await?)
                .build();
            log::info!(
                "{} fetch {} commitments and {} nullifiers",
                FetcherLogOptions::builder()
                    .address(option.contract_address.to_string())
                    .chain_id(option.chain_id)
                    .start_block(option.start_block)
                    .end_block(option.actual_target_block)
                    .data_type(DataType::Full)
                    .build()
                    .to_string(),
                fulldata.commitments.len(),
                fulldata.nullifiers.len()
            );
            R::from_data(Data::Full(fulldata))
        }
        DataType::Lite => {
            let litedata = LiteData::builder().commitments(commitments).build();
            log::info!(
                "{} fetch {} commitments",
                FetcherLogOptions::builder()
                    .address(option.contract_address.to_string())
                    .chain_id(option.chain_id)
                    .start_block(option.start_block)
                    .end_block(option.actual_target_block)
                    .data_type(DataType::Lite)
                    .build()
                    .to_string(),
                litedata.commitments.len()
            );
            R::from_data(Data::Lite(litedata))
        }
    };
    Ok(ContractData::builder()
        .address(option.contract_address.to_string())
        .start_block(option.start_block)
        .end_block(option.actual_target_block)
        .data(data)
        .build())
}

async fn fetch_commitments(
    option: &ProviderContractFetchOptions,
    retry_config: &ProviderRetryConfig,
) -> Result<Vec<Commitment>> {
    match (&option.contract_type, &option.bridge_type) {
        (ContractType::Pool, _) => build_commitments(CommitmentDataEvent {
            crosschain_events: vec![],
            queued_events: fetch_logs::<CommitmentQueuedFilter>(option, retry_config).await?,
            included_events: fetch_logs::<CommitmentIncludedFilter>(option, retry_config).await?,
        }),
        (ContractType::Deposit, BridgeType::Loop) => Ok(vec![]),
        (ContractType::Deposit, _) => build_commitments(CommitmentDataEvent {
            crosschain_events: fetch_logs::<CommitmentCrossChainFilter>(option, retry_config).await?,
            queued_events: vec![],
            included_events: vec![],
        }),
    }
}

async fn fetch_nullifiers(
    option: &ProviderContractFetchOptions,
    retry_config: &ProviderRetryConfig,
) -> Result<Vec<Nullifier>> {
    if option.contract_type == ContractType::Deposit {
        return Ok(vec![]);
    }
    build_nullifiers(fetch_logs::<CommitmentSpentFilter>(option, retry_config).await?)
}

async fn fetch_logs<E: EthEvent>(
    option: &ProviderContractFetchOptions,
    retry_config: &ProviderRetryConfig,
) -> Result<Vec<Event<E>>> {
    let mut events: Vec<Event<E>> = vec![];
    if option.start_block > option.actual_target_block {
        return Ok(events);
    }
    let address = option
        .contract_address
        .parse::<Address>()
        .map_err(ProviderFetcherError::FromHexError)?;

    let mut start_block = option.start_block;
    let mut to_block;
    let mut last_request_time = None;
    loop {
        last_request_time = throttle(last_request_time, retry_config.max_requests_per_second).await;
        to_block = option
            .actual_target_block
            .min(start_block + option.event_filter_size - 1);
        info!("start_block: {}, to_block: {}", start_block, to_block);
        let filter = Filter::new()
            .topic0(E::signature())
            .address(address)
            .from_block(BlockNumber::Number(U64::from(start_block)))
            .to_block(BlockNumber::Number(U64::from(to_block)));
        let logs = fetch_logs_with_retry(option, retry_config, filter).await?;
        for log in logs {
            let my_log = Log {
                address: log.address,
                topics: log.topics,
                data: log.data,
                block_hash: log.block_hash,
                block_number: log.block_number,
                transaction_hash: log.transaction_hash,
            };
            let metadata: LogMeta = (&my_log).into();
            let event = E::decode_log(&my_log.into_raw())?;
            events.push(Event { raw: event, metadata });
        }
        if to_block == option.actual_target_block {
            break;
        }
        start_block = to_block + 1;
    }
    Ok(events)
}

async fn fetch_logs_with_retry(
    option: &ProviderContractFetchOptions,
    config: &ProviderRetryConfig,
    filter: Filter,
) -> Result<Vec<ethers_core::types::Log>> {
    let mut current_retry_time = 1;
    loop {
        let ethers_logs = match option
            .provider
            .get_logs(&filter)
            .await
            .map_err(ProviderFetcherError::ProviderError)
        {
            Ok(logs) => logs,
            Err(err) => {
                if current_retry_time > config.max_retry_times {
                    return Err(err.into());
                }
                current_retry_time += 1;
                info!("error: {:?} retry fetch logs: {}", err, current_retry_time);
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
        return Ok(ethers_logs);
    }
}

async fn throttle(last_request_time: Option<Instant>, max_requests_per_second: u32) -> Option<Instant> {
    let now: Instant = Instant::now();
    if let Some(last_instant) = last_request_time.as_ref() {
        let elapsed = now.duration_since(*last_instant).as_millis();
        let wait_ms = 1000 / max_requests_per_second;
        if elapsed < wait_ms as u128 {
            info!("wait for: {}", wait_ms as u128 - elapsed);
            sleep(Duration::from_millis((wait_ms as u128 - elapsed) as u64)).await;
        }
    }
    Some(now)
}

fn build_commitments(events: CommitmentDataEvent) -> Result<Vec<Commitment>> {
    let CommitmentDataEvent {
        crosschain_events,
        queued_events,
        included_events,
    } = events;
    let mut commitments: Vec<Commitment> = Vec::new();
    crosschain_events.iter().for_each(|event| {
        let commitment = Commitment::builder()
            .commitment_hash(u256_to_bytes(&event.raw.commitment))
            .status(CommitmentStatus::SrcSucceeded)
            .block_number(event.metadata.block_number.as_u64())
            .included_block_number(None)
            .src_chain_block_number(event.metadata.block_number.as_u64())
            .leaf_index(None)
            .rollup_fee(None)
            .encrypted_note(None)
            .queued_transaction_hash(None)
            .included_transaction_hash(None)
            .src_chain_transaction_hash(event.metadata.transaction_hash.to_fixed_bytes().to_vec())
            .build();
        commitments.push(commitment);
    });
    queued_events.iter().for_each(|event| {
        let commitment = Commitment::builder()
            .commitment_hash(u256_to_bytes(&event.raw.commitment))
            .status(CommitmentStatus::Queued)
            .block_number(event.metadata.block_number.as_u64())
            .included_block_number(None)
            .src_chain_block_number(None)
            .leaf_index(event.raw.leaf_index.as_u64())
            .rollup_fee(u256_to_bytes(&event.raw.rollup_fee))
            .encrypted_note(event.raw.encrypted_note.to_vec())
            .queued_transaction_hash(event.metadata.transaction_hash.to_fixed_bytes().to_vec())
            .included_transaction_hash(None)
            .src_chain_transaction_hash(None)
            .build();
        commitments.push(commitment);
    });
    included_events.iter().for_each(|event| {
        let commitment = Commitment::builder()
            .commitment_hash(u256_to_bytes(&event.raw.commitment))
            .status(CommitmentStatus::Included)
            .block_number(event.metadata.block_number.as_u64())
            .included_block_number(event.metadata.block_number.as_u64())
            .src_chain_block_number(None)
            .leaf_index(None)
            .rollup_fee(None)
            .encrypted_note(None)
            .queued_transaction_hash(None)
            .included_transaction_hash(event.metadata.transaction_hash.to_fixed_bytes().to_vec())
            .src_chain_transaction_hash(None)
            .build();
        commitments.push(commitment);
    });
    commitments.sort_by_key(|commitment| commitment.block_number);
    Ok(commitments)
}

fn build_nullifiers(spent_events: Vec<Event<CommitmentSpentFilter>>) -> Result<Vec<Nullifier>> {
    let mut nullifiers: Vec<Nullifier> = Vec::new();
    spent_events.iter().for_each(|event| {
        let nullifier = Nullifier::builder()
            .transaction_hash(event.metadata.transaction_hash.as_bytes().to_vec())
            .block_number(event.metadata.block_number.as_u64())
            .nullifier(u256_to_bytes(&event.raw.serial_number))
            .build();
        nullifiers.push(nullifier);
    });
    nullifiers.sort_by_key(|nullifier| nullifier.block_number);
    Ok(nullifiers)
}
