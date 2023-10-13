use crate::data::{ChainResult, ContractData, ContractResult, Data, DataType, FullData, LiteData, LoadedData};
use crate::fetcher::{ChainLoadedBlockOptions, DataFetcher, FetchOptions, FetchResult, FetcherError};
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_config::MystikoConfig;
use mystiko_etherscan_client::{EtherScanClient, EtherScanClientOptions, Event, GetLogsOptions};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::loader::v1::EtherscanFetcherConfig;
use mystiko_types::{BridgeType, ContractType};
use mystiko_utils::convert::u256_to_bytes;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Error, Debug)]
pub enum EtherscanFetcherError {
    #[error("etherscan fetcher no chain config found for chain id: {0}")]
    UnsupportedChainError(u64),
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EtherscanFetcher<R> {
    etherscan_clients: Vec<Arc<EtherScanClient>>,
    #[builder(default = Some(1))]
    concurrency: Option<u32>,
    #[builder(default)]
    chain_delay_num_blocks: HashMap<u64, u64>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct EtherscanFetchOptions {
    pub(crate) address: String,
    pub(crate) from_block: u64,
    pub(crate) to_block: u64,
    pub(crate) disabled_at: bool,
    pub(crate) contract_type: ContractType,
    pub(crate) bridge_type: BridgeType,
}

#[async_trait]
impl<R> DataFetcher<R> for EtherscanFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        info!(
            "chain={} start fetch data, start_block={}, target_block={}",
            option.chain_id, option.start_block, option.target_block
        );
        let (current_block_num, client) = self
            .chain_safe_current_block(option.chain_id, option.config.clone())
            .await?;
        let options: Vec<EtherscanFetchOptions> =
            to_options(option, current_block_num).map_err(FetcherError::AnyhowError)?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(
                group_fetch::<R>(&client, options, self.concurrency.unwrap_or(1) as usize)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
            )
            .build())
    }

    async fn chain_loaded_block(&self, options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        let (current_safe_block, _) = self
            .chain_safe_current_block(options.chain_id, options.config.clone())
            .await?;
        Ok(current_safe_block)
    }
}

impl<R> EtherscanFetcher<R>
where
    R: LoadedData,
{
    async fn chain_safe_current_block(
        &self,
        chain_id: u64,
        config: Arc<MystikoConfig>,
    ) -> Result<(u64, Arc<EtherScanClient>), FetcherError> {
        let client = get_etherscan_client(chain_id, &self.etherscan_clients).map_err(FetcherError::AnyhowError)?;
        let delay_num_blocks = if let Some(delay_blocks) = self.chain_delay_num_blocks.get(&chain_id) {
            *delay_blocks
        } else {
            config
                .find_chain(chain_id)
                .map(|c| c.event_delay_blocks())
                .unwrap_or_default()
        };
        let current_block = client
            .get_block_number()
            .await
            .map_err(|err| FetcherError::AnyhowError(err.into()))?
            .as_u64();
        let current_safe_block = if current_block >= delay_num_blocks {
            current_block - delay_num_blocks
        } else {
            0
        };
        Ok((current_safe_block, client))
    }
}

