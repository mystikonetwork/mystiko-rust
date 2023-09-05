use async_trait::async_trait;
use ethers_core::types::U64;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::{ChainResult, ContractResult};
use mystiko_dataloader::data::{FullData, LoadedData};
use mystiko_dataloader::fetcher::{DataFetcher, FetchOptions, FetchResult};
use mystiko_dataloader::handler::HandlerError;
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption, QueryResult,
};
use mystiko_dataloader::loader::{ChainDataLoader, DataLoaderResult};
use mystiko_dataloader::loader::{DataLoader, LoadOption};
use mystiko_dataloader::validator::{DataValidator, ValidateOption, ValidateResult};
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct MockFetcher<R>
where
    R: LoadedData,
{
    pub chain_id: u64,
    pub result: RwLock<Option<ChainData<R>>>,
}

impl<R> MockFetcher<R>
where
    R: LoadedData + Clone,
{
    pub fn new(chain_id: u64) -> Self {
        MockFetcher {
            chain_id,
            result: RwLock::new(None),
        }
    }

    pub async fn set_result(&self, r: ChainData<R>) {
        *self.result.write().await = Some(r);
    }

    pub async fn set_error_result(&self) {
        *self.result.write().await = None;
    }
}

#[async_trait]
impl<R> DataFetcher<R> for MockFetcher<R>
where
    R: LoadedData + Clone,
{
    async fn fetch(&self, _option: &FetchOptions) -> FetchResult<R> {
        if let Some(ref result) = *self.result.read().await {
            let contract_results = result
                .contracts_data
                .clone()
                .into_iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(ContractData::builder()
                            .address(d.address)
                            .start_block(d.start_block)
                            .end_block(d.end_block)
                            .build()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(self.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            Err(anyhow::Error::msg("error".to_string()).into())
        }
    }
}

pub struct MockValidator {
    all_success: RwLock<bool>,
    result: RwLock<ValidateResult>,
}

impl Default for MockValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl MockValidator {
    pub fn new() -> Self {
        MockValidator {
            all_success: RwLock::new(true),
            result: RwLock::new(ValidateResult::Err(anyhow::Error::msg("error".to_string()).into())),
        }
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: ValidateResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }
}

#[async_trait]
impl<R> DataValidator<R> for MockValidator
where
    R: LoadedData,
{
    async fn validate(&self, data: &ChainData<R>, _option: &ValidateOption) -> ValidateResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("error".to_string()).into()),
            }
        }
    }
}

pub struct MockHandler<R>
where
    R: LoadedData,
{
    contracts: RwLock<Vec<ContractConfig>>,
    chain_loaded_blocks_error: RwLock<bool>,
    contract_loaded_blocks_error: RwLock<HashMap<String, bool>>,
    all_success: RwLock<bool>,
    result: RwLock<HandleResult>,
    data: RwLock<HashMap<String, ContractData<R>>>,
}

impl<R> Default for MockHandler<R>
where
    R: LoadedData + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> MockHandler<R>
