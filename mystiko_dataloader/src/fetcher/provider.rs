use crate::data::ChainResult;
use crate::data::ContractData;
use crate::data::ContractResult;
use crate::data::LoadedData;
use crate::data::{Data, DataType};
use crate::data::{FullData, LiteData};
use crate::fetcher::error::FetcherError;
use crate::fetcher::types::{DataFetcher, FetchOptions, FetchResult};
use crate::fetcher::FetcherLogOptions;
use anyhow::Result;
use async_trait::async_trait;
use ethers_contract::EthEvent;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, Filter, U64};
use ethers_providers::Middleware;
use ethers_providers::ProviderError;
use log::{error, info};
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::pool::Providers;
use mystiko_etherscan_client::log::{Log, LogMeta};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::u256_to_bytes;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Error, Debug)]
pub enum ProviderFetcherError {
    #[error("no chain config found for chain id: {0}")]
    ChainConfigNotFoundError(u64),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Event<R> {
    pub raw: R,
    pub metadata: LogMeta,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CommitmentDataEvent {
    pub crosschain_events: Vec<Event<CommitmentCrossChainFilter>>,
    pub queued_events: Vec<Event<CommitmentQueuedFilter>>,
    pub included_events: Vec<Event<CommitmentIncludedFilter>>,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProviderFetcher<R: LoadedData, P = Box<dyn Providers>> {
    providers: RwLock<P>,
    #[builder(default = Some(1), setter(strip_option))]
    concurrent_nums: Option<u32>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ProviderContractFetchOptions {
    pub chain_id: u64,
    pub contract_address: String,
    pub start_block: u64,
    pub actual_target_block: u64,
    pub provider: Arc<Provider>,
}

#[async_trait]
impl<R, P> DataFetcher<R> for ProviderFetcher<R, P>
where
    R: LoadedData,
    P: Providers,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let provider = if self.providers.read().await.get_provider(option.chain_id).is_some() {
            self.providers.read().await.get_provider(option.chain_id).unwrap()
        } else {
            self.providers
                .write()
                .await
                .get_or_create_provider(option.chain_id)
                .await?
        };
        let fetch_options = to_options(
            option,
            get_block_num(Arc::clone(&provider))
                .await
                .map_err(FetcherError::AnyhowError)?
                .as_u64(),
            Arc::clone(&provider),
        )?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(
                fetch_contracts::<R>(fetch_options, self.concurrent_nums.unwrap_or(1) as usize)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
            )
            .build())
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
    option.contract_options.as_ref().map_or_else(
        || {
            let chain_config = option
                .config
                .find_chain(option.chain_id)
                .ok_or_else(|| ProviderFetcherError::ChainConfigNotFoundError(option.chain_id))?;
            Ok(chain_config
                .contracts_with_disabled()
                .into_iter()
                .map(|contract_config| {
                    ProviderContractFetchOptions::builder()
                        .chain_id(option.chain_id)
                        .contract_address(contract_config.address().to_string())
                        .start_block(option.start_block)
                        .actual_target_block(if option.target_block > current_block_num {
                            current_block_num
                        } else {
                            option.target_block
                        })
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
                    ProviderContractFetchOptions::builder()
                        .chain_id(option.chain_id)
                        .contract_address(contract_option.contract_config.address().to_string())
                        .start_block(contract_option.start_block.unwrap_or(option.start_block))
                        .actual_target_block(if target_block > current_block_num {
                            current_block_num
                        } else {
                            target_block
                        })
                        .provider(Arc::clone(&provider))
                        .build()
                })
                .collect::<Vec<ProviderContractFetchOptions>>())
        },
    )
}

async fn fetch_contracts<R: LoadedData>(
    options: Vec<ProviderContractFetchOptions>,
    concurrent_nums: usize,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let chunk_nums = (options.len() + concurrent_nums - 1) / concurrent_nums;
    let chunks = options.chunks(chunk_nums);
    let mut group_tasks = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        group_tasks.push(group_fetch_contracts::<R>(chunk.to_vec()))
    }
    let group_results = futures::future::try_join_all(group_tasks).await?;
    Ok(group_results
        .into_iter()
        .flatten()
        .collect::<Vec<ContractResult<ContractData<R>>>>())
}

async fn group_fetch_contracts<R: LoadedData>(
    options: Vec<ProviderContractFetchOptions>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut group_result = Vec::with_capacity(options.len());
    for option in options {
        let contract_result = fetch_contract(option).await?;
        group_result.push(contract_result);
    }
    Ok(group_result)
}

async fn fetch_contract<R: LoadedData>(
    option: ProviderContractFetchOptions,
) -> Result<ContractResult<ContractData<R>>> {
    Ok(ContractResult::builder()
        .address(option.contract_address.to_string())
        .result(fetch_contract_result(&option).await)
        .build())
}

async fn fetch_contract_result<R: LoadedData>(option: &ProviderContractFetchOptions) -> Result<ContractData<R>> {
    let commitments = fetch_commitments(option).await?;
    let data = match R::data_type() {
        DataType::Full => {
            let fulldata = FullData::builder()
                .commitments(commitments)
                .nullifiers(fetch_nullifiers(option).await?)
                .build();
            info!(
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
            info!(
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

async fn fetch_commitments(option: &ProviderContractFetchOptions) -> Result<Vec<Commitment>> {
    build_commitments(CommitmentDataEvent {
        crosschain_events: fetch_logs::<CommitmentCrossChainFilter>(option).await?,
        queued_events: fetch_logs::<CommitmentQueuedFilter>(option).await?,
        included_events: fetch_logs::<CommitmentIncludedFilter>(option).await?,
    })
}

async fn fetch_nullifiers(option: &ProviderContractFetchOptions) -> Result<Vec<Nullifier>> {
    build_nullifiers(fetch_logs::<CommitmentSpentFilter>(option).await?)
}

async fn fetch_logs<E: EthEvent>(option: &ProviderContractFetchOptions) -> Result<Vec<Event<E>>> {
    let mut events: Vec<Event<E>> = vec![];
    let filter = Filter::new()
        .topic0(E::signature())
        .address(option.contract_address.parse::<Address>().unwrap())
        .from_block(BlockNumber::Number(U64::from(option.start_block)))
        .to_block(BlockNumber::Number(U64::from(option.actual_target_block)));
    let logs = option
        .provider
        .get_logs(&filter)
        .await
        .map_err(ProviderFetcherError::ProviderError)?;
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
    Ok(events)
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
