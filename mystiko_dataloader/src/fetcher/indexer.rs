use crate::data::ChainResult;
use crate::data::ContractData;
use crate::data::ContractResult;
use crate::data::LoadedData;
use crate::data::{Data, DataType};
use crate::data::{FullData, LiteData};
use crate::fetcher::FetchOptions;
use crate::fetcher::FetchResult;
use crate::fetcher::FetcherError;
use crate::fetcher::FetcherLogOptions;
use crate::fetcher::{ChainLoadedBlockOptions, DataFetcher};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use hex::FromHexError;
use log::info;
use mystiko_indexer_client::{
    CommitmentForDataLoader, CommitmentStatus as IndexerCommitmentStatus, ContractResultDataResponse,
    DataLoaderRequest, IndexerClient, NullifierForDataLoader,
};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::biguint_str_to_bytes;
use mystiko_utils::hex::decode_hex;
use std::collections::HashMap;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Error, Debug)]
pub enum IndexerFetcherError {
    #[error("fetcher contract with error {0}")]
    ContractResultError(String),
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
    #[error("unsupported chain id: {0}")]
    UnsupportedChainError(u64),
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IndexerFetcher<R> {
    indexer_client: IndexerClient,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct ContractDataLoaderOptions {
    pub(crate) chain_id: u64,
    pub(crate) contract_address: String,
    pub(crate) start_block: u64,
    pub(crate) end_block: u64,
    pub(crate) indexer_filter_size: u64,
}

#[async_trait]
impl<R> DataFetcher<R> for IndexerFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let chain_config = option.config.find_chain(option.chain_id).ok_or_else(|| {
            FetcherError::AnyhowError(anyhow!(IndexerFetcherError::UnsupportedChainError(option.chain_id)))
        })?;
        let indexer_filter_size = chain_config.indexer_filter_size().max(1);
        let contract_options = to_options(indexer_filter_size, option);
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(match contract_options {
                Some(options) => fetch_contracts::<R>(options, &self.indexer_client)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
                None => fetch_all::<R>(indexer_filter_size, option, &self.indexer_client)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
            })
            .build())
    }

    async fn chain_loaded_block(&self, options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        Ok(self
            .indexer_client
            .query_chain_sync_repsonse_by_id(options.chain_id)
            .await
            .map_err(FetcherError::AnyhowError)?
            .current_sync_block_num)
    }
}

impl<R> From<IndexerClient> for IndexerFetcher<R> {
    fn from(client: IndexerClient) -> Self {
        Self::builder().indexer_client(client).build()
    }
}

async fn fetch_contracts<R: LoadedData>(
    options: Vec<ContractDataLoaderOptions>,
    client: &IndexerClient,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut results = Vec::new();
    for option in options {
        let result = fetch_contract_result::<R>(&option, client).await?;
        results.push(result);
    }
    Ok(results)
}

async fn fetch_contract_result<R: LoadedData>(
    option: &ContractDataLoaderOptions,
    client: &IndexerClient,
) -> Result<ContractResult<ContractData<R>>> {
    Ok(ContractResult::builder()
        .address(option.contract_address.to_string())
        .result(fetch_data(option, client).await)
        .build())
}

async fn fetch_data<R: LoadedData>(
    option: &ContractDataLoaderOptions,
    client: &IndexerClient,
) -> Result<ContractData<R>> {
    let contract_results = fetch_contract::<R>(option, client).await?;
    let chain_id = option.chain_id;
    let address = &option.contract_address;
    let start_block = option.start_block;
    build_contract_result::<R>(&contract_results, chain_id, address, start_block)
}

