use crate::loader::{MockFetcher, MockHandler, MockValidator};
use async_trait::async_trait;
use ethers_providers::{MockError, MockProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::fetcher::FetcherOptions;
use mystiko_dataloader::loader::{ChainDataLoader, FetcherWrapper};
use mystiko_ethers::{FailoverProvider, Provider, ProviderWrapper, Providers};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;

pub(crate) type ChainDataLoaderFullDataType =
    ChainDataLoader<FullData, MockHandler<FullData>, MockFetcher<FullData>, MockValidator<FullData>>;

pub(crate) async fn create_loader(
    chain_id: u64,
    fetcher_count: usize,
    validator_count: usize,
    skip_validation: bool,
) -> (
    Arc<MystikoConfig>,
    Arc<ChainDataLoaderFullDataType>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator<FullData>>>,
    Arc<MockHandler<FullData>>,
) {
    create_loader_with_priority(
        chain_id,
        fetcher_count,
        validator_count,
        skip_validation,
        vec![0; fetcher_count],
    )
    .await
}

pub(crate) async fn create_loader_with_priority(
    chain_id: u64,
    fetcher_count: usize,
    validator_count: usize,
    skip_validation: bool,
    priority: Vec<u32>,
) -> (
    Arc<MystikoConfig>,
    Arc<ChainDataLoaderFullDataType>,
    Vec<Arc<MockFetcher<FullData>>>,
    Vec<Arc<MockValidator<FullData>>>,
    Arc<MockHandler<FullData>>,
) {
    let core_cfg = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );

    let mut fetchers = vec![];
    let mut wrappers = vec![];
    for index in 0..fetcher_count {
        let fetcher = Arc::new(MockFetcher::new(chain_id));
        fetchers.push(fetcher.clone());
        wrappers.push(
            FetcherWrapper::builder()
                .fetcher(fetcher)
                .options(
                    FetcherOptions::builder()
                        .skip_validation(skip_validation)
                        .target_block_priority(priority[index])
                        .build(),
                )
                .build(),
        );
    }

    let validators = (0..validator_count)
        .map(|_| Arc::new(MockValidator::new()))
        .collect::<Vec<_>>();
    let handler = Arc::new(MockHandler::new());

    let loader = ChainDataLoaderFullDataType::builder()
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .fetchers(wrappers)
        .validators(validators.clone())
        .handler(handler.clone())
        .build();
    (core_cfg.clone(), Arc::new(loader), fetchers, validators, handler)
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