where
    R: LoadedData + Clone,
{
    pub fn new() -> Self {
        MockHandler {
            contracts: RwLock::new(vec![]),
            chain_loaded_blocks_error: RwLock::new(false),
            contract_loaded_blocks_error: RwLock::new(HashMap::new()),
            all_success: RwLock::new(true),
            result: RwLock::new(HandleResult::Err(anyhow::Error::msg("handle error".to_string()).into())),
            data: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_contracts(&self, chain_id: u64, contracts: HashSet<&str>, core_cfg: Arc<MystikoConfig>) {
        let c = core_cfg
            .find_chain(chain_id)
            .unwrap()
            .contracts_with_disabled()
            .into_iter()
            .filter(|c| contracts.contains(c.address()))
            .collect::<Vec<_>>();

        *self.contracts.write().await = c;
    }

    pub async fn set_all_success(&self) {
        *self.all_success.write().await = true;
    }

    pub async fn set_result(&self, result: HandleResult) {
        *self.all_success.write().await = false;
        *self.result.write().await = result;
    }

    pub async fn drain_data(&self) -> HashMap<String, ContractData<R>> {
        let mut data_write = self.data.write().await;
        let mut drained_data = HashMap::new();
        data_write.drain().for_each(|(k, v)| {
            drained_data.insert(k, v);
        });
        drained_data
    }
}

pub type HandlerErrorResult<T> = anyhow::Result<T, HandlerError>;

#[async_trait]
impl<R> DataHandler<R> for MockHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> HandlerErrorResult<Option<Vec<ContractConfig>>> {
        if self.contracts.read().await.is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.contracts.read().await.clone()))
        }
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> HandlerErrorResult<Option<u64>> {
        if *self.chain_loaded_blocks_error.read().await {
            return Err(anyhow::Error::msg("error".to_string()).into());
        }

        let data = self.data.read().await;
        let min_end_block = data.iter().map(|(_, v)| v.end_block).min();

        Ok(min_end_block)
    }

    async fn query_contract_loaded_block(
        &self,
        _chain_id: u64,
        contract_address: &str,
    ) -> HandlerErrorResult<Option<u64>> {
        if let Some(blocks) = self.contract_loaded_blocks_error.read().await.get(contract_address) {
            if *blocks {
                return Err(anyhow::Error::msg("error".to_string()).into());
            }
        }

        let end_block = self
            .data
            .read()
            .await
            .get(contract_address)
            .map(|d| d.end_block)
            .unwrap_or(0_u64);

        Ok(Some(end_block))
    }

    async fn query_commitments(
        &self,
        _option: &CommitmentQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Commitment>>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()).into())
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_commitments(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn query_nullifiers(
        &self,
        _option: &NullifierQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Nullifier>>> {
        Err(anyhow::Error::msg("query_commitments error".to_string()).into())
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_nullifiers(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn handle(&self, data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        if *self.all_success.read().await {
            let contract_results = data
                .contracts_data
                .iter()
                .map(|d| {
                    ContractResult::builder()
                        .address(d.address.clone())
                        .result(Ok(()))
                        .build()
                })
                .collect::<Vec<_>>();

            let mut data_write = self.data.write().await;
            data.contracts_data.iter().for_each(|d| {
                data_write.insert(
                    d.address.clone(),
                    ContractData::builder()
                        .address(d.address.clone())
                        .start_block(d.start_block)
                        .end_block(d.end_block)
                        .build(),
                );
            });

            Ok(ChainResult::builder()
                .chain_id(data.chain_id)
                .contract_results(contract_results)
                .build())
        } else {
            let result = self.result.read().await;
            match &*result {
                Ok(result) => {
                    let contract_results = result
                        .contract_results
                        .iter()
                        .map(|d| match d.result {
                            Ok(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Ok(()))
                                .build(),
                            Err(_) => ContractResult::builder()
                                .address(d.address.clone())
                                .result(Err(anyhow::Error::msg("error".to_string())))
                                .build(),
                        })
                        .collect::<Vec<_>>();

                    let mut data_write = self.data.write().await;
                    data.contracts_data.iter().for_each(|d| {
                        data_write.insert(
                            d.address.clone(),
                            ContractData::builder()
                                .address(d.address.clone())
                                .start_block(d.start_block)
                                .end_block(d.end_block)
                                .build(),
                        );
                    });

                    Ok(ChainResult::builder()
                        .chain_id(result.chain_id)
                        .contract_results(contract_results)
                        .build())
                }
                Err(_) => Err(anyhow::Error::msg("handle error".to_string()).into()),
            }
        }
    }
}

pub type ChainDataLoaderFullDataType =
    ChainDataLoader<FullData, Provider, MockFetcher<FullData>, MockValidator, MockHandler<FullData>>;

pub async fn loader_load(loader: Arc<ChainDataLoaderFullDataType>, delay_block: Option<u64>) -> DataLoaderResult<()> {
    let load_option = delay_block.map(|d| LoadOption::builder().delay_block(d).build());
    loader.load(load_option).await
}

pub async fn create_loader(fetch_result: bool, contract_address: &str, end_block: u64) -> ChainDataLoaderFullDataType {
    let chain_id = 1_u64;
    let fetcher = MockFetcher::new(chain_id);
    if fetch_result {
        fetcher
            .set_result(
                ChainData::builder()
                    .chain_id(chain_id)
                    .contracts_data(vec![ContractData::builder()
                        .address(contract_address.to_string())
                        .start_block(1_u64)
                        .end_block(end_block)
                        .build()])
                    .build(),
            )
            .await;
    }

    let validator = MockValidator::new();
    let handler = MockHandler::new();

    let (_, mock) = EthersProvider::mocked();
    let provider = create_mock_provider(&mock);
    let provider = Arc::new(provider);

    let block_number = U64::from(10000);
    mock.push(block_number).unwrap();

    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();

    ChainDataLoaderFullDataType::builder()
        .chain_id(1_u64)
        .config(Arc::new(core_cfg))
        .provider(provider)
        .fetchers(vec![Arc::new(fetcher)])
        .validators(vec![Arc::new(validator)])
        .handler(handler)
        .build()
}

pub async fn create_shared_loader(
    chain_id: u64,
    feature_count: usize,
    validator_count: usize,
) -> (
    Arc<MystikoConfig>,
    Arc<ChainDataLoaderFullDataType>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator>>,
    Arc<MockHandler<FullData>>,
    Arc<MockProvider>,
) {
    let core_cfg = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );

    let fetchers = (0..feature_count)
        .map(|_| Arc::new(MockFetcher::new(chain_id)))
        .collect::<Vec<_>>();
    let validators = (0..validator_count)
        .map(|_| Arc::new(MockValidator::new()))
        .collect::<Vec<_>>();
    let handler = Arc::new(MockHandler::new());

    let (_, mock) = EthersProvider::mocked();
    let provider = create_mock_provider(&mock);
    let provider = Arc::new(provider);

    let loader = ChainDataLoaderFullDataType::builder()
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .provider(provider.clone())
        .fetchers(fetchers.clone())
        .validators(validators.clone())
        .handler(handler.clone())
        .build();
    (
        core_cfg.clone(),
        Arc::new(loader),
        fetchers,
        validators,
        handler,
        Arc::new(mock),
    )
}

#[derive(Debug, Default)]
struct MockProviderRetryPolicy;

impl RetryPolicy<MockError> for MockProviderRetryPolicy {
    fn should_retry(&self, _error: &MockError) -> bool {
        false
    }

    fn backoff_hint(&self, _error: &MockError) -> Option<Duration> {
        Duration::from_secs(10).into()
    }
}

fn create_mock_provider(provider: &MockProvider) -> Provider {
    let retry_provider_builder = RetryClientBuilder::default();
    let inner_provider = retry_provider_builder.build(provider.clone(), Box::<MockProviderRetryPolicy>::default());

    let mut provider_builder = FailoverProvider::dyn_rpc();
    provider_builder = provider_builder.add_provider(Box::new(inner_provider));
    Provider::new(ProviderWrapper::new(Box::new(provider_builder.build())))
}

pub fn contract_data_partial_eq(a: &HashMap<String, ContractData<FullData>>, b: &Vec<ContractData<FullData>>) -> bool {
    if a.len() != b.len() {
        println!("a.len() != b.len() {:?} {:?}", a.len(), b.len());
        return false;
    }

    for d in b.iter() {
        if let Some(v) = a.get(&d.address) {
            if v.address != d.address {
                println!("v.address != d.address");
                return false;
            }

            if v.start_block != d.start_block {
                println!("v.start_block != d.start_block");
                return false;
            }

            if v.end_block != d.end_block {
                println!("v.end_block != d.end_block");
                return false;
            }
        } else {
            println!("Address not found: {}", &d.address);
            return false;
        }
    }

    true
}
