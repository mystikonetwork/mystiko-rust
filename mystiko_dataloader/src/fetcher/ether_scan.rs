use anyhow::Result;
use async_trait::async_trait;
use log::info;
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_etherscan_client::client::{EtherScanClient, Event, GetLogsOptions};
use thiserror::Error;
use typed_builder::TypedBuilder;
use crate::data::{ChainResult,ContractData,ContractResult,Data,DataType,FullData, LiteData,LoadedData};
use crate::fetcher::{DataFetcher,FetchOptions,FetchResult,FetcherError};
use mystiko_utils::convert::u256_to_bytes;
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};

#[derive(Error, Debug)]
pub enum EtherScanFetcherError {
    #[error("no chain config found for chain id: {0}")]
    ChainConfigNotFoundError(u64),
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct EtherScanFetcher<R> {
    ether_scan_client: EtherScanClient,
    #[builder(default, setter(skip))]
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataFetcher<R> for EtherScanFetcher<R>
where
    R: LoadedData,
{
    async fn fetch(&self, option: &FetchOptions) -> FetchResult<R> {
        info!(
            "chain={} start fetch data, start_block={}, target_block={}",
            option.chain_id, option.start_block, option.target_block
        );
        let current_block_num: u64 = self
            .ether_scan_client
            .get_block_number()
            .await
            .map_err(|err| FetcherError::AnyhowError(err.into()))?
            .as_u64();
        let options: Vec<GetLogsOptions> = to_options(option, current_block_num).map_err(FetcherError::AnyhowError)?;

        let contracts_response = match R::data_type() {
            DataType::Full => get_full_data::<R>(&self.ether_scan_client, options).await,
            DataType::Lite => get_lite_data::<R>(&self.ether_scan_client, options).await,
        }
        .map_err(FetcherError::AnyhowError)?;
        info!(
            "chain={} successfully fetch data, start_block={}, target_block={}",
            option.chain_id, option.start_block, option.target_block
        );
        Ok(ChainResult::builder()
            .chain_id(option.chain_id)
            .contract_results(contracts_response)
            .build())
    }
}

fn to_options(option: &FetchOptions, current_block_num: u64) -> Result<Vec<GetLogsOptions>> {
    option.contract_options.as_ref().map_or_else(
        || {
            let chain_config = option
                .config
                .find_chain(option.chain_id)
                .ok_or_else(|| EtherScanFetcherError::ChainConfigNotFoundError(option.chain_id))?;
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

async fn get_full_data<R: LoadedData>(
    client: &EtherScanClient,
    options: Vec<GetLogsOptions>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut result: Vec<ContractResult<ContractData<R>>> = Vec::new();
    for option in options {
        let commitments: Vec<Commitment> = fetch_commitments(client, &option).await?;
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
        let r = ContractData::builder()
            .address(&option.address)
            .start_block(option.from_block)
            .end_block(option.to_block)
            .data(R::from_data(Data::Full(full_data)))
            .build();
        let contract_result = ContractResult::builder().address(&option.address).result(Ok(r)).build();
        result.push(contract_result);
    }
    Ok(result)
}

async fn get_lite_data<R: LoadedData>(
    client: &EtherScanClient,
    options: Vec<GetLogsOptions>,
) -> Result<Vec<ContractResult<ContractData<R>>>> {
    let mut result: Vec<ContractResult<ContractData<R>>> = Vec::new();
    for option in options {
        let commitments = fetch_commitments(client, &option).await?;
        info!(
            "contract={} fetch {} commitments from {} to {}",
            option.address,
            &commitments.len(),
            option.from_block,
            option.to_block,
        );
        let lite_data = LiteData::builder().commitments(commitments).build();
        let r = ContractData::builder()
            .address(&option.address)
            .start_block(option.from_block)
            .end_block(option.to_block)
            .data(R::from_data(Data::Lite(lite_data)))
            .build();
        let contract_result = ContractResult::builder().address(&option.address).result(Ok(r)).build();
        result.push(contract_result);
    }
    Ok(result)
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
