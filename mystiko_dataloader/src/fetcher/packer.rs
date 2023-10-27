use crate::data::{ChainResult, ContractData, ContractResult, DataType, FullData, LiteData, LoadedData};
use crate::fetcher::{
    ChainLoadedBlockOptions, ContractFetchOptions, DataFetcher, FetchOptions, FetchResult, FetcherError,
};
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mystiko_config::MystikoConfig;
use mystiko_datapacker_client::{ChainQuery, DataPackerClient};
use mystiko_protos::data::v1::ChainData;
use mystiko_utils::address::{ethers_address_from_bytes, ethers_address_from_string, ethers_address_to_string};
use std::cmp::min;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

const PACKER_FETCHER_NAME: &str = "packer";

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DataPackerFetcher<R: LoadedData, C: DataPackerClient<ChainData>> {
    client: Arc<C>,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[derive(Debug, Error)]
pub enum DataPackerFetcherError {
    #[error("client error raised error: {0}")]
    ClientError(anyhow::Error),
    #[error("no chain data found with options {0:?}")]
    NoChainDataError(ChainQuery),
    #[error("start block {0} is bigger than end block {1}")]
    StartBlockTooBigError(u64, u64),
    #[error("missing data of contract {0}")]
    MissingContractDataError(String),
}

pub type DataPackerFetcherV1<R> = DataPackerFetcher<R, mystiko_datapacker_client::v1::DataPackerClient>;

#[async_trait]
impl<R, C> DataFetcher<R> for DataPackerFetcher<R, C>
where
    R: LoadedData,
    C: DataPackerClient<ChainData>,
{
    fn name(&self) -> &'static str {
        PACKER_FETCHER_NAME
    }

    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        let query = to_chain_query(option);
        let chain_data = self.query_chain(query).await?;
        Ok(to_fetch_result::<R>(option, chain_data)?)
    }

    async fn chain_loaded_block(&self, options: &ChainLoadedBlockOptions) -> Result<u64, FetcherError> {
        Ok(self
            .client
            .query_chain_loaded_block(options.chain_id)
            .await
            .map_err(FetcherError::AnyhowError)?)
    }
}

impl<R, C> DataPackerFetcher<R, C>
where
    R: LoadedData,
    C: DataPackerClient<ChainData>,
{
    pub fn new(client: Arc<C>) -> Self {
        Self::builder().client(client).build()
    }

    async fn query_chain(&self, options: ChainQuery) -> Result<ChainData> {
        match self.client.query_chain(&options).await {
            Ok(result) => {
                if let Some(chain_data) = result.chain_data {
                    Ok(chain_data)
                } else {
                    Err(anyhow::anyhow!(DataPackerFetcherError::NoChainDataError(options)))
                }
            }
            Err(err) => {
                log::error!(
                    "failed to fetch chain data from packer with options {:?}: {:?}",
                    options,
                    err
                );
                Err(err)
            }
        }
    }
}

impl<R, C> From<Arc<C>> for DataPackerFetcher<R, C>
where
    R: LoadedData,
    C: DataPackerClient<ChainData>,
{
    fn from(client: Arc<C>) -> Self {
        Self::new(client)
    }
}

impl<R> From<Arc<MystikoConfig>> for DataPackerFetcherV1<R>
where
    R: LoadedData,
{
    fn from(config: Arc<MystikoConfig>) -> Self {
        let client = Arc::new(mystiko_datapacker_client::v1::DataPackerClient::new(config));
        Self::new(client)
    }
}

fn to_chain_query(option: &FetchOptions) -> ChainQuery {
    let start_block = option
        .contract_options
        .as_ref()
        .and_then(|contracts| {
            contracts
                .iter()
                .map(|c| c.start_block.unwrap_or(option.start_block))
                .min()
        })
        .unwrap_or(option.start_block);
    let target_block = option
        .contract_options
        .as_ref()
        .and_then(|contracts| {
            contracts
                .iter()
                .map(|c| c.target_block.unwrap_or(option.target_block))
                .max()
        })
        .unwrap_or(option.target_block);
    let query = ChainQuery::builder()
        .chain_id(option.chain_id)
        .target_block(target_block)
        .start_block(start_block)
        .build();
    log::info!("fetching packer with options {:?}", query);
    query
}

fn to_fetch_result<R: LoadedData>(
    options: &FetchOptions,
    chain_data: ChainData,
) -> Result<ChainResult<ContractData<R>>> {
    log::debug!(
        "fetched chain data from packer with chain_id {}, start_block {}, end_block {}, {} contracts",
        options.chain_id,
        chain_data.start_block,
        chain_data.end_block,
        chain_data.contract_data.len()
    );
    let start_block = chain_data.start_block;
    let end_block = chain_data.end_block;
    let contracts_options = to_contracts_options_map(options)?;
    let contracts_result_map = to_contract_result_map(options, start_block, end_block, chain_data, &contracts_options);
    Ok(build_result(options, &contracts_options, contracts_result_map))
}

