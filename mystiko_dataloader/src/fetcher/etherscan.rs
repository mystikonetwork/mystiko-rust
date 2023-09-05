use crate::data::{ChainResult, ContractData, ContractResult, Data, DataType, FullData, LiteData, LoadedData};
use crate::fetcher::{DataFetcher, FetchOptions, FetchResult, FetcherError};
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_etherscan_client::{EtherScanClient, Event, GetLogsOptions};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::u256_to_bytes;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Error, Debug)]
pub enum EtherscanFetcherError {
    #[error("no chain config found for chain id: {0}")]
    UnsupportedChainError(u64),
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EtherscanFetcher<R> {
    etherscan_clients: Vec<Arc<EtherScanClient>>,
    #[builder(default = Some(1))]
    concurrency: Option<u32>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
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
        let client =
            get_etherscan_client(option.chain_id, &self.etherscan_clients).map_err(FetcherError::AnyhowError)?;
        let current_block_num: u64 = client
            .get_block_number()
            .await
            .map_err(|err| FetcherError::AnyhowError(err.into()))?
            .as_u64();
        let options: Vec<GetLogsOptions> = to_options(option, current_block_num).map_err(FetcherError::AnyhowError)?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(
                group_fetch::<R>(&client, options, self.concurrency.unwrap_or(1) as usize)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
            )
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

fn to_options(option: &FetchOptions, current_block_num: u64) -> Result<Vec<GetLogsOptions>> {
    option.contract_options.as_ref().map_or_else(
        || {
            let chain_config = option
                .config
                .find_chain(option.chain_id)
                .ok_or(EtherscanFetcherError::UnsupportedChainError(option.chain_id))?;
            Ok(chain_config
                .contracts_with_disabled()
                .into_iter()
                .map(|contract_config| {
                    GetLogsOptions::builder()
                        .address(contract_config.address().to_string())
                        .from_block(option.start_block)
                        .to_block(if option.target_block > current_block_num {
                            current_block_num
                        } else {
                            option.target_block
                        })
                        .build()
                })
                .collect::<Vec<GetLogsOptions>>())
        },
        |contract_options| {
            Ok(contract_options
                .iter()
                .map(|contract_option| {
                    let target_block = contract_option.target_block.unwrap_or(option.target_block);
                    GetLogsOptions::builder()
                        .address(contract_option.contract_config.address().to_string())
                        .from_block(contract_option.start_block.unwrap_or(option.start_block))
                        .to_block(if target_block > current_block_num {
                            current_block_num
                        } else {
                            target_block
                        })
                        .build()
                })
                .collect::<Vec<GetLogsOptions>>())
        },
    )
}

async fn group_fetch<R: LoadedData>(
    client: &Arc<EtherScanClient>,
    options: Vec<GetLogsOptions>,
    concurrency: usize,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
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
    options: Vec<GetLogsOptions>,
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
    option: GetLogsOptions,
) -> Result<ContractData<R>> {
    let commitments: Vec<Commitment> = fetch_commitments(client, &option).await?;
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

async fn fetch_commitments(client: &EtherScanClient, option: &GetLogsOptions) -> Result<Vec<Commitment>> {
    let mut commitments: Vec<Commitment> = Vec::new();
    client
        .fetch_event_logs::<CommitmentCrossChainFilter>(option.clone())
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
    client
        .fetch_event_logs::<CommitmentQueuedFilter>(option.clone())
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
        .fetch_event_logs::<CommitmentIncludedFilter>(option.clone())
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
    commitments.sort_by_key(|commitment| commitment.block_number);
    Ok(commitments)
}

async fn fetch_nullifiers(client: &EtherScanClient, option: &GetLogsOptions) -> Result<Vec<Nullifier>> {
    let mut nullifiers: Vec<Nullifier> = Vec::new();
    client
        .fetch_event_logs::<CommitmentSpentFilter>(option.clone())
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