async fn fetch_contract<R: LoadedData>(
    option: &ContractDataLoaderOptions,
    client: &IndexerClient,
) -> Result<Vec<ContractResultDataResponse>> {
    let mut contract_results = Vec::new();
    if option.start_block > option.end_block {
        return Ok(contract_results);
    }
    let mut start_block = option.start_block;
    let mut to_block;
    loop {
        to_block = option.end_block.min(start_block + option.indexer_filter_size - 1);
        let r = DataLoaderRequest::builder()
            .contract_address(option.contract_address.to_string())
            .start_block(Some(start_block))
            .end_block(Some(to_block))
            .build();
        let result = send_requests::<R>(option.chain_id, start_block, to_block, client, Some(vec![r])).await?;
        contract_results.extend(result);
        if to_block == option.end_block {
            break;
        }
        start_block = to_block + 1;
    }
    Ok(contract_results)
}

async fn fetch_all<R: LoadedData>(
    indexer_filter_size: u64,
    option: &FetchOptions,
    client: &IndexerClient,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut contract_results: Vec<ContractResult<ContractData<R>>> = Vec::new();
    if option.start_block > option.target_block {
        return Ok(contract_results);
    }
    let mut start_block = option.start_block;
    let mut to_block;
    let mut contracts_results: Vec<ContractResultDataResponse> = Vec::new();
    loop {
        to_block = option.target_block.min(start_block + indexer_filter_size - 1);
        let result = send_requests::<R>(option.chain_id, start_block, to_block, client, None).await?;
        contracts_results.extend(result);
        if to_block == option.target_block {
            break;
        }
        start_block = to_block + 1;
    }
    let contract_result_map: HashMap<String, Vec<ContractResultDataResponse>> =
        contracts_results.into_iter().fold(HashMap::new(), |mut map, result| {
            let address = result.contract_address.clone();
            match map.get_mut(&address) {
                Some(results) => {
                    results.push(result);
                }
                None => {
                    map.insert(address, vec![result]);
                }
            }
            map
        });
    for (address, a) in contract_result_map.iter() {
        contract_results.push(
            ContractResult::builder()
                .address(address.to_string())
                .result(build_contract_result::<R>(
                    a,
                    option.chain_id,
                    address,
                    option.start_block,
                ))
                .build(),
        );
    }
    Ok(contract_results)
}

async fn send_requests<R: LoadedData>(
    chain_id: u64,
    start_block: u64,
    end_block: u64,
    client: &IndexerClient,
    requests: Option<Vec<DataLoaderRequest>>,
) -> Result<Vec<ContractResultDataResponse>> {
    match R::data_type() {
        DataType::Full => client.find_full_data(chain_id, start_block, end_block, requests).await,
        DataType::Lite => client.find_lite_data(chain_id, start_block, end_block, requests).await,
    }
}

fn to_options(indexer_filter_size: u64, option: &FetchOptions) -> Option<Vec<ContractDataLoaderOptions>> {
    option.contract_options.as_ref().map(|contract_options| {
        contract_options
            .iter()
            .map(|contract_option| {
                ContractDataLoaderOptions::builder()
                    .chain_id(option.chain_id)
                    .contract_address(contract_option.contract_config.address().to_string())
                    .start_block(contract_option.start_block.unwrap_or(option.start_block))
                    .end_block(contract_option.target_block.unwrap_or(option.target_block))
                    .indexer_filter_size(indexer_filter_size)
                    .build()
            })
            .collect::<Vec<ContractDataLoaderOptions>>()
    })
}