impl<R> EtherscanFetcher<R> {
    pub fn from_config<C: Into<EtherscanFetcherConfig>>(
        loader_chain_id: u64,
        mystiko_config: Arc<MystikoConfig>,
        config: C,
    ) -> Result<Self> {
        let mut config = config.into();
        if config.chains.is_empty() {
            config.chains.insert(loader_chain_id, Default::default());
        }

        let mut clients = Vec::new();

        for (chain_id, fetcher_chain_cfg) in config.chains.iter() {
            let api_key = fetcher_chain_cfg.api_key.clone().unwrap_or_default();
            let chain_config = mystiko_config
                .find_chain(*chain_id)
                .ok_or(EtherscanFetcherError::UnsupportedChainError(*chain_id))?;
            let url = fetcher_chain_cfg
                .url
                .clone()
                .unwrap_or(chain_config.explorer_api_url().to_string());
            let option = EtherScanClientOptions::builder()
                .chain_id(*chain_id)
                .base_url(url)
                .url_prefix(fetcher_chain_cfg.url_prefix.clone())
                .api_key(api_key.clone())
                .max_requests_per_second(fetcher_chain_cfg.max_requests_per_second)
                .offset(fetcher_chain_cfg.page_size)
                .build();

            let client = EtherScanClient::new(option)?;
            clients.push(Arc::new(client));
        }

        let delay_blocks: HashMap<_, _> = config
            .chains
            .iter()
            .filter_map(|(k, v)| v.delay_num_blocks.map(|delay| (*k, delay)))
            .collect();

        Ok(EtherscanFetcher::builder()
            .concurrency(config.concurrency)
            .chain_delay_num_blocks(delay_blocks)
            .etherscan_clients(clients)
            .build())
    }
}

impl<R> From<Arc<EtherScanClient>> for EtherscanFetcher<R> {
    fn from(client: Arc<EtherScanClient>) -> Self {
        vec![client].into()
    }
}

impl<R> From<Vec<Arc<EtherScanClient>>> for EtherscanFetcher<R> {
    fn from(clients: Vec<Arc<EtherScanClient>>) -> Self {
        Self::builder().etherscan_clients(clients).build()
    }
}

fn get_etherscan_client(chain_id: u64, etherscan_clients: &[Arc<EtherScanClient>]) -> Result<Arc<EtherScanClient>> {
    for client in etherscan_clients {
        if client.chain_id == chain_id {
            return Ok(client.clone());
        }
    }
    Err(EtherscanFetcherError::UnsupportedChainError(chain_id).into())
}

fn to_options(option: &FetchOptions, current_block_num: u64) -> Result<Vec<EtherscanFetchOptions>> {
    option.contract_options.as_ref().map_or_else(
        || {
            let chain_config = option
                .config
                .find_chain(option.chain_id)
                .ok_or(EtherscanFetcherError::UnsupportedChainError(option.chain_id))?;
            Ok(chain_config
                .contracts()
                .into_iter()
                .map(|contract_config| {
                    EtherscanFetchOptions::builder()
                        .address(contract_config.address().to_string())
                        .from_block(option.start_block)
                        .to_block(option.target_block.min(current_block_num))
                        .disabled_at(option.start_block > contract_config.disabled_at().unwrap_or(option.start_block))
                        .contract_type(contract_config.contract_type().clone())
                        .bridge_type(contract_config.bridge_type().clone())
                        .build()
                })
                .collect::<Vec<EtherscanFetchOptions>>())
        },
        |contract_options| {
            Ok(contract_options
                .iter()
                .map(|contract_option| {
                    let start_block = contract_option.start_block.unwrap_or(option.start_block);
                    let target_block = contract_option.target_block.unwrap_or(option.target_block);
                    EtherscanFetchOptions::builder()
                        .address(contract_option.contract_config.address().to_string())
                        .from_block(start_block)
                        .to_block(target_block.min(current_block_num))
                        .disabled_at(start_block > contract_option.contract_config.disabled_at().unwrap_or(start_block))
                        .contract_type(contract_option.contract_config.contract_type().clone())
                        .bridge_type(contract_option.contract_config.bridge_type().clone())
                        .build()
                })
                .collect::<Vec<EtherscanFetchOptions>>())
        },
    )
}

async fn group_fetch<R: LoadedData>(
    client: &Arc<EtherScanClient>,
    options: Vec<EtherscanFetchOptions>,
    concurrency: usize,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let concurrency = if concurrency == 0 { 1 } else { concurrency };
    let chunk_nums = (options.len() + concurrency - 1) / concurrency;
    let chunks = options.chunks(chunk_nums);
    let mut group_tasks = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        group_tasks.push(group_fetch_contracts::<R>(client, chunk.to_vec()));
    }
    let group_results = futures::future::try_join_all(group_tasks).await?;
    Ok(group_results
        .into_iter()
        .flatten()
        .collect::<Vec<ContractResult<ContractData<R>>>>())
}

