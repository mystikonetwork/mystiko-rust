use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::result::ContractResult;
use crate::data::types::LoadedData;
use crate::data::types::{Data, DataType};
use crate::data::types::{FullData, LiteData};
use crate::fetcher::types::ContractFetchOption;
use crate::fetcher::types::DataFetcher;
use crate::fetcher::types::FetchOption;
use crate::fetcher::types::FetchResult;
use crate::fetcher::types::LogPrefixOptions;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_indexer_client::client::IndexerClient;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoader;
use mystiko_indexer_client::types::commitment::CommitmentForDataLoaderResponse;
use mystiko_indexer_client::types::commitment::DepositStatus;
use mystiko_indexer_client::types::commitment_spent::{DataLoaderRequest, DataLoaderResponse, NullifierForDataLoader};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use std::collections::HashMap;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct IndexerFetcherOptions {
    chain_id: u64,
    indexer_client: IndexerClient,
}

pub struct IndexerFetcher<R> {
    chain_id: u64,
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
        match option {
            FetchOption::Chain(chain_option) => {
                if chain_option.chain_id != self.chain_id {
                    error!(
                        "chain_id param in ChainFetchOption({}) is different from IndexerFetcher({})",
                        chain_option.chain_id, self.chain_id
                    );
                    return Err(anyhow!(format!(
                        "chain_id param in ChainFetchOption({}) is different from IndexerFetcher({})",
                        chain_option.chain_id, self.chain_id
                    )));
                }
                let chain_config = chain_option.config.find_chain(chain_option.chain_id).unwrap();
                let contracts: Vec<ContractConfig> =
                    if chain_option.contracts.is_none() || chain_option.contracts.clone().unwrap().is_empty() {
                        chain_config.contracts_with_disabled()
                    } else {
                        chain_option.contracts.clone().unwrap()
                    };
                contracts.iter().for_each(|contract| {
                    contract_fetch_options.push(ContractFetchOption {
                        config: Arc::clone(&chain_option.config),
                        chain_id: chain_option.chain_id,
                        address: contract.address().to_string(),
                        start_block: chain_option.start_block,
                        target_block: chain_option.target_block,
                    })
                });
            }
            FetchOption::Contracts(contract_options) => {
                let any_other_chain: Vec<&ContractFetchOption> = contract_options
                    .iter()
                    .filter(|option| option.chain_id != self.chain_id)
                    .collect();
                if !any_other_chain.is_empty() {
                    error!("param chain_id in options is inconsistent with that in IndexerFetcher");
                    return Err(anyhow!(
                        "param chain_id in options is inconsistent with that in IndexerFetcher"
                    ));
                }
                contract_fetch_options = contract_options.clone().to_owned();
            }
        }
        if contract_fetch_options.len() == 0 {
            warn!("IndexerFetcher found ContractFetcherOptions is empty, will do nothing");
            return Err(anyhow!(
                "IndexerFetcher found ContractFetcherOptions is empty, will do nothing"
            ));
        }
        info!(
            "{} start to fetch contracts, address list: {}",
            self.get_log_prefix(None),
            contract_fetch_options
                .iter()
                .map(|o| o.address.clone())
                .collect::<Vec<String>>()
                .join(",")
        );
        contract_results = self.fetch_contracts(&contract_fetch_options).await?;
        Ok(ChainResult::builder()
            .chain_id(self.chain_id)
            .contract_results(contract_results)
            .build())
    }
}

