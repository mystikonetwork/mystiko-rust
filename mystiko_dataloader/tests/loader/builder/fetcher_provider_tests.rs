use crate::loader::{create_mock_providers, MockHandler};
use ethers_core::types::{Log, U64};
use ethers_providers::Provider as EthersProvider;
use log::LevelFilter;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::handler::DataHandler;
use mystiko_dataloader::loader::LoadOption;
use mystiko_dataloader::loader::{ChainDataLoader, DataLoader, FromConfig, LoaderConfigOptions};
use mystiko_ethers::Providers;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::common::v1::ProviderType;
use mystiko_protos::loader::v1::{FetcherConfig, LoaderConfig, ProviderFetcherChainConfig, ProviderFetcherConfig};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[tokio::test]
async fn test_create_chain_data_loader_default_provider_fetcher() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();

    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let cfg = LoaderConfig::builder().mystiko_config_options(cfg_option).build();
    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers)
        .handler(Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>))
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());

    let block_number = U64::from(16690441);
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push::<Vec<Log>, _>(vec![]).unwrap();
    mock.push(block_number).unwrap();
    mock.push(block_number).unwrap();
    let loader = loader.unwrap();
    let result = loader.load(LoadOption::default()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_chain_data_loader_default_provider_fetcher_skip_validation() {
    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option)
        .fetcher_config(
            FetcherConfig::builder()
                .provider(
                    ProviderFetcherConfig::builder()
                        .concurrency(2)
                        .timeout_ms(10)
                        .skip_validation(true)
                        .build(),
                )
                .build(),
        )
        .build();
    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .providers(providers)
        .handler(Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>))
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
}

#[tokio::test]
async fn test_create_chain_data_loader_providers_merge() {
    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko_wrong_provider.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .provider(ProviderFetcherConfig::builder().timeout_ms(1234).build())
                .build(),
        )
        .build();
    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;
    let handler = Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>);

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());

    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .provider(ProviderFetcherConfig::builder().build())
                .build(),
        )
        .build();
    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());

    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .provider(
                    ProviderFetcherConfig::builder()
                        .chains({
                            let mut chains = HashMap::new();
                            let mut urls = HashMap::new();
                            urls.insert(0, "http://localhost:38545".to_string());
                            let cfg = ProviderFetcherChainConfig::builder().urls(urls.clone()).build();
                            chains.insert(chain_id, cfg);
                            chains
                        })
                        .build(),
                )
                .build(),
        )
        .build();

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());

    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .provider(
                    ProviderFetcherConfig::builder()
                        .concurrency(1)
                        .timeout_ms(1)
                        .chains({
                            let mut chains = HashMap::new();
                            let mut urls = HashMap::new();
                            urls.insert(0, "http://localhost:38545".to_string());
                            let cfg = ProviderFetcherChainConfig::builder()
                                .urls(urls)
                                .delay_num_blocks(100)
                                .provider_type(ProviderType::Failover as i32)
                                .build();
                            chains.insert(chain_id, cfg);
                            chains
                        })
                        .build(),
                )
                .build(),
        )
        .build();

    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
}
