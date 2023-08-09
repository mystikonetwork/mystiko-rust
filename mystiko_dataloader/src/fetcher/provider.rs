use crate::data::contract::ContractData;
use crate::data::result::ChainResult;
use crate::data::result::ContractResult;
use crate::data::types::LoadedData;
use crate::data::types::{Data, DataType};
use crate::data::types::{FullData, LiteData};
use crate::fetcher::types::LogPrefixOptions;
use crate::fetcher::types::{ContractFetchOption, DataFetcher, FetchOption, FetchResult};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ethers_contract::EthEvent;
use ethers_core::abi::Address;
use ethers_core::types::{BlockNumber, Filter, U64};
use ethers_providers::Middleware;
use log::{debug, error, info, warn};
use mystiko_abi::commitment_pool::{CommitmentIncludedFilter, CommitmentQueuedFilter, CommitmentSpentFilter};
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_ethers::provider::factory::Provider;
use mystiko_etherscan_client::log::{Log, LogMeta};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::u256_to_bytes;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct ProviderFetcherOptions {
    pub provider: Arc<Provider>,
    pub chain_id: u64,
}

pub struct ProviderFetcher<R> {
    chain_id: u64,
    provider: Arc<Provider>,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataFetcher<R> for ProviderFetcher<R>
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
                        "chain_id param in ChainFetchOption({}) is different from ProviderFetcher({})",
                        chain_option.chain_id, self.chain_id
                    );
                    return Err(anyhow!(format!(
                        "chain_id param in ChainFetchOption({}) is different from ProviderFetcher({})",
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
                    error!("param chain_id in options is inconsistent with that in ProviderFetcher");
                    return Err(anyhow!(
                        "param chain_id in options is inconsistent with that in ProviderFetcher"
                    ));
                }
                contract_fetch_options = contract_options.clone().to_owned();
            }
        }
        if contract_fetch_options.len() == 0 {
            warn!("Providerfetcher found ContractFetcherOptions is empty, will do nothing");
            return Err(anyhow!(
                "providerfetcher found ContractFetcherOptions is empty, will do nothing"
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

impl<R> ProviderFetcher<R>
where
    R: LoadedData,
{
    pub fn new(option: ProviderFetcherOptions) -> Self {
        ProviderFetcher {
            provider: option.provider,
            chain_id: option.chain_id,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn fetch_contracts(
        &self,
        options: &Vec<ContractFetchOption>,
    ) -> Result<Vec<ContractResult<ContractData<R>>>> {
        let current_block_num = self.provider.get_block_number().await?.as_u64();
        info!(
            "{} found current block number is {}",
            self.get_log_prefix(None),
            current_block_num
        );
        let mut contract_results = vec![];
        for option in options {
            debug!("{} start to fetch", self.get_log_prefix(Some(&option.address)));
            let contract_data: ContractData<R>;
            let actual_end_block = if option.target_block <= current_block_num {
                option.target_block
            } else {
                debug!("{} found current block number less than target_block({}), will use current_block_num instead of target_block", self.get_log_prefix(Some(&option.address)), option.target_block);
                current_block_num
            };
            let get_log_option = GetLogsOption::builder()
                .address(option.address.clone())
                .start_block(option.start_block)
                .actual_end_block(actual_end_block)
                .build();
            let commitments = self.fetch_contract_commitments(&get_log_option).await?;
            let log_option = LogPrefixOptions::builder()
                .address(get_log_option.address.clone())
                .from_block(get_log_option.start_block)
                .to_block(get_log_option.actual_end_block)
                .build();
            info!(
                "{} fetch {} commitments",
                self.get_log_prefix_with_block(&log_option),
                commitments.len()
            );
            match R::data_type() {
                DataType::Full => {
                    let nullifiers = self.fetch_contract_nullifiers(&get_log_option).await?;
                    info!(
                        "{} fetch {} nullifiers",
                        self.get_log_prefix_with_block(&log_option),
                        nullifiers.len()
                    );
                    let fulldata = FullData::builder()
                        .commitments(commitments)
                        .nullifiers(nullifiers)
                        .build();
                    contract_data = ContractData::builder()
                        .address(option.address.to_string())
                        .start_block(option.start_block)
                        .end_block(actual_end_block)
                        .data(R::from_data(Data::Full(fulldata)))
                        .build();
                }
                DataType::Lite => {
                    let litedata = LiteData::builder().commitments(commitments).build();
                    contract_data = ContractData::builder()
                        .address(option.address.to_string())
                        .start_block(option.start_block)
                        .end_block(actual_end_block)
                        .data(R::from_data(Data::Lite(litedata)))
                        .build();
                }
            }
            contract_results.push(
                ContractResult::builder()
                    .address(option.address.to_string())
                    .result(Ok(contract_data))
                    .build(),
            );
        }
        debug!(
            "{} fetch {} contract results",
            self.get_log_prefix(None),
            contract_results.len()
        );
        Ok(contract_results)
    }

    async fn fetch_contract_commitments(&self, option: &GetLogsOption) -> Result<Vec<Commitment>> {
        let log_option = LogPrefixOptions::builder()
            .address(option.address.clone())
            .from_block(option.start_block)
            .to_block(option.actual_end_block)
            .build();
        let crosschain_events = self.fetch_logs::<CommitmentCrossChainFilter>(&option).await?;
        info!(
            "{} fetch {} commitment_crosschain events",
            self.get_log_prefix_with_block(&log_option),
            crosschain_events.len()
        );
        let queued_events = self.fetch_logs::<CommitmentQueuedFilter>(&option).await?;
        info!(
            "{} fetch {} commitment_queued events",
            self.get_log_prefix_with_block(&log_option),
            queued_events.len()
        );
        let included_events = self.fetch_logs::<CommitmentIncludedFilter>(&option).await?;
        info!(
            "{} fetch {} commitment_included events",
            self.get_log_prefix_with_block(&log_option),
            included_events.len()
        );
        let commitments = wrap_commitments_from_events(CommitmentDataEvent {
            crosschain_events,
            queued_events,
            included_events,
        })
        .await?;
        Ok(commitments)
    }

    async fn fetch_contract_nullifiers(&self, option: &GetLogsOption) -> Result<Vec<Nullifier>> {
        let spent_events = self.fetch_logs::<CommitmentSpentFilter>(&option).await?;
        info!(
            "{} fetch {} commitment_included events",
            self.get_log_prefix_with_block(
                &LogPrefixOptions::builder()
                    .address(option.address.clone())
                    .from_block(option.start_block)
                    .to_block(option.actual_end_block)
                    .build()
            ),
            spent_events.len()
        );
        let nullifiers = wrap_nullifiers_from_events(NullifierDataEvent { spent_events }).await?;
        Ok(nullifiers)
    }

    async fn fetch_logs<E: EthEvent>(&self, option: &GetLogsOption) -> Result<Vec<Event<E>>> {
        let mut events: Vec<Event<E>> = vec![];
        let filter = Filter::new()
            .topic0(E::signature())
            .address(option.address.parse::<Address>().unwrap())
            .from_block(BlockNumber::Number(U64::from(option.start_block)))
            .to_block(BlockNumber::Number(U64::from(option.actual_end_block)));
        let logs = self.provider.get_logs(&filter).await?;
        debug!(
            "{} fetch {} {} logs",
            self.get_log_prefix_with_block(
                &LogPrefixOptions::builder()
                    .address(option.address.clone())
                    .from_block(option.start_block)
                    .to_block(option.actual_end_block)
                    .build()
            ),
            logs.len(),
            E::name()
        );
        for log in logs {
            let my_log = Log {
                address: log.address,
                topics: log.topics,
                data: log.data,
                block_hash: log.block_hash,
                block_number: log.block_number,
                transaction_hash: log.transaction_hash,
            };
            let metadata: LogMeta = (&my_log).into();
            let event = E::decode_log(&my_log.into_raw())?;
            events.push(Event { raw: event, metadata });
        }
        Ok(events)
    }

    fn get_log_prefix(&self, address: Option<&str>) -> String {
        if address.is_none() {
            format!("ProviderFetcher[type={:?}, chain_id={}]", R::data_type(), self.chain_id)
        } else {
            format!(
                "ProviderFetcher[type={:?}, chain_id={}, address={}]",
                R::data_type(),
                self.chain_id,
                address.unwrap()
            )
        }
    }

    fn get_log_prefix_with_block(&self, opt: &LogPrefixOptions) -> String {
        format!(
            "ProviderFetcher[type={:?}, chain_id={}, address={}, from_block={}, to_block={}]",
            R::data_type(),
            self.chain_id,
            opt.address,
            opt.from_block,
            opt.to_block
        )
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct CommitmentDataEvent {
    pub crosschain_events: Vec<Event<CommitmentCrossChainFilter>>,
    pub queued_events: Vec<Event<CommitmentQueuedFilter>>,
    pub included_events: Vec<Event<CommitmentIncludedFilter>>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct NullifierDataEvent {
    pub spent_events: Vec<Event<CommitmentSpentFilter>>,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Event<R> {
    pub raw: R,
    pub metadata: LogMeta,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct GetLogsOption {
    address: String,
    start_block: u64,
    actual_end_block: u64,
}

pub async fn wrap_commitments_from_events(events: CommitmentDataEvent) -> Result<Vec<Commitment>> {
    let CommitmentDataEvent {
        crosschain_events,
        queued_events,
        included_events,
    } = events;
    let mut commitments: Vec<Commitment> = Vec::new();
    crosschain_events.iter().for_each(|event| {
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
            .src_chain_transaction_hash(event.metadata.transaction_hash.as_bytes().to_vec())
            .build();
        commitments.push(commitment);
    });
    queued_events.iter().for_each(|event| {
        let commitment = Commitment::builder()
            .commitment_hash(u256_to_bytes(&event.raw.commitment))
            .status(CommitmentStatus::Queued)
            .block_number(event.metadata.block_number.as_u64())
            .included_block_number(None)
            .src_chain_block_number(None)
            .leaf_index(event.raw.leaf_index.as_u64())
            .rollup_fee(u256_to_bytes(&event.raw.rollup_fee))
            .encrypted_note(event.raw.encrypted_note.to_vec())
            .queued_transaction_hash(event.metadata.transaction_hash.as_bytes().to_vec())
            .included_transaction_hash(None)
            .src_chain_transaction_hash(None)
            .build();
        commitments.push(commitment);
    });
    included_events.iter().for_each(|event| {
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
            .included_transaction_hash(event.metadata.transaction_hash.as_bytes().to_vec())
            .src_chain_transaction_hash(None)
            .build();
        commitments.push(commitment);
    });
    commitments.sort_by(|a, b| a.block_number.cmp(&b.block_number));
    Ok(commitments)
}

pub async fn wrap_nullifiers_from_events(events: NullifierDataEvent) -> Result<Vec<Nullifier>> {
    let NullifierDataEvent { spent_events } = events;
    let mut nullifiers: Vec<Nullifier> = Vec::new();
    spent_events.iter().for_each(|event| {
        let nullifier = Nullifier::builder()
            .transaction_hash(event.metadata.transaction_hash.as_bytes().to_vec())
            .block_number(event.metadata.block_number.as_u64())
            .nullifier(u256_to_bytes(&event.raw.serial_number))
            .build();
        nullifiers.push(nullifier);
    });
    nullifiers.sort_by(|a, b| a.block_number.cmp(&b.block_number));
    return Ok(nullifiers);
}
