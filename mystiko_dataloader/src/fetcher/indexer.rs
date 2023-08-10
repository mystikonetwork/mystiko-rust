use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::result::ContractResult;
use crate::data::types::LoadedData;
use crate::data::types::{Data, DataType};
use crate::data::types::{FullData, LiteData};
use crate::error::DataLoaderError;
use crate::fetcher::types::ContractFetchOption;
use crate::fetcher::types::DataFetcher;
use crate::fetcher::types::FetchOption;
use crate::fetcher::types::FetchResult;
use crate::fetcher::types::LogPrefixOptions;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoader;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoaderResponse;
use mystiko_indexer_client::types::commitment::DepositStatus;
use mystiko_indexer_client::types::commitment_spent::NullifierForDataLoaderResponse;
use mystiko_indexer_client::types::commitment_spent::{DataLoaderRequest, NullifierForDataLoader};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use std::collections::HashMap;
use std::sync::Arc;

pub struct IndexerFetcher<R> {
    indexer_client: IndexerClient,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataFetcher<R> for IndexerFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOption) -> FetchResult<R> {
        let mut contract_fetch_options = vec![];
        let contract_results;
        let fetch_chain_id;
        match option {
            FetchOption::Chain(chain_fetch_option) => {
                if let Some(chain_config) = chain_fetch_option.config.find_chain(chain_fetch_option.chain_id) {
                    fetch_chain_id = chain_fetch_option.chain_id;
                    let contracts = if let Some(contracts_config) = &chain_fetch_option.contracts {
                        contracts_config.to_owned()
                    } else {
                        chain_config.contracts_with_disabled()
                    };
                    contracts.into_iter().for_each(|contract| {
                        contract_fetch_options.push(ContractFetchOption {
                            config: Arc::clone(&chain_fetch_option.config),
                            chain_id: chain_fetch_option.chain_id,
                            address: contract.address().to_string(),
                            start_block: chain_fetch_option.start_block,
                            target_block: chain_fetch_option.target_block,
                        })
                    });
                } else {
                    error!(
                        "chain(id={}) config can not found in mystiko config",
                        chain_fetch_option.chain_id
                    );
                    return Err(anyhow!(DataLoaderError::FetcherParamsError(format!(
                        "chain(id={}) config can not found in mystiko config",
                        chain_fetch_option.chain_id
                    ))));
                }
            }
            FetchOption::Contracts(contract_options) => {
                if let Some(first_option) = contract_options.get(0) {
                    fetch_chain_id = first_option.chain_id;
                    let any_other_chain = contract_options
                        .iter()
                        .filter(|opt| opt.chain_id != fetch_chain_id)
                        .count();
                    if any_other_chain > 0 {
                        error!("contract fetch options has different chain id");
                        return Err(anyhow!(DataLoaderError::FetcherParamsError(
                            "contract fetch options has different chain id".to_string()
                        )));
                    }
                    contract_fetch_options = contract_options.to_vec();
                } else {
                    warn!("Fetcher found ContractFetcherOptions is empty, will do nothing");
                    return Err(anyhow!(DataLoaderError::FetcherParamsError(
                        "Fetcher found ContractFetcherOptions is empty, will do nothing".to_string()
                    )));
                }
            }
        }
        info!(
            "{} start to fetch contracts, address list: {}",
            self.get_log_prefix(fetch_chain_id, None),
            contract_fetch_options
                .iter()
                .map(|o| o.address.clone())
                .collect::<Vec<String>>()
                .join(",")
        );
        contract_results = self.fetch_contracts(&contract_fetch_options).await?;
        Ok(ChainResult::builder()
            .chain_id(fetch_chain_id)
            .contract_results(contract_results)
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

    async fn fetch_contracts(
        &self,
        options: &Vec<ContractFetchOption>,
    ) -> Result<Vec<ContractResult<ContractData<R>>>> {
        let requests = options
            .iter()
            .map(|option| {
                DataLoaderRequest::builder()
                    .contract_address(option.address.to_string())
                    .start_block(option.start_block)
                    .end_block(option.target_block)
                    .build()
            })
            .collect::<Vec<DataLoaderRequest>>();
        let first_option;
        if let Some(some_first_option) = options.get(0) {
            first_option = some_first_option;
        } else {
            error!("fetch options is empty");
            return Err(anyhow!(DataLoaderError::FetcherParamsError(
                "fetch options is empty".to_string()
            )));
        }
        match R::data_type() {
            DataType::Full => {
                let full_data_resp = self
                    .indexer_client
                    .find_full_data(first_option.chain_id, requests)
                    .await?;
                self.wrap_contracts_result(
                    full_data_resp.commitment_responses,
                    Some(full_data_resp.nullifier_responses),
                    &options,
                )
                .await
            }
            DataType::Lite => {
                let commitment_responses = self
                    .indexer_client
                    .find_lite_data(first_option.chain_id, requests)
                    .await?;
                self.wrap_contracts_result(commitment_responses, None, &options).await
            }
        }
    }

    async fn wrap_contracts_result(
        &self,
        commitment_responses: Vec<CommitmentForDataLoaderResponse>,
        nullifier_responses_opt: Option<Vec<NullifierForDataLoaderResponse>>,
        options: &Vec<ContractFetchOption>,
    ) -> Result<Vec<ContractResult<ContractData<R>>>> {
        let mut contract_results = vec![];
        let mut commitment_responses_map = HashMap::new();
        commitment_responses.into_iter().for_each(|commitment_response| {
            commitment_responses_map.insert(commitment_response.contract_address.to_string(), commitment_response);
        });
        let mut nullifier_responses_map = HashMap::new();
        if let Some(nullifier_responses) = nullifier_responses_opt {
            nullifier_responses.into_iter().for_each(|nullifier_response| {
                nullifier_responses_map.insert(nullifier_response.contract_address.to_string(), nullifier_response);
            });
        }
        for option in options {
            let commitment_response;
            if let Some(some_commitment_response) = commitment_responses_map.get(&option.address) {
                commitment_response = some_commitment_response.to_owned();
            } else {
                error!("no commitment response for contract {}", &option.address);
                return Err(anyhow!(DataLoaderError::FetcherAssembleDataError(format!(
                    "no commitment response for contract {}",
                    &option.address
                ))));
            }
            let nullifier_response_opt = nullifier_responses_map.get(&option.address);
            let contract_data = self
                .wrap_contract_data(commitment_response, nullifier_response_opt, option)
                .await;
            contract_results.push(
                ContractResult::builder()
                    .address(option.address.to_string())
                    .result(contract_data)
                    .build(),
            );
        }
        Ok(contract_results)
    }

    async fn wrap_contract_data(
        &self,
        commitment_response: &CommitmentForDataLoaderResponse,
        nullifier_response_opt: Option<&NullifierForDataLoaderResponse>,
        option: &ContractFetchOption,
    ) -> Result<ContractData<R>> {
        let actual_end_block = if option.target_block <= commitment_response.current_sync_block_num {
            option.target_block
        } else {
            debug!("{} found current sync block number({}) less than target_block({}), will use current sync block number instead of target_block", self.get_log_prefix(option.chain_id, Some(&option.address)), commitment_response.current_sync_block_num, option.target_block);
            commitment_response.current_sync_block_num
        };
        let log_prefix = LogPrefixOptions::builder()
            .address(option.address.to_string())
            .chain_id(option.chain_id)
            .from_block(option.start_block)
            .to_block(actual_end_block)
            .build();
        let data = match R::data_type() {
            DataType::Full => {
                let mut nullifiers = &Vec::new();
                if let Some(some_nullifier_response) = nullifier_response_opt {
                    nullifiers = &some_nullifier_response.nullifiers;
                }
                let fulldata = FullData::builder()
                    .commitments(convert_to_sorted_commitments(&commitment_response.commitments))
                    .nullifiers(convert_to_sorted_nullifiers(&nullifiers))
                    .build();
                info!(
                    "{} fetch {} commitments and {} nullifiers",
                    self.get_log_prefix_with_block(&log_prefix),
                    fulldata.commitments.len(),
                    fulldata.nullifiers.len()
                );
                R::from_data(Data::Full(fulldata))
            }
            DataType::Lite => {
                let litedata = LiteData::builder()
                    .commitments(convert_to_sorted_commitments(&commitment_response.commitments))
                    .build();
                info!(
                    "{} fetch {} commitments",
                    self.get_log_prefix_with_block(&log_prefix),
                    litedata.commitments.len()
                );
                R::from_data(Data::Lite(litedata))
            }
        };
        Ok(ContractData::builder()
            .address(option.address.to_string())
            .start_block(option.start_block)
            .end_block(actual_end_block)
            .data(data)
            .build())
    }

    fn get_log_prefix(&self, chain_id: u64, address: Option<&str>) -> String {
        if let Some(some_address) = address {
            format!(
                "IndexerFetcher[type={:?}, chain_id={}, address={}]",
                R::data_type(),
                chain_id,
                some_address
            )
        } else {
            format!("IndexerFetcher[type={:?}, chain_id={}]", R::data_type(), chain_id)
        }
    }

    fn get_log_prefix_with_block(&self, opt: &LogPrefixOptions) -> String {
        format!(
            "IndexerFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
            R::data_type(),
            opt.chain_id,
            opt.address,
            opt.from_block,
            opt.to_block
        )
    }
}

fn convert_to_sorted_commitments(commitment_resp: &Vec<CommitmentForDataLoader>) -> Vec<Commitment> {
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

fn convert_to_sorted_nullifiers(nullifiers_resp: &Vec<NullifierForDataLoader>) -> Vec<Nullifier> {
    let mut nullifiers = nullifiers_resp
        .iter()
        .map(|nullifier| {
            Nullifier::builder()
                .block_number(nullifier.block_number)
                .transaction_hash(nullifier.transaction_hash.as_bytes())
                .nullifier(nullifier.nullifier.as_bytes())
                .build()
        })
        .collect::<Vec<Nullifier>>();
    nullifiers.sort_by(|a, b| a.block_number.cmp(&b.block_number));
    nullifiers
}

fn convert_status(deposit_status: &DepositStatus) -> CommitmentStatus {
    match deposit_status {
        DepositStatus::SrcSucceeded => CommitmentStatus::SrcSucceeded,
        DepositStatus::Queued => CommitmentStatus::Queued,
        DepositStatus::Succeeded => CommitmentStatus::Included,
        DepositStatus::Failed => CommitmentStatus::Unspecified,
    }
}
