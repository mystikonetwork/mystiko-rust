use crate::loader::{MockFetcher, MockHandler, MockValidator};
use async_trait::async_trait;
use ethers_core::types::U64;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::loader::{ChainDataLoader, DataLoaderResult};
use mystiko_dataloader::loader::{DataLoader, LoadOption};
use mystiko_ethers::{FailoverProvider, Provider, ProviderWrapper, Providers};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;

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

pub struct MockProviders {
    provider: Option<Arc<Provider>>,
}

impl MockProviders {
    fn new(provider: Option<Arc<Provider>>) -> Self {
        MockProviders { provider }
    }
}

impl Debug for MockProviders {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[async_trait]
impl Providers for MockProviders {
    async fn get_provider(&self, _chain_id: u64) -> anyhow::Result<Arc<Provider>> {
        self.provider
            .clone()
            .ok_or(anyhow::Error::msg("get_provider error".to_string()))
    }

    async fn has_provider(&self, _chain_id: u64) -> bool {
        false
    }

    async fn set_provider(&self, _chain_id: u64, _provider: Arc<Provider>) -> Option<Arc<Provider>> {
        None
    }

    async fn delete_provider(&self, _chain_id: u64) -> Option<Arc<Provider>> {
        None
    }
}

pub fn create_mock_providers(provider: Option<&MockProvider>) -> MockProviders {
    match provider {
        None => MockProviders::new(None),
        Some(provider) => {
            let provider = create_mock_provider(provider);
            MockProviders::new(Some(Arc::new(provider)))
        }
    }
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
