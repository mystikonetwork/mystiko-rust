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

#[derive(Clone, Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
struct FetchContractOptions {
    pub(crate) chain_id: u64,
    pub(crate) contract_address: String,
    pub(crate) start_block: u64,
    pub(crate) target_block: u64,
    pub(crate) sequencer_fetch_size: u64,
}

#[async_trait]
impl<R, C> DataFetcher<R> for SequencerFetcher<R, C>
where
    R: LoadedData,
    C: SequencerClient<FetchChainRequest, FetchChainResponse>,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let chain_config = option.config.find_chain(option.chain_id).ok_or_else(|| {
            FetcherError::AnyhowError(anyhow!(SequencerFetcherError::UnsupportedChainError(option.chain_id)))
        })?;
        let sequencer_fetch_size = chain_config.sequencer_fetch_size().max(1);

        let contract_options = to_options(sequencer_fetch_size, option);
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(match contract_options {
                Some(options) => fetch_contracts::<R, C>(options, &self.client)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
                None => fetch_all::<R, C>(sequencer_fetch_size, option, &self.client)
                    .await
                    .map_err(FetcherError::AnyhowError)?,
            })
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

async fn fetch_contracts<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    options: Vec<FetchContractOptions>,
    client: &Arc<C>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut results = Vec::new();
    for option in options {
        let result = fetch_contract_result::<R, C>(&option, client).await?;
        results.push(result);
    }
    Ok(results)
}

async fn fetch_contract_result<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    option: &FetchContractOptions,
    client: &Arc<C>,
) -> Result<ContractResult<ContractData<R>>> {
    Ok(ContractResult::builder()
        .address(option.contract_address.to_string())
        .result(fetch_data(option, client).await)
        .build())
}

async fn fetch_data<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    option: &FetchContractOptions,
    client: &Arc<C>,
) -> Result<ContractData<R>> {
    let chain_id = option.chain_id;
    let address = &option.contract_address;
    let start_block = option.start_block;
    let contract_results = fetch_contract::<R, C>(option, client).await?;
    build_contract_result::<R>(chain_id, address, start_block, &contract_results)
}

async fn fetch_contract<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    option: &FetchContractOptions,
    client: &Arc<C>,
) -> Result<Vec<FetchContractResponse>> {
    let address = &option.contract_address;
    let mut contract_results = Vec::new();
    if option.start_block > option.target_block {
        return Err(anyhow::anyhow!(SequencerFetcherError::StartBlockTooBigError(
            option.start_block,
            option.target_block
        )));
    }
    let is_full = R::data_type() == DataType::Full;
    let mut start_block = option.start_block;
    let mut to_block;
    loop {
        to_block = option.target_block.min(start_block + option.sequencer_fetch_size - 1);
        let request = FetchChainRequest::builder()
            .chain_id(option.chain_id)
            .start_block(start_block)
            .target_block(to_block)
            .contracts(vec![FetchContractRequest::builder()
                .contract_address(string_address_to_bytes(address)?)
                .start_block(start_block)
                .target_block(to_block)
                .build()])
            .is_full(is_full)
            .build();
        let result = client.fetch_chain(request).await?;
        contract_results.extend(result.contracts);
        if to_block == option.target_block {
            break;
        }
        start_block = to_block + 1;
    }
    Ok(contract_results)
}

async fn fetch_all<R: LoadedData, C: SequencerClient<FetchChainRequest, FetchChainResponse>>(
    sequencer_fetch_size: u64,
    option: &FetchOptions,
    client: &Arc<C>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut contract_results: Vec<ContractResult<ContractData<R>>> = Vec::new();
    if option.start_block > option.target_block {
        return Err(anyhow::anyhow!(SequencerFetcherError::StartBlockTooBigError(
            option.start_block,
            option.target_block
        )));
    }
    let is_full = R::data_type() == DataType::Full;
    let mut start_block = option.start_block;
    let mut to_block;
    let mut contracts_results: Vec<FetchContractResponse> = Vec::new();
    loop {
        to_block = option.target_block.min(start_block + sequencer_fetch_size - 1);
        let request = FetchChainRequest::builder()
            .chain_id(option.chain_id)
            .start_block(start_block)
            .target_block(to_block)
            .contracts(vec![])
            .is_full(is_full)
            .build();
        let result = client.fetch_chain(request).await?;
        contracts_results.extend(result.contracts);
        if to_block == option.target_block {
            break;
        }
        start_block = to_block + 1;
    }
    let contract_result_map: HashMap<String, Vec<FetchContractResponse>> =
        contracts_results.into_iter().fold(HashMap::new(), |mut map, result| {
            let address = string_address_from_bytes(result.contract_address.clone());
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
                    option.chain_id,
                    address,
                    option.start_block,
                    a,
                ))
                .build(),
        );
    }
    Ok(contract_results)
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

fn to_options(sequencer_fetch_size: u64, option: &FetchOptions) -> Option<Vec<FetchContractOptions>> {
    option.contract_options.as_ref().map(|contract_options| {
        contract_options
            .iter()
            .map(|contract_option| {
                FetchContractOptions::builder()
                    .chain_id(option.chain_id)
                    .contract_address(contract_option.contract_config.address().to_string())
                    .start_block(contract_option.start_block.unwrap_or(option.start_block))
                    .target_block(contract_option.target_block.unwrap_or(option.target_block))
                    .sequencer_fetch_size(sequencer_fetch_size)
                    .build()
            })
            .collect::<Vec<FetchContractOptions>>()
    })
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
