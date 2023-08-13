use super::error::FetcherError;
use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::result::ContractResult;
use crate::data::types::LoadedData;
use crate::data::types::{Data, DataType};
use crate::data::types::{FullData, LiteData};
use crate::fetcher::types::DataFetcher;
use crate::fetcher::types::FetchOptions;
use crate::fetcher::types::FetchResult;
use crate::fetcher::types::LogPrefixOptions;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::info;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoader;
use mystiko_indexer_client::types::commitment::ContractResultDataResponse;
use mystiko_indexer_client::types::commitment::DepositStatus;
use mystiko_indexer_client::types::commitment_spent::{DataLoaderRequest, NullifierForDataLoader};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};

pub struct IndexerFetcher<R> {
    indexer_client: IndexerClient,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataFetcher<R> for IndexerFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let requests = self.wrap_contract_options_requests(option);
        let contracts_response = match R::data_type() {
            DataType::Full => {
                self.indexer_client
                    .find_full_data(option.chain_id, option.start_block, option.target_block, requests)
                    .await?
            }
            DataType::Lite => {
                self.indexer_client
                    .find_lite_data(option.chain_id, option.start_block, option.target_block, requests)
                    .await?
            }
        };
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(self.handle_contracts_response(option.chain_id, contracts_response))
            .build())
    }
}

impl<R> IndexerFetcher<R>
where
    R: LoadedData,
{
    pub fn new(indexer_client: IndexerClient) -> Self {
        IndexerFetcher {
            indexer_client,
            _phantom: std::marker::PhantomData,
        }
    }

    fn wrap_contract_options_requests(&self, option: &FetchOptions) -> Option<Vec<DataLoaderRequest>> {
        if let Some(some_contract_options) = option.contract_options.as_ref() {
            let mut requests: Vec<DataLoaderRequest> = Vec::new();
            for contract_option in some_contract_options {
                requests.push(
                    DataLoaderRequest::builder()
                        .contract_address(contract_option.contract_config.address().to_string())
                        .start_block(contract_option.start_block)
                        .end_block(contract_option.target_block)
                        .build(),
                );
            }
            Some(requests)
        } else {
            None
        }
    }

    fn handle_contracts_response(
        &self,
        chain_id: u64,
        contracts_response: Vec<ContractResultDataResponse>,
    ) -> Vec<ContractResult<ContractData<R>>> {
        let mut contracts_results = Vec::new();
        for contract_response in contracts_response {
            contracts_results.push(
                ContractResult::builder()
                    .address(contract_response.contract_address.to_string())
                    .result(self.wrap_contract_data_result(chain_id, contract_response))
                    .build(),
            );
        }
        contracts_results
    }

    fn wrap_contract_data_result(
        &self,
        chain_id: u64,
        contract_response: ContractResultDataResponse,
    ) -> Result<ContractData<R>> {
        if contract_response.is_error {
            Err(anyhow!(FetcherError::FetchContractResultError(
                contract_response
                    .error_msg
                    .unwrap_or_else(|| String::from("unkonw error"))
            )))
        } else {
            let data = match R::data_type() {
                DataType::Full => {
                    let fulldata = FullData::builder()
                        .commitments(convert_to_sorted_commitments(contract_response.commitments.as_ref()))
                        .nullifiers(convert_to_sorted_nullifiers(contract_response.nullifiers.as_ref()))
                        .build();
                    info!(
                        "{} fetch {} commitments and {} nullifiers",
                        self.get_log_prefix_with_block(
                            &LogPrefixOptions::builder()
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
                        .commitments(convert_to_sorted_commitments(contract_response.commitments.as_ref()))
                        .build();
                    info!(
                        "{} fetch {} commitments",
                        self.get_log_prefix_with_block(
                            &LogPrefixOptions::builder()
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

    fn get_log_prefix_with_block(&self, opt: &LogPrefixOptions) -> String {
        format!(
            "IndexerFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
            R::data_type(),
            opt.chain_id,
            opt.address,
            opt.start_block,
            opt.end_block
        )
    }
}

fn convert_to_sorted_commitments(commitment_resp: &[CommitmentForDataLoader]) -> Vec<Commitment> {
    let mut commitments = commitment_resp
        .iter()
        .map(|commit| {
            Commitment::builder()
                .commitment_hash(commit.commitment_hash.as_bytes())
                .status(convert_status(&commit.status))
                .block_number(commit.block_number)
                .included_block_number(commit.included_block_number)
                .src_chain_block_number(commit.src_chain_block_number)
                .leaf_index(commit.leaf_index)
                .rollup_fee(commit.rollup_fee.as_ref().map(|rf| rf.as_bytes().to_vec()))
                .encrypted_note(commit.encrypted_note.as_ref().map(|en| en.as_bytes().to_vec()))
                .queued_transaction_hash(
                    commit
                        .queued_transaction_hash
                        .as_ref()
                        .map(|qth| qth.as_bytes().to_vec()),
                )
                .included_transaction_hash(
                    commit
                        .included_transaction_hash
                        .as_ref()
                        .map(|ith| ith.as_bytes().to_vec()),
                )
                .src_chain_transaction_hash(
                    commit
                        .src_chain_transaction_hash
                        .as_ref()
                        .map(|scth| scth.as_bytes().to_vec()),
                )
                .build()
        })
        .collect::<Vec<Commitment>>();
    commitments.sort_by(|a, b| a.block_number.cmp(&b.block_number));
    commitments
}

fn convert_to_sorted_nullifiers(nullifiers_resp_opt: Option<&Vec<NullifierForDataLoader>>) -> Vec<Nullifier> {
    if let Some(nullifiers_resp) = nullifiers_resp_opt {
        let mut nullifiers = nullifiers_resp
            .iter()
            .map(|nullifier| {
                Nullifier::builder()
                    .block_number(nullifier.block_number)
                    .transaction_hash(nullifier.transaction_hash.as_bytes().to_vec())
                    .nullifier(nullifier.nullifier.as_bytes().to_vec())
                    .build()
            })
            .collect::<Vec<Nullifier>>();
        nullifiers.sort_by(|a, b| a.block_number.cmp(&b.block_number));
        nullifiers
    } else {
        Vec::new()
    }
}

fn convert_status(deposit_status: &DepositStatus) -> CommitmentStatus {
    match deposit_status {
        DepositStatus::SrcSucceeded => CommitmentStatus::SrcSucceeded,
        DepositStatus::Queued => CommitmentStatus::Queued,
        DepositStatus::Succeeded => CommitmentStatus::Included,
        DepositStatus::Failed => CommitmentStatus::Unspecified,
    }
}