async fn group_fetch_contracts<R: LoadedData>(
    client: &Arc<EtherScanClient>,
    options: Vec<EtherscanFetchOptions>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut group_result = Vec::with_capacity(options.len());
    for option in options {
        let contract_result = ContractResult::builder()
            .address(option.address.to_string())
            .result(fetch_contract_data(client, option).await)
            .build();
        group_result.push(contract_result);
    }
    Ok(group_result)
}

async fn fetch_contract_data<R: LoadedData>(
    client: &Arc<EtherScanClient>,
    option: EtherscanFetchOptions,
) -> Result<ContractData<R>> {
    let commitments: Vec<Commitment> = if option.disabled_at {
        Vec::new()
    } else {
        fetch_commitments(client, &option).await?
    };
    let contracts_response = match R::data_type() {
        DataType::Lite => {
            info!(
                "contract={} fetch {} commitments from {} to {}",
                option.address,
                &commitments.len(),
                option.from_block,
                option.to_block,
            );
            let lite_data = LiteData::builder().commitments(commitments).build();
            ContractData::builder()
                .address(&option.address)
                .start_block(option.from_block)
                .end_block(option.to_block)
                .data(R::from_data(Data::Lite(lite_data)))
                .build()
        }
        DataType::Full => {
            let nullifiers = fetch_nullifiers(client, &option).await?;
            info!(
                "contract={} fetch {} commitments and {} nullifiers from {} to {}",
                option.address,
                &commitments.len(),
                &nullifiers.len(),
                option.from_block,
                option.to_block,
            );
            let full_data = FullData::builder()
                .commitments(commitments)
                .nullifiers(nullifiers)
                .build();
            ContractData::builder()
                .address(&option.address)
                .start_block(option.from_block)
                .end_block(option.to_block)
                .data(R::from_data(Data::Full(full_data)))
                .build()
        }
    };
    Ok(contracts_response)
}

async fn fetch_commitments(client: &EtherScanClient, option: &EtherscanFetchOptions) -> Result<Vec<Commitment>> {
    let get_logs_option = GetLogsOptions::builder()
        .address(option.address.to_string())
        .from_block(option.from_block)
        .to_block(option.to_block)
        .build();
    let mut commitments: Vec<Commitment> = Vec::new();
    match (&option.contract_type, &option.bridge_type) {
        (ContractType::Pool, _) => {
            client
                .fetch_event_logs::<CommitmentQueuedFilter>(get_logs_option.clone())
                .await?
                .iter()
                .for_each(|event: &Event<CommitmentQueuedFilter>| {
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
            client
                .fetch_event_logs::<CommitmentIncludedFilter>(get_logs_option.clone())
                .await?
                .iter()
                .for_each(|event| {
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
        }
        (ContractType::Deposit, BridgeType::Loop) => {
            return Ok(commitments);
        }
        (ContractType::Deposit, _) => {
            client
                .fetch_event_logs::<CommitmentCrossChainFilter>(get_logs_option.clone())
                .await?
                .iter()
                .for_each(|event: &Event<CommitmentCrossChainFilter>| {
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
        }
    }
    commitments.sort_by_key(|commitment| commitment.block_number);
    Ok(commitments)
}

async fn fetch_nullifiers(client: &EtherScanClient, option: &EtherscanFetchOptions) -> Result<Vec<Nullifier>> {
    let mut nullifiers: Vec<Nullifier> = Vec::new();
    if option.contract_type == ContractType::Deposit {
        return Ok(nullifiers);
    }
    let get_logs_option = GetLogsOptions::builder()
        .address(option.address.to_string())
        .from_block(option.from_block)
        .to_block(option.to_block)
        .build();
    client
        .fetch_event_logs::<CommitmentSpentFilter>(get_logs_option.clone())
        .await?
        .iter()
        .for_each(|event| {
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
