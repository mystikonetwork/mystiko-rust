use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::sequencer::v1::{FetchContractRequest, FetchContractResponse};
use mystiko_sequencer_client::SequencerClient;
use mystiko_utils::address::{string_address_from_bytes, string_address_to_bytes};
use thiserror::Error;
use typed_builder::TypedBuilder;

use crate::data::Data;
use crate::data::{ChainResult, ContractData, ContractResult, DataType, FullData, LiteData, LoadedData};
use crate::fetcher::{
    ChainLoadedBlockOptions, DataFetcher, FetchOptions, FetchResult, FetcherError, FetcherLogOptions,
};

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct SequencerFetcher<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>> {
    client: Arc<C>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Error)]
pub enum SequencerFetcherError {
    #[error("client error raised error: {0}")]
    ClientError(anyhow::Error),
    #[error("start block {0} is bigger than end block {1}")]
    StartBlockTooBigError(u64, u64),
    #[error("no chain config found for chain id: {0}")]
    UnsupportedChainError(u64),
}

#[async_trait]
impl<R, C> DataFetcher<R> for SequencerFetcher<R, C>
where
    R: LoadedData,
    C: SequencerClient<FetchChainRequest, FetchChainResponse>,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        if option.start_block > option.target_block {
            return Err(FetcherError::AnyhowError(anyhow!(
                SequencerFetcherError::StartBlockTooBigError(option.start_block, option.target_block)
            )));
        }

        let chain_config = option.config.find_chain(option.chain_id).ok_or_else(|| {
            FetcherError::AnyhowError(anyhow!(SequencerFetcherError::UnsupportedChainError(option.chain_id)))
        })?;
        let sequencer_fetch_size = chain_config.sequencer_fetch_size().max(1);

        let responses = fetch_response::<R, C>(sequencer_fetch_size, option, &self.client)
            .await
            .map_err(FetcherError::AnyhowError)?;
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(build_results::<R>(responses, option))
            .build())
    }

    async fn chain_loaded_block(&self, options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        Ok(self
            .client
            .chain_loaded_block(options.chain_id)
            .await
            .map_err(|err| FetcherError::AnyhowError(err.into()))?)
    }
}

async fn fetch_response<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    sequencer_fetch_size: u64,
    option: &FetchOptions,
    client: &Arc<C>,
) -> Result<Vec<FetchContractResponse>> {
    let mut responses = Vec::new();
    let is_full = R::data_type() == DataType::Full;
    let mut chain_start_block = option.start_block;
    let mut chain_target_block;
    let mut i: u64 = 0;

    loop {
        let mut contracts = Vec::new();
        let mut is_some = false;

        if let Some(contract_options) = option.contract_options.as_ref() {
            is_some = true;
            for contract_option in contract_options {
                let contract_start_block =
                    contract_option.start_block.unwrap_or(option.start_block) + sequencer_fetch_size * i;
                let contract_to_block = contract_option
                    .target_block
                    .unwrap_or(option.target_block)
                    .min(contract_start_block + sequencer_fetch_size - 1);

                if contract_start_block <= contract_to_block {
                    let address = string_address_to_bytes(contract_option.contract_config.address())?;
                    let contract_request = FetchContractRequest::builder()
                        .contract_address(address)
                        .start_block(contract_start_block)
                        .target_block(contract_to_block)
                        .build();

                    contracts.push(contract_request);
                }
            }
        }
        if is_some && contracts.is_empty() {
            break;
        }

        chain_target_block = option.target_block.min(chain_start_block + sequencer_fetch_size - 1);

        let chain_request = FetchChainRequest::builder()
            .chain_id(option.chain_id)
            .start_block(chain_start_block)
            .target_block(chain_target_block)
            .contracts(contracts)
            .is_full(is_full)
            .build();
        let response = client.fetch_chain(chain_request).await?;
        responses.extend(response.contracts);

        if !is_some && chain_target_block == option.target_block {
            break;
        }
        chain_start_block = chain_target_block + 1;
        i += 1;
    }

    Ok(responses)
}

fn build_results<R: LoadedData>(
    contracts_results: Vec<FetchContractResponse>,
    option: &FetchOptions,
) -> Vec<ContractResult<ContractData<R>>> {
    let contract_result_map: HashMap<String, Vec<FetchContractResponse>> =
        contracts_results.into_iter().fold(HashMap::new(), |mut map, result| {
            let address = string_address_from_bytes(result.contract_address.clone());
            map.entry(address).or_default().push(result);
            map
        });
    let mut results: Vec<ContractResult<ContractData<R>>> = Vec::new();
    if let Some(contract_options) = option.contract_options.as_ref() {
        for contract_option in contract_options {
            let address = contract_option.contract_config.address();
            if let Some(response) = contract_result_map.get(address) {
                results.push(
                    ContractResult::builder()
                        .address(address.to_string())
                        .result(build_contract_result::<R>(
                            option.chain_id,
                            &address.to_string(),
                            contract_option.start_block.unwrap_or(option.start_block),
                            response,
                        ))
                        .build(),
                );
            }
        }
    } else {
        for (address, contract_results) in contract_result_map.iter() {
            results.push(
                ContractResult::builder()
                    .address(address.to_string())
                    .result(build_contract_result::<R>(
                        option.chain_id,
                        address,
                        option.start_block,
                        contract_results,
                    ))
                    .build(),
            );
        }
    }
    results
}

fn build_contract_result<R: LoadedData>(
    chain_id: u64,
    address: &String,
    start_block: u64,
    contract_results: &Vec<FetchContractResponse>,
) -> Result<ContractData<R>> {
    let mut actual_end_block: u64 = 0;
    let mut commitments: Vec<Commitment> = Vec::new();
    let mut nullifiers: Vec<Nullifier> = Vec::new();
    for contract_response in contract_results {
        commitments.extend_from_slice(&contract_response.commitments);
        nullifiers.extend_from_slice(&contract_response.nullifiers);
        actual_end_block = actual_end_block.max(contract_response.end_block);
    }
    commitments.sort_by_key(|commitment| commitment.block_number);

    let data = match R::data_type() {
        DataType::Full => {
            nullifiers.sort_by_key(|nullifier| nullifier.block_number);
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

impl<R, C> SequencerFetcher<R, C>
where
    R: LoadedData,
    C: SequencerClient<FetchChainRequest, FetchChainResponse>,
{
    pub fn new(client: Arc<C>) -> Self {
        Self::builder().client(client).build()
    }
}

impl<R, C> From<Arc<C>> for SequencerFetcher<R, C>
where
    R: LoadedData,
    C: SequencerClient<FetchChainRequest, FetchChainResponse>,
{
    fn from(client: Arc<C>) -> Self {
        Self::new(client)
    }
}
