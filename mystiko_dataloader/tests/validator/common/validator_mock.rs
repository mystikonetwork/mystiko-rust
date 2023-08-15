use async_trait::async_trait;
use ethers_providers::{MockError, MockProvider, Provider as EthersProvider, RetryClientBuilder, RetryPolicy};
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_dataloader::data::{ChainData, LiteData};
use mystiko_dataloader::data::{FullData, LoadedData};
use mystiko_dataloader::handler::HandlerError;
use mystiko_dataloader::handler::{
    CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption, QueryResult,
};
use mystiko_dataloader::validator::rule::counter::CounterChecker;
use mystiko_dataloader::validator::rule::sequence::SequenceChecker;
use mystiko_dataloader::validator::rule::tree::TreeChecker;
use mystiko_dataloader::validator::rule::{RuleChecker, RuleValidator, RuleValidatorInitParam};
use mystiko_ethers::provider::factory::Provider;
use mystiko_ethers::provider::failover::FailoverProvider;
use mystiko_ethers::provider::pool::Providers;
use mystiko_ethers::provider::wrapper::ProviderWrapper;
use mystiko_fs::read_file_bytes;
use mystiko_protos::data::v1::{Commitment, Nullifier};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct MockHandler<R> {
    pub _phantom: std::marker::PhantomData<R>,
    pub cms: RwLock<Vec<Vec<Commitment>>>,
    pub nullifiers: RwLock<Vec<Vec<Nullifier>>>,
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
            _phantom: std::marker::PhantomData,
            cms: RwLock::new(vec![]),
            nullifiers: RwLock::new(vec![]),
        }
    }

    pub async fn add_commitments(&self, commitments: Vec<Commitment>) {
        let mut cms = self.cms.write().await;
        cms.push(commitments);
    }

    pub async fn add_nullifiers(&self, nullifiers: Vec<Nullifier>) {
        let mut ns = self.nullifiers.write().await;
        ns.push(nullifiers);
    }
}

pub type HandlerErrorResult<T> = anyhow::Result<T, HandlerError>;

#[async_trait]
impl<R> DataHandler<R> for MockHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(&self, _chain_id: u64) -> HandlerErrorResult<Option<Vec<ContractConfig>>> {
        Ok(None)
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> HandlerErrorResult<Option<u64>> {
        Err(anyhow::Error::msg("query_chain_loaded_block error".to_string()).into())
    }

    async fn query_contract_loaded_block(
        &self,
        _chain_id: u64,
        _contract_address: &str,
    ) -> HandlerErrorResult<Option<u64>> {
        Err(anyhow::Error::msg("query_contract_loaded_block error".to_string()).into())
    }

    async fn query_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Commitment>>> {
        self.cms.write().await.pop().map_or_else(
            || Err(anyhow::Error::msg("mock query commitments error".to_string()).into()),
            |cms| Ok(QueryResult::builder().end_block(option.end_block).result(cms).build()),
        )
    }

    async fn count_commitments(&self, option: &CommitmentQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let mut query_result = self.query_commitments(option).await?;
        query_result.result.pop().map_or_else(
            || {
                Ok(QueryResult::builder()
                    .end_block(query_result.end_block)
                    .result(0_u64)
                    .build())
            },
            |cm| {
                Ok(QueryResult::builder()
                    .end_block(option.end_block)
                    .result(cm.leaf_index.unwrap() + 1)
                    .build())
            },
        )
    }

    async fn query_nullifiers(
        &self,
        _option: &NullifierQueryOption,
    ) -> HandlerErrorResult<QueryResult<Vec<Nullifier>>> {
        self.nullifiers.write().await.pop().map_or_else(
            || Err(anyhow::Error::msg("mock query nullifiers error".to_string()).into()),
            |ns| Ok(QueryResult::builder().end_block(0_u64).result(ns).build()),
        )
    }

