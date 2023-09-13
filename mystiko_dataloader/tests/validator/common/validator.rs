use crate::validator::common::validator_checker_mock::{MockRuleChecker, RuleCheckerType};
use crate::validator::common::validator_handler_mock::MockChainDataHandler;
use crate::validator::common::validator_provider_mock::create_mock_providers;
use ethers_providers::{MockProvider, Provider as EthersProvider};
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::data::LiteData;
use mystiko_dataloader::validator::rule::{
    create_full_rule_validator, CounterChecker, IntegrityChecker, MerkleTreeChecker, RuleChecker, RuleValidator,
    RuleValidatorOptions, SequenceChecker,
};
use mystiko_protos::data::v1::{Commitment, Nullifier};
use std::sync::Arc;
use tokio::sync::RwLock;

type FullDataRuleValidator = RuleValidator<FullData, MockChainDataHandler<FullData>>;
type FullDataMockRuleValidator = RuleValidator<FullData, MockChainDataHandler<FullData>, MockRuleChecker<FullData>>;

pub fn create_single_rule_full_data_validator(
    rules: Option<Vec<RuleCheckerType>>,
) -> (
    FullDataRuleValidator,
    Arc<MockChainDataHandler<FullData>>,
    MockProvider,
    FullDataMockRuleValidator,
    Arc<MockRuleChecker<FullData>>,
) {
    let (_, mock) = EthersProvider::mocked();

    let handler = Arc::new(MockChainDataHandler::new());
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(providers);

    let rule_types = match rules {
        Some(rules) => rules,
        None => vec![
            RuleCheckerType::Sequence,
            RuleCheckerType::Counter,
            RuleCheckerType::Tree,
        ],
    };

    let checkers = rule_types
        .iter()
        .map(|t| match t {
            RuleCheckerType::Integrity => {
                let checker = IntegrityChecker::builder().build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<FullData>>)
            }
            RuleCheckerType::Sequence => {
                let checker = SequenceChecker::builder().handler(handler.clone()).build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<FullData>>)
            }
            RuleCheckerType::Counter => {
                let checker = CounterChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<FullData>>)
            }
            RuleCheckerType::Tree => {
                let checker = MerkleTreeChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<FullData>>)
            }
        })
        .collect::<Vec<_>>();

    let validator = RuleValidator::new(
        &RuleValidatorOptions::builder()
            .handler(handler.clone())
            .checkers(checkers)
            .build(),
    );

    let mock_checker = MockRuleChecker::builder().merged_data(RwLock::new(None)).build();
    let mock_checker = Arc::new(mock_checker);
    let validator_mock_rule = RuleValidator::new(
        &RuleValidatorOptions::builder()
            .handler(handler.clone())
            .checkers(vec![mock_checker.clone()])
            .build(),
    );

    (validator, handler, mock, validator_mock_rule, mock_checker)
}

pub fn create_full_rule_full_data_validator(
) -> (FullDataRuleValidator, Arc<MockChainDataHandler<FullData>>, MockProvider) {
    let (_, mock) = EthersProvider::mocked();
    let handler = Arc::new(MockChainDataHandler::new());
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(providers);
    let validator = create_full_rule_validator(handler.clone(), providers).unwrap();
    (validator, handler, mock)
}

type LiteDataRuleValidator = RuleValidator<LiteData, MockChainDataHandler<LiteData>>;

pub fn create_single_rule_lite_data_validator(
    rules: Option<Vec<RuleCheckerType>>,
) -> (LiteDataRuleValidator, Arc<MockChainDataHandler<LiteData>>, MockProvider) {
    let (_, mock) = EthersProvider::mocked();
    let handler = Arc::new(MockChainDataHandler::new());
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(providers);

    let rule_types = match rules {
        Some(rules) => rules,
        None => vec![
            RuleCheckerType::Sequence,
            RuleCheckerType::Counter,
            RuleCheckerType::Tree,
        ],
    };

    let checkers = rule_types
        .iter()
        .map(|t| match t {
            RuleCheckerType::Integrity => {
                let checker = IntegrityChecker::builder().build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<LiteData>>)
            }
            RuleCheckerType::Sequence => {
                let checker = SequenceChecker::builder().handler(handler.clone()).build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<LiteData>>)
            }
            RuleCheckerType::Counter => {
                let checker = CounterChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<LiteData>>)
            }
            RuleCheckerType::Tree => {
                let checker = MerkleTreeChecker::builder()
                    .providers(providers.clone())
                    .handler(handler.clone())
                    .build();
                Arc::new(Box::new(checker) as Box<dyn RuleChecker<LiteData>>)
            }
        })
        .collect::<Vec<_>>();
    let validator = RuleValidator::new(
        &RuleValidatorOptions::builder()
            .handler(handler.clone())
            .checkers(checkers)
            .build(),
    );

    (validator, handler, mock)
}

pub async fn load_commitments(file: &str) -> Vec<Commitment> {
    let bytes = tokio::fs::read(file).await.unwrap();
    let commitments: Vec<Commitment> = serde_json::from_slice(bytes.as_slice()).unwrap();
    commitments
}

pub async fn load_nullifiers(file: &str) -> Vec<Nullifier> {
    let bytes = tokio::fs::read(file).await.unwrap();
    let nullifiers: Vec<Nullifier> = serde_json::from_slice(bytes.as_slice()).unwrap();
    nullifiers
}
