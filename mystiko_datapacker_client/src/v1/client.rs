use crate::{ChainQuery, ChainResponse};
use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_datapacker_common::v1::PathSchema as PathSchemaV1;
use mystiko_datapacker_common::{CheckSum, Compression, PathSchema, Sha512CheckSum, ZstdCompression};
use mystiko_protos::data::v1::{ChainData, ContractData};
use mystiko_types::{PackerChecksum, PackerCompression};
use prost::Message;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub const DEFAULT_PACKER_V1_URL: &str = "https://static.mystiko.network/packer/v1";

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DataPackerClient<P = Box<dyn PathSchema>, X = Box<dyn Compression>, C = Box<dyn CheckSum>> {
    url: String,
    path_schema: P,
    compression: X,
    checksum: C,
    http_client: reqwest::Client,
    config: Arc<MystikoConfig>,
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct DataPackerClientOptions {
    #[builder(default, setter(strip_option))]
    url: Option<String>,
    #[builder(default, setter(strip_option))]
    path_schema: Option<Box<dyn PathSchema>>,
    #[builder(default, setter(strip_option))]
    compression: Option<Box<dyn Compression>>,
    #[builder(default, setter(strip_option))]
    checksum: Option<Box<dyn CheckSum>>,
    #[builder(default, setter(strip_option))]
    http_client: Option<reqwest::Client>,
    config: Arc<MystikoConfig>,
}

#[derive(Debug, Error)]
pub enum DataPackerClientError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    DecodeError(#[from] prost::DecodeError),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    FromHexError(#[from] rustc_hex::FromHexError),
    #[error("HTTP error: {0}")]
    HttpError(reqwest::StatusCode),
    #[error("Decompression error: {0}")]
    DecompressionError(anyhow::Error),
    #[error("Bad query: {0}")]
    BadQueryError(String),
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainError(u64),
    #[error("Missing data from block {0} to {1}")]
    MissingDataError(u64, u64),
}

impl DataPackerClient {
    pub fn new<O>(options: O) -> Self
    where
        O: Into<DataPackerClientOptions>,
    {
        let options: DataPackerClientOptions = options.into();
        let packer_config = options.config.packer();
        let url = options
            .url
            .or(packer_config.map(|c| c.url().to_string()))
            .unwrap_or(DEFAULT_PACKER_V1_URL.to_string());
        let path_schema = options.path_schema.unwrap_or(Box::<PathSchemaV1>::default());
        let compression = options
            .compression
            .or(packer_config.map(|c| match c.compression() {
                PackerCompression::Zstd => Box::<ZstdCompression>::default() as Box<dyn Compression>,
            }))
            .unwrap_or(Box::<ZstdCompression>::default());
        let checksum = options
            .checksum
            .or(packer_config.map(|c| match c.checksum() {
                PackerChecksum::Sha512 => Box::<Sha512CheckSum>::default() as Box<dyn CheckSum>,
            }))
            .unwrap_or(Box::<Sha512CheckSum>::default());
        let http_client = options.http_client.unwrap_or(Default::default());
        DataPackerClient {
            url,
            path_schema,
            compression,
            checksum,
            http_client,
            config: options.config,
        }
    }
}

impl From<Arc<MystikoConfig>> for DataPackerClientOptions {
    fn from(config: Arc<MystikoConfig>) -> Self {
        DataPackerClientOptions::builder().config(config).build()
    }
}

#[async_trait]
impl<P, X, C> crate::DataPackerClient<ChainData> for DataPackerClient<P, X, C>
where
    P: PathSchema,
    X: Compression,
    C: CheckSum,
{
    async fn query_chain(&self, query: &ChainQuery) -> anyhow::Result<ChainResponse<ChainData>> {
        if let Some(chain_config) = self.config.find_chain(query.chain_id) {
            let initial_block = chain_config.start_block();
            let start_block = query.start_block.unwrap_or(initial_block + 1);
            if start_block <= initial_block {
                return Err(anyhow::anyhow!(DataPackerClientError::BadQueryError(format!(
                    "start_block {} must be greater than initial_block {}",
                    start_block, initial_block
                ))));
            }
            let target_block = query.target_block;
            if target_block < start_block {
                return Err(anyhow::anyhow!(DataPackerClientError::BadQueryError(format!(
                    "target_block {} must be greater than start_block {}",
                    target_block, start_block
                ))));
            }
            let mut plans = create_plan(
                start_block,
                target_block,
                initial_block,
                chain_config.granularities().to_vec(),
            );
            let mut chain_data_list: Vec<ChainData> = vec![];
            let mut current_plan = plans.pop();
            while let Some(plan) = &current_plan {
                let mut next_plan = plans.pop();
                chain_data_list.extend(self.query_granularity(query, plan, &mut next_plan).await?);
                current_plan = next_plan;
            }
            let chain_data = merge_chain_data(query, initial_block, chain_data_list)?;
            if let Some(chain_data) = &chain_data {
                if chain_data.start_block > start_block {
                    return Err(anyhow::anyhow!(DataPackerClientError::MissingDataError(
                        start_block,
                        chain_data.start_block - 1,
                    )));
                }
            }
            Ok(ChainResponse::builder()
                .chain_id(query.chain_id)
                .chain_data(chain_data)
                .build())
        } else {
            Err(anyhow::anyhow!(DataPackerClientError::UnsupportedChainError(
                query.chain_id
            )))
        }
    }
}

impl<P, X, C> DataPackerClient<P, X, C>
where
    P: PathSchema,
    X: Compression,
    C: CheckSum,
{
    async fn query_granularity(
        &self,
        query: &ChainQuery,
        plan: &GranularityPlan,
        next_plan: &mut Option<GranularityPlan>,
    ) -> Result<Vec<ChainData>> {
        let block_promises = plan
            .start_blocks
            .iter()
            .map(|start_block| self.query_granularity_block(query, plan.granularity, *start_block))
            .collect::<Vec<_>>();
        let mut chain_data_list: Vec<ChainData> = vec![];
        for (start_block, chain_data) in futures::future::try_join_all(block_promises).await?.into_iter() {
            if let Some(chain_data) = chain_data {
                chain_data_list.push(chain_data);
            } else if let Some(next_plan) = next_plan.as_mut() {
                let factor = plan.granularity / next_plan.granularity;
                for i in 0..factor {
                    next_plan.start_blocks.push(start_block + i * next_plan.granularity);
                }
                next_plan.start_blocks.sort();
            }
        }
        Ok(chain_data_list)
    }

    async fn query_granularity_block(
        &self,
        query: &ChainQuery,
        granularity: u64,
        start_block: u64,
    ) -> Result<(u64, Option<ChainData>)> {
        log::debug!(
            "datapacker client query chain {}, granularity {}, start_block {}",
            query.chain_id,
            granularity,
            start_block
        );
        let data_checksum_url = format!(
            "{}{}",
            self.url,
            self.path_schema
                .checksum_path(query.chain_id, granularity, start_block)
                .to_string_lossy()
        );
        if let Some(checksum_bytes) = self.http_get(&data_checksum_url).await? {
            let checksum = String::from_utf8(checksum_bytes)?;
            let data_url = format!(
                "{}{}",
                self.url,
                self.path_schema
                    .data_path(query.chain_id, granularity, start_block)
                    .to_string_lossy()
            );
            if let Some(data_bytes) = self.http_get(&data_url).await? {
                let calculated_checksum = self.checksum.checksum(&data_bytes);
                if checksum == calculated_checksum {
                    let decompressed_data = self
                        .compression
                        .decompress(&data_bytes)
                        .await
                        .map_err(DataPackerClientError::DecompressionError)?;
                    let chain_data = ChainData::decode(&mut Cursor::new(&decompressed_data))?;
                    if chain_data.start_block == start_block && chain_data.end_block == start_block + granularity - 1 {
                        return Ok((start_block, Some(chain_data)));
                    }
                    log::debug!(
                        "datapacker client corrupted data from chain {}, granularity {}, \
                        expected block range [{}, {}] vs actual block range [{}, {}]",
                        query.chain_id,
                        granularity,
                        start_block,
                        start_block + granularity - 1,
                        chain_data.start_block,
                        chain_data.end_block,
                    )
                } else {
                    log::debug!(
                        "datapacker client checksum mismatch from chain {}, granularity {}, start_block {}: \
                        expected {} vs actual {} ",
                        query.chain_id,
                        granularity,
                        start_block,
                        checksum,
                        calculated_checksum,
                    );
                }
            } else {
                log::debug!(
                    "datapacker client missing data from chain {}, granularity {}, start_block {}",
                    query.chain_id,
                    granularity,
                    start_block
                );
            }
        } else {
            log::debug!(
                "datapacker client missing checksum from chain {}, granularity {}, start_block {}",
                query.chain_id,
                granularity,
                start_block
            );
        }
        Ok((start_block, None))
    }

    async fn http_get(&self, url: &str) -> Result<Option<Vec<u8>>> {
        log::debug!("datapacker client http GET request: {}", url);
        let response = self.http_client.get(url).send().await?;
        log::debug!("datapacker client http GET response status: {}", response.status());
        if response.status().is_client_error() {
            Ok(None)
        } else if response.status().is_success() {
            let bytes = response.bytes().await?;
            Ok(Some(bytes.to_vec()))
        } else {
            Err(anyhow::anyhow!(DataPackerClientError::HttpError(response.status())))
        }
    }
}

#[derive(Debug, TypedBuilder)]
struct GranularityPlan {
    pub(crate) granularity: u64,
    pub(crate) start_blocks: Vec<u64>,
}

fn create_plan(
    start_block: u64,
    target_block: u64,
    initial_block: u64,
    mut granularities: Vec<u64>,
) -> Vec<GranularityPlan> {
    granularities.sort();
    let mut granularities_indexes: Vec<Vec<u64>> = vec![];
    for (index, granularity) in granularities.iter().enumerate() {
        if index == 0 {
            let start_index = (start_block - initial_block - 1) / granularity;
            let end_index = (target_block - initial_block - 1) / granularity;
            let num_blocks = end_index - start_index + 1;
            let mut granularity_indexes: Vec<u64> = vec![];
            for i in 0..num_blocks {
                granularity_indexes.push(start_index + i);
            }
            granularities_indexes.push(granularity_indexes);
        } else {
            let last_granularity = granularities[index - 1];
            let factor = granularity / last_granularity;
            let last_granularity_indexes = granularities_indexes.remove(index - 1);
            let num_last_granularities: u64 = last_granularity_indexes.len() as u64;
            let mut new_last_granularity_indexes: Vec<u64> = vec![];
            let mut granularity_indexes: Vec<u64> = vec![];
            let mut i: u64 = 0;
            while i < num_last_granularities {
                let last_granularity_index = last_granularity_indexes[i as usize];
                if last_granularity_index % factor == 0 && i + factor <= num_last_granularities {
                    granularity_indexes.push(last_granularity_index / factor);
                    i += factor;
                } else {
                    new_last_granularity_indexes.push(last_granularity_index);
                    i += 1;
                }
            }
            granularities_indexes.push(new_last_granularity_indexes);
            granularities_indexes.push(granularity_indexes);
        }
    }
    granularities_indexes
        .into_iter()
        .enumerate()
        .map(|(i, granularity_indexes)| {
            GranularityPlan::builder()
                .granularity(granularities[i])
                .start_blocks(
                    granularity_indexes
                        .into_iter()
                        .map(|granularity_index| granularities[i] * granularity_index + initial_block + 1)
                        .collect::<Vec<_>>(),
                )
                .build()
        })
        .collect()
}

fn merge_chain_data(
    query: &ChainQuery,
    initial_block: u64,
    mut chain_data_list: Vec<ChainData>,
) -> Result<Option<ChainData>> {
    chain_data_list.sort_by_key(|d| d.start_block);
    let mut merged_chain_data: Option<ChainData> = None;
    for next_chain_data in chain_data_list.into_iter() {
        if let Some(chain_data) = merged_chain_data.as_mut() {
            if next_chain_data.start_block == chain_data.end_block + 1 {
                chain_data.end_block = next_chain_data.end_block;
                chain_data.contract_data.extend(next_chain_data.contract_data);
            }
        } else {
            merged_chain_data = Some(next_chain_data);
        }
    }
    merged_chain_data
        .map(|chain_data| merge_contracts_data(query, initial_block, chain_data))
        .transpose()
}

fn merge_contracts_data(query: &ChainQuery, initial_block: u64, chain_data: ChainData) -> Result<ChainData> {
    let start_block = query.start_block.unwrap_or(initial_block + 1);
    let target_block = query.target_block;
    let mut contracts_data: HashMap<Address, ContractData> = HashMap::new();
    for mut contract_data in chain_data.contract_data.into_iter() {
        let contract_address = Address::from_slice(&contract_data.contract_address);
        contract_data.commitments = contract_data
            .commitments
            .into_iter()
            .filter(|c| c.block_number <= target_block && c.block_number >= start_block)
            .collect::<Vec<_>>();
        contract_data.nullifiers = contract_data
            .nullifiers
            .into_iter()
            .filter(|n| n.block_number <= target_block && n.block_number >= start_block)
            .collect::<Vec<_>>();
        if let Some(existing_contract_data) = contracts_data.get_mut(&contract_address) {
            existing_contract_data.commitments.extend(contract_data.commitments);
            existing_contract_data.nullifiers.extend(contract_data.nullifiers);
            existing_contract_data.commitments.sort_by_key(|c| c.block_number);
            existing_contract_data.nullifiers.sort_by_key(|n| n.block_number);
        } else {
            contracts_data.insert(contract_address, contract_data);
        }
    }
    Ok(ChainData {
        start_block: max(chain_data.start_block, start_block),
        end_block: min(chain_data.end_block, target_block),
        contract_data: contracts_data.into_values().collect(),
    })
}