    async fn count_nullifiers(&self, option: &NullifierQueryOption) -> HandlerErrorResult<QueryResult<u64>> {
        let query_result = self.query_nullifiers(option).await?;
        Ok(QueryResult::builder()
            .end_block(query_result.end_block)
            .result(query_result.result.len() as u64)
            .build())
    }

    async fn handle(&self, _data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        Err(anyhow::Error::msg("handle error".to_string()).into())
    }
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

struct MockProviders {
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
    fn get_provider(&self, _chain_id: u64) -> Option<Arc<Provider>> {
        self.provider.as_ref().cloned()
    }

    async fn get_or_create_provider(&mut self, _chain_id: u64) -> anyhow::Result<Arc<Provider>> {
        Err(anyhow::Error::msg("get_or_create_provider error".to_string()))
    }
}

fn create_mock_providers(provider: Option<&MockProvider>) -> MockProviders {
    match provider {
        None => MockProviders::new(None),
        Some(provider) => {
            let provider = create_mock_provider(provider);
            MockProviders::new(Some(Arc::new(provider)))
        }
    }
}
pub enum RuleCheckerType {
    Sequence,
    Counter,
    Tree,
}

type FullDataRuleValidator = RuleValidator<FullData, MockHandler<FullData>>;

pub fn create_full_data_validator(
    rules: Option<Vec<RuleCheckerType>>,
) -> (FullDataRuleValidator, Arc<MockHandler<FullData>>, MockProvider) {
    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(providers);
    let handler = Arc::new(MockHandler::new());
    let rule_types = match rules {
        Some(rules) => rules,
        None => vec![
            RuleCheckerType::Sequence,
            RuleCheckerType::Counter,
            RuleCheckerType::Tree,
        ],
    };

    let rules = rule_types
        .iter()
        .map(|t| match t {
            RuleCheckerType::Sequence => {
                let checker = SequenceChecker::builder().handler(handler.clone()).build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
            RuleCheckerType::Counter => {
                let checker = CounterChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
            RuleCheckerType::Tree => {
                let checker = TreeChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
        })
        .collect::<Vec<_>>();
    let validator = RuleValidator::new(
        &RuleValidatorInitParam::builder()
            .handler(handler.clone())
            .rules(rules)
            .build(),
    );

    (validator, handler, mock)
}

type LiteDataRuleValidator = RuleValidator<LiteData, MockHandler<LiteData>>;

pub fn create_lite_data_validator(
    rules: Option<Vec<RuleCheckerType>>,
) -> (LiteDataRuleValidator, Arc<MockHandler<LiteData>>, MockProvider) {
    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(providers);
    let handler = Arc::new(MockHandler::new());
    let rule_types = match rules {
        Some(rules) => rules,
        None => vec![
            RuleCheckerType::Sequence,
            RuleCheckerType::Counter,
            RuleCheckerType::Tree,
        ],
    };

    let rules = rule_types
        .iter()
        .map(|t| match t {
            RuleCheckerType::Sequence => {
                let checker = SequenceChecker::builder().handler(handler.clone()).build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
            RuleCheckerType::Counter => {
                let checker = CounterChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
            RuleCheckerType::Tree => {
                let checker = TreeChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker>)
            }
        })
        .collect::<Vec<_>>();
    let validator = RuleValidator::new(
        &RuleValidatorInitParam::builder()
            .handler(handler.clone())
            .rules(rules)
            .build(),
    );

    (validator, handler, mock)
}

pub async fn load_commitments(file: &str) -> Vec<Commitment> {
    let bytes = read_file_bytes(file).await.unwrap();
    let commitments: Vec<Commitment> = serde_json::from_slice(bytes.as_slice()).unwrap();
    commitments
}

pub async fn load_nullifiers(file: &str) -> Vec<Nullifier> {
    let bytes = read_file_bytes(file).await.unwrap();
    let nullifiers: Vec<Nullifier> = serde_json::from_slice(bytes.as_slice()).unwrap();
    nullifiers
}
