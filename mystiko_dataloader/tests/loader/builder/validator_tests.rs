use crate::loader::{create_mock_providers, MockFetcher, MockHandler, MockValidator};
use ethers_core::types::{Log, U64};
use ethers_providers::Provider as EthersProvider;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::fetcher::DataFetcher;
use mystiko_dataloader::handler::DataHandler;
use mystiko_dataloader::loader::LoadOption;
use mystiko_dataloader::loader::{ChainDataLoader, DataLoader, FromConfig, LoaderConfigOptions};
use mystiko_dataloader::validator::DataValidator;
use mystiko_dataloader::DataLoaderError;
use mystiko_ethers::Providers;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::loader::v1::{
    LoaderConfig, RuleValidatorCheckerType, RuleValidatorConfig, ValidatorConfig, ValidatorType,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[tokio::test]
async fn test_create_chain_data_loader_rule_validator() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", log::LevelFilter::Debug)
        .try_init();

    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let mut validators = HashMap::new();
    validators.insert(0, ValidatorType::Rule as i32);
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .validators(validators.clone())
        .build();
    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;
    let handler = Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>);

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers.clone())
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());

    let block_number = U64::from(16691439);
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push(block_number).unwrap();
    mock.push(block_number).unwrap();
    let loader = loader.unwrap();
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(result.is_ok());

    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .validators(validators.clone())
        .validator_config(
            ValidatorConfig::builder()
                .rule({
                    let mut checkers = HashMap::new();
                    checkers.insert(0, RuleValidatorCheckerType::Counter as i32);
                    RuleValidatorConfig::builder().checkers(checkers.clone()).build()
                })
                .build(),
        )
        .build();
    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers.clone())
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
    let block_number = block_number + 100;
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push(block_number).unwrap();
    mock.push(block_number).unwrap();
    let loader = loader.unwrap();
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .validators(validators.clone())
        .validator_config(
            ValidatorConfig::builder()
                .rule({
                    let mut checkers = HashMap::new();
                    checkers.insert(0, RuleValidatorCheckerType::Sequence as i32);
                    RuleValidatorConfig::builder().checkers(checkers).build()
                })
                .build(),
        )
        .build();
    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers.clone())
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
    let block_number = block_number;
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push(block_number).unwrap();
    mock.push(block_number).unwrap();
    let loader = loader.unwrap();
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_chain_data_loader_mock_fetcher_validator() {
    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let mut validators = HashMap::new();
    validators.insert(0, ValidatorType::Rule as i32);
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .validators(validators.clone())
        .build();
    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;
    let handler = Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>);

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);

    let fetchers = vec![Arc::new(
        Box::new(MockFetcher::<FullData>::new(chain_id)) as Box<dyn DataFetcher<FullData>>
    )];
    let validators = vec![Arc::new(
        Box::new(MockValidator::new()) as Box<dyn DataValidator<FullData>>
    )];

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers.clone())
        .handler(handler.clone())
        .fetchers(fetchers)
        .validators(validators)
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
}
