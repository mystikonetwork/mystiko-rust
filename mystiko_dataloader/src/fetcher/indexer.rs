use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::result::ContractResult;
use crate::data::types::LoadedData;
use crate::data::types::{Data, DataType};
use crate::data::types::{FullData, LiteData};
use crate::fetcher::error::FetcherError;
use crate::fetcher::types::DataFetcher;
use crate::fetcher::types::FetchOptions;
use crate::fetcher::types::FetchResult;
use crate::fetcher::types::FetcherLogOptions;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use hex::FromHexError;
use log::info;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoader;
use mystiko_indexer_client::types::commitment::ContractResultDataResponse;
use mystiko_indexer_client::types::commitment_spent::{DataLoaderRequest, NullifierForDataLoader};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::biguint_str_to_bytes;
use mystiko_utils::hex::decode_hex;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct IndexerFetcher<R> {
    indexer_client: IndexerClient,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataFetcher<R> for IndexerFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let requests = to_requests(option);
        let contracts_response = match R::data_type() {
            DataType::Full => {
                self.indexer_client
                    .find_full_data(option.chain_id, option.start_block, option.target_block, requests)
                    .await
            }
            DataType::Lite => {
                self.indexer_client
                    .find_lite_data(option.chain_id, option.start_block, option.target_block, requests)
                    .await
            }
        }
        .map_err(FetcherError::AnyhowError)?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(handle_contracts_response::<R>(option.chain_id, contracts_response))
            .build())
    }
}

fn handle_contracts_response<R: LoadedData>(
    chain_id: u64,
    contracts_response: Vec<ContractResultDataResponse>,
) -> Vec<ContractResult<ContractData<R>>> {
    contracts_response
        .into_iter()
        .map(|contract_response| {
            ContractResult::builder()
                .address(contract_response.contract_address.to_string())
                .result(build_contract_result::<R>(chain_id, contract_response))
                .build()
        })
        .collect::<Vec<ContractResult<ContractData<R>>>>()
}

fn to_requests(option: &FetchOptions) -> Option<Vec<DataLoaderRequest>> {
    option.contract_options.as_ref().map(|contract_options| {
        contract_options
            .iter()
            .map(|contract_option| {
                DataLoaderRequest::builder()
                    .contract_address(contract_option.contract_config.address().to_string())
                    .start_block(contract_option.start_block)
                    .end_block(contract_option.target_block)
                    .build()
            })
            .collect::<Vec<DataLoaderRequest>>()
    })
}

fn build_contract_result<R: LoadedData>(
    chain_id: u64,
    contract_response: ContractResultDataResponse,
) -> Result<ContractData<R>> {
    if contract_response.is_error {
        Err(anyhow!(IndexerFetcherError::ContractResultError(
            contract_response
                .error_msg
                .unwrap_or_else(|| String::from("unkonw error"))
        )))
    } else {
        let data = match R::data_type() {
            DataType::Full => {
                let fulldata = FullData::builder()
                    .commitments(sort_commitments(contract_response.commitments.as_ref())?)
                    .nullifiers(sort_nullifiers(contract_response.nullifiers.as_ref())?)
                    .build();
                info!(
                    "{} fetch {} commitments and {} nullifiers",
                    build_log_options(
                        &DataType::Full,
                        &FetcherLogOptions::builder()
                            .address(contract_response.contract_address.to_string())
                            .chain_id(chain_id)
                            .start_block(contract_response.start_block)
                            .end_block(contract_response.actual_end_block)
                            .build()
                    ),
                    fulldata.commitments.len(),
                    fulldata.nullifiers.len()
                );
                R::from_data(Data::Full(fulldata))
            }
            DataType::Lite => {
                let litedata = LiteData::builder()
                    .commitments(sort_commitments(contract_response.commitments.as_ref())?)
                    .build();
                info!(
                    "{} fetch {} commitments",
                    build_log_options(
                        &DataType::Lite,
                        &FetcherLogOptions::builder()
                            .address(contract_response.contract_address.to_string())
                            .chain_id(chain_id)
                            .start_block(contract_response.start_block)
                            .end_block(contract_response.actual_end_block)
                            .build()
                    ),
                    litedata.commitments.len()
                );
                R::from_data(Data::Lite(litedata))
            }
        };
        Ok(ContractData::builder()
            .address(contract_response.contract_address.to_string())
            .start_block(contract_response.start_block)
            .end_block(contract_response.actual_end_block)
            .data(data)
            .build())
    }
}

fn sort_commitments(commitment_resp: &[CommitmentForDataLoader]) -> Result<Vec<Commitment>> {
    let mut commitments = commitment_resp
        .iter()
        .map(|commit| -> Result<Commitment> {
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
                        .map(|rf| -> Result<Vec<u8>> { biguint_str_to_bytes(rf) })
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
            .map(|nullifier| -> Result<Nullifier> {
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
    hex_str_opt
        .map(|hs| -> Result<Vec<u8>> { decode_hex_str(hs) })
        .transpose()
}

fn convert_status(commitment_status: &mystiko_indexer_client::types::commitment::CommitmentStatus) -> CommitmentStatus {
    match commitment_status {
        mystiko_indexer_client::types::commitment::CommitmentStatus::SrcSucceeded => CommitmentStatus::SrcSucceeded,
        mystiko_indexer_client::types::commitment::CommitmentStatus::Queued => CommitmentStatus::Queued,
        mystiko_indexer_client::types::commitment::CommitmentStatus::Succeeded => CommitmentStatus::Included,
        mystiko_indexer_client::types::commitment::CommitmentStatus::Failed => CommitmentStatus::Unspecified,
    }
}

fn build_log_options(data_type: &DataType, opt: &FetcherLogOptions) -> String {
    format!(
        "IndexerFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
        data_type, opt.chain_id, opt.address, opt.start_block, opt.end_block
    )
}

#[derive(Error, Debug)]
pub enum IndexerFetcherError {
    #[error("fetcher contract with error {0}")]
    ContractResultError(String),
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
}