fn to_contracts_options_map(options: &FetchOptions) -> Result<HashMap<Address, ContractFetchOptions>> {
    let mut contracts = HashMap::new();
    if let Some(contract_options) = options.contract_options.as_ref() {
        for contract_option in contract_options {
            let address = ethers_address_from_string(contract_option.contract_config.address())?;
            contracts.insert(address, contract_option.clone());
        }
    }
    Ok(contracts)
}

fn to_contract_result_map<R: LoadedData>(
    options: &FetchOptions,
    start_block: u64,
    end_block: u64,
    chain_data: ChainData,
    contracts_options: &HashMap<Address, ContractFetchOptions>,
) -> HashMap<Address, ContractResult<ContractData<R>>> {
    let mut contracts_result = HashMap::new();
    for contract_data in chain_data.contract_data.into_iter() {
        let address = ethers_address_from_bytes(&contract_data.contract_address);
        if contracts_options.is_empty() || contracts_options.contains_key(&address) {
            contracts_result.insert(
                address,
                to_contract_result::<R>(
                    &address,
                    options,
                    start_block,
                    end_block,
                    contract_data,
                    contracts_options,
                ),
            );
        }
    }
    contracts_result
}

fn to_contract_result<R: LoadedData>(
    address: &Address,
    options: &FetchOptions,
    start_block: u64,
    end_block: u64,
    contract_data: mystiko_protos::data::v1::ContractData,
    contracts_options: &HashMap<Address, ContractFetchOptions>,
) -> ContractResult<ContractData<R>> {
    let address_string = ethers_address_to_string(address);
    let contract_options = contracts_options.get(address);
    let expected_start_block = contract_options
        .map(|c| c.start_block.unwrap_or(options.start_block))
        .unwrap_or(options.start_block);
    let expected_target_block = contract_options
        .map(|c| c.target_block.unwrap_or(options.target_block))
        .unwrap_or(options.target_block);
    let loaded_block = min(end_block, expected_target_block);
    if start_block > expected_start_block {
        let error = DataPackerFetcherError::StartBlockTooBigError(start_block, expected_start_block);
        return ContractResult::builder()
            .address(address_string)
            .result(Err(anyhow::anyhow!(error)))
            .build();
    }
    log::debug!(
        "fetched contract data from packer with chain_id {}, contract_address {}, \
        start_block {}, end_block {}, {} commitments, {} nullifiers",
        options.chain_id,
        address_string,
        start_block,
        end_block,
        contract_data.commitments.len(),
        contract_data.nullifiers.len()
    );
    let commitments = contract_data
        .commitments
        .into_iter()
        .filter(|c| c.block_number >= expected_start_block && c.block_number <= expected_target_block)
        .collect::<Vec<_>>();
    let nullifiers = contract_data
        .nullifiers
        .into_iter()
        .filter(|n| n.block_number >= expected_start_block && n.block_number <= expected_target_block)
        .collect::<Vec<_>>();
    log::debug!(
        "filtered contract data from packer with chain_id {}, contract_address {}, \
        expected_start_block {}, expected_end_block {}, {} commitments, {} nullifiers",
        options.chain_id,
        address_string,
        expected_start_block,
        expected_target_block,
        commitments.len(),
        nullifiers.len()
    );
    let contract_data = match R::data_type() {
        DataType::Full => FullData::builder()
            .commitments(commitments)
            .nullifiers(nullifiers)
            .build()
            .into_data(),
        DataType::Lite => LiteData::builder().commitments(commitments).build().into_data(),
    };
    let contract_data = ContractData::<R>::builder()
        .address(address_string.clone())
        .start_block(expected_start_block)
        .end_block(loaded_block)
        .data(R::from_data(contract_data))
        .build();
    ContractResult::builder()
        .address(address_string)
        .result(Ok(contract_data))
        .build()
}

fn build_result<R: LoadedData>(
    options: &FetchOptions,
    contracts_options: &HashMap<Address, ContractFetchOptions>,
    mut contracts_result: HashMap<Address, ContractResult<ContractData<R>>>,
) -> ChainResult<ContractData<R>> {
    let contracts_result = if contracts_options.is_empty() {
        contracts_result.into_values().collect::<Vec<_>>()
    } else {
        contracts_options
            .keys()
            .map(|address| {
                let address_string = ethers_address_to_string(address);
                if let Some(contract_result) = contracts_result.remove(address) {
                    contract_result
                } else {
                    log::warn!(
                        "missing contract data from packer with chain_id {}, contract_address {}",
                        options.chain_id,
                        address_string
                    );
                    ContractResult::builder()
                        .address(address_string.clone())
                        .result(Err(anyhow::anyhow!(DataPackerFetcherError::MissingContractDataError(
                            address_string
                        ))))
                        .build()
                }
            })
            .collect::<Vec<_>>()
    };
    ChainResult::builder()
        .chain_id(options.chain_id)
        .contract_results(contracts_result)
        .build()
}