impl<R> IndexerFetcher<R>
where
    R: LoadedData,
{
    pub fn new(option: IndexerFetcherOptions) -> Self {
        IndexerFetcher {
            chain_id: option.chain_id,
            indexer_client: option.indexer_client,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn fetch_contracts(
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
        let result;
        match R::data_type() {
            DataType::Full => {
                debug!(
                    "{} start to fetch {} contracts full data",
                    self.get_log_prefix(None),
                    requests.len()
                );
                let full_data_resp = self.indexer_client.find_full_data(self.chain_id, requests).await?;
                debug!("{} fetch all contracts full data success", self.get_log_prefix(None));
                result = self.handle_full_data(&full_data_resp, &options).await?;
            }
            DataType::Lite => {
                info!(
                    "{} start to fetch {} contracts lite data",
                    self.get_log_prefix(None),
                    requests.len()
                );
                let lite_data_resp = self.indexer_client.find_lite_data(self.chain_id, requests).await?;
                info!("{} fetch all contracts lite data success", self.get_log_prefix(None));
                result = self.handle_lite_data(lite_data_resp, &options).await?;
            }
        }
        Ok(result)
    }

    async fn handle_full_data(
        &self,
        full_data_resp: &DataLoaderResponse,
        options: &Vec<ContractFetchOption>,
    ) -> Result<Vec<ContractResult<ContractData<R>>>> {
        let mut contract_results = vec![];
        let contract_commtments = &full_data_resp.commitment_responses;
        let mut contract_commtments_map = HashMap::new();
        contract_commtments.iter().for_each(|commit| {
            contract_commtments_map.insert(commit.contract_address.to_string(), commit);
        });
        let contract_nullifiers = &full_data_resp.nullifier_responses;
        let mut contract_nullfiers_map = HashMap::new();
        contract_nullifiers.iter().for_each(|nullifier| {
            contract_nullfiers_map.insert(nullifier.contract_address.to_string(), nullifier);
        });
        for option in options {
            let contract_commit_resp = contract_commtments_map.get(&option.address).unwrap();
            let contract_nullifier = contract_nullfiers_map.get(&option.address).unwrap();
            let actual_end_block = if option.target_block <= contract_commit_resp.current_sync_block_num {
                option.target_block
            } else {
                debug!("{} found current sync block number less than target_block({}), will use current sync block number instead of target_block", self.get_log_prefix(Some(&option.address)), option.target_block);
                contract_commit_resp.current_sync_block_num
            };
            let fulldata = FullData::builder()
                .commitments(convert_to_sorted_commitments(&contract_commit_resp.commitments))
                .nullifiers(convert_to_sorted_nullifiers(&contract_nullifier.nullifiers))
                .build();
            info!(
                "{} fetch {} commitments and {} nullifiers",
                self.get_log_prefix_with_block(
                    &LogPrefixOptions::builder()
                        .address(option.address.to_string())
                        .from_block(option.start_block)
                        .to_block(actual_end_block)
                        .build()
                ),
                fulldata.commitments.len(),
                fulldata.nullifiers.len()
            );
            let contract_data = ContractData::builder()
                .address(option.address.to_string())
                .start_block(option.start_block)
                .end_block(actual_end_block)
                .data(R::from_data(Data::Full(fulldata)))
                .build();
            contract_results.push(
                ContractResult::builder()
                    .address(option.address.to_string())
                    .result(Ok(contract_data))
                    .build(),
            );
        }
        debug!(
            "{} fetch {} contract results success",
            self.get_log_prefix(None),
            contract_results.len()
        );
        Ok(contract_results)
    }

    async fn handle_lite_data(
        &self,
        contract_commitments: Vec<CommitmentForDataLoaderResponse>,
        options: &Vec<ContractFetchOption>,
    ) -> Result<Vec<ContractResult<ContractData<R>>>> {
        let mut contract_results = vec![];
        let mut contract_commtments_map = HashMap::new();
        contract_commitments.iter().for_each(|commit| {
            contract_commtments_map.insert(commit.contract_address.to_string(), commit);
        });
        for option in options {
            let contract_commit_resp = contract_commtments_map.get(&option.address).unwrap();
            let actual_end_block = if option.target_block <= contract_commit_resp.current_sync_block_num {
                option.target_block
            } else {
                debug!("{} found current sync block number less than target_block({}), will use current sync block number instead of target_block", self.get_log_prefix(Some(&option.address)), option.target_block);
                contract_commit_resp.current_sync_block_num
            };
            let litedata = LiteData::builder()
                .commitments(convert_to_sorted_commitments(&contract_commit_resp.commitments))
                .build();
            info!(
                "{} fetch {} commitments",
                self.get_log_prefix_with_block(
                    &LogPrefixOptions::builder()
                        .address(option.address.to_string())
                        .from_block(option.start_block)
                        .to_block(actual_end_block)
                        .build()
                ),
                litedata.commitments.len()
            );
            let contract_data = ContractData::builder()
                .address(option.address.to_string())
                .start_block(option.start_block)
                .end_block(actual_end_block)
                .data(R::from_data(Data::Lite(litedata)))
                .build();
            contract_results.push(
                ContractResult::builder()
                    .address(option.address.to_string())
                    .result(Ok(contract_data))
                    .build(),
            );
        }
        debug!(
            "{} fetch {} contract results success",
            self.get_log_prefix(None),
            contract_results.len()
        );
        Ok(contract_results)
    }

    fn get_log_prefix(&self, address: Option<&str>) -> String {
        if address.is_none() {
            format!("IndexerFetcher[type={:?}, chain_id={}]", R::data_type(), self.chain_id)
        } else {
            format!(
                "IndexerFetcher[type={:?}, chain_id={}, address={}]",
                R::data_type(),
                self.chain_id,
                address.unwrap()
            )
        }
    }

    fn get_log_prefix_with_block(&self, opt: &LogPrefixOptions) -> String {
        format!(
            "IndexerFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
            R::data_type(),
            self.chain_id,
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
                .rollup_fee(commit.rollup_fee.as_ref().map(|rf| rf.clone().into_bytes()))
                .encrypted_note(commit.encrypted_note.as_ref().map(|en| en.clone().into_bytes()))
                .queued_transaction_hash(
                    commit
                        .queued_transaction_hash
                        .as_ref()
                        .map(|qth| qth.clone().into_bytes()),
                )
                .included_transaction_hash(
                    commit
                        .included_transaction_hash
                        .as_ref()
                        .map(|ith| ith.clone().into_bytes()),
                )
                .src_chain_transaction_hash(
                    commit
                        .src_chain_transaction_hash
                        .as_ref()
                        .map(|scth| scth.clone().into_bytes()),
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
                .transaction_hash(nullifier.transaction_hash.clone().into_bytes())
                .nullifier(nullifier.nullifier.clone().into_bytes())
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