fn build_contract_result<R: LoadedData>(
    contract_results: &Vec<ContractResultDataResponse>,
    chain_id: u64,
    address: &str,
    start_block: u64,
) -> Result<ContractData<R>> {
    let mut actual_end_block: u64 = 0;
    let mut commitments = Vec::new();
    let mut nullifiers = Vec::new();
    for contract_response in contract_results {
        if contract_response.is_error {
            let esg = &contract_response.error_msg;
            return Err(anyhow!(IndexerFetcherError::ContractResultError(
                esg.clone().unwrap_or_else(|| String::from("unkonw error"))
            )));
        } else {
            commitments.extend(sort_commitments(contract_response.commitments.as_ref())?);
            nullifiers.extend(sort_nullifiers(contract_response.nullifiers.as_ref())?);
            actual_end_block = actual_end_block.max(contract_response.actual_end_block);
        }
    }
    let data = match R::data_type() {
        DataType::Full => {
            let fulldata = FullData::builder()
                .commitments(commitments)
                .nullifiers(nullifiers)
                .build();
            info!(
                "{} fetch {} commitments and {} nullifiers",
                FetcherLogOptions::builder()
                    .address(address.to_string())
                    .chain_id(chain_id)
                    .start_block(start_block)
                    .end_block(actual_end_block)
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
                    .address(address.to_string())
                    .chain_id(chain_id)
                    .start_block(start_block)
                    .end_block(actual_end_block)
                    .data_type(DataType::Lite)
                    .build()
                    .to_string(),
                litedata.commitments.len()
            );
            R::from_data(Data::Lite(litedata))
        }
    };
    Ok(ContractData::builder()
        .address(address.to_string())
        .start_block(start_block)
        .end_block(actual_end_block)
        .data(data)
        .build())
}

fn sort_commitments(commitment_resp: &[CommitmentForDataLoader]) -> Result<Vec<Commitment>> {
    let mut commitments = commitment_resp
        .iter()
        .map(|commit| {
            Ok(Commitment::builder()
                .commitment_hash(biguint_str_to_bytes(&commit.commitment_hash)?)
                .status(convert_status(&commit.status))
                .block_number(commit.block_number)
                .included_block_number(commit.included_block_number)
                .src_chain_block_number(commit.src_chain_block_number)
                .leaf_index(commit.leaf_index)
                .rollup_fee(
                    commit
                        .rollup_fee
                        .as_ref()
                        .map(|rf| biguint_str_to_bytes(rf))
                        .transpose()?,
                )
                .encrypted_note(decode_hex_str_opt(commit.encrypted_note.as_ref())?)
                .queued_transaction_hash(decode_hex_str_opt(commit.queued_transaction_hash.as_ref())?)
                .included_transaction_hash(decode_hex_str_opt(commit.included_transaction_hash.as_ref())?)
                .src_chain_transaction_hash(decode_hex_str_opt(commit.src_chain_transaction_hash.as_ref())?)
                .build())
        })
        .collect::<Result<Vec<Commitment>>>()?;
    commitments.sort_by_key(|commitment| commitment.block_number);
    Ok(commitments)
}

fn sort_nullifiers(nullifiers_resp_opt: Option<&Vec<NullifierForDataLoader>>) -> Result<Vec<Nullifier>> {
    nullifiers_resp_opt.map_or(Ok(Vec::new()), |nullifiers_resp| {
        let mut nullifiers = nullifiers_resp
            .iter()
            .map(|nullifier| {
                Ok(Nullifier::builder()
                    .block_number(nullifier.block_number)
                    .transaction_hash(decode_hex_str(&nullifier.transaction_hash)?)
                    .nullifier(biguint_str_to_bytes(&nullifier.nullifier)?)
                    .build())
            })
            .collect::<Result<Vec<Nullifier>>>()?;
        nullifiers.sort_by_key(|nullifier| nullifier.block_number);
        Ok(nullifiers)
    })
}

fn decode_hex_str(hex: &str) -> Result<Vec<u8>> {
    Ok(decode_hex(hex).map_err(IndexerFetcherError::FromHexError)?)
}

fn decode_hex_str_opt(hex_str_opt: Option<&String>) -> Result<Option<Vec<u8>>> {
    hex_str_opt.map(|hs| decode_hex_str(hs)).transpose()
}

fn convert_status(commitment_status: &IndexerCommitmentStatus) -> CommitmentStatus {
    match commitment_status {
        IndexerCommitmentStatus::SrcSucceeded => CommitmentStatus::SrcSucceeded,
        IndexerCommitmentStatus::Queued => CommitmentStatus::Queued,
        IndexerCommitmentStatus::Succeeded => CommitmentStatus::Included,
        IndexerCommitmentStatus::Failed => CommitmentStatus::Unspecified,
    }
}
