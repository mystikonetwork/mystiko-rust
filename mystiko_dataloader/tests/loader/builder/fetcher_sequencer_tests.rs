use crate::loader::{create_mock_providers, MockHandler};
use ethers_providers::Provider as EthersProvider;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::handler::DataHandler;
use mystiko_dataloader::loader::{ChainDataLoader, FromConfig, LoaderConfigOptions};
use mystiko_ethers::Providers;
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::loader::v1::{FetcherConfig, FetcherType, LoaderConfig, SequencerFetcherConfig};
use mystiko_protos::service::v1::ClientOptions;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[tokio::test]
async fn test_create_chain_data_loader_fetcher_sequencer() {
    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko_wrong_provider.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());
    let mut fetchers = HashMap::new();
    fetchers.insert(0, FetcherType::Sequencer as i32);
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetchers(fetchers.clone())
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

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetchers(fetchers.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .sequencer(
                    SequencerFetcherConfig::builder()
                        .skip_validation(true)
                        .target_block_priority(22)
                        .options(
                            ClientOptions::builder()
                                .host("localhost".to_string())
                                .port(50051)
                                .is_ssl(true)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .providers(providers)
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
}

#[tokio::test]
async fn test_create_chain_data_loader_fetcher_sequencer_skip_validation() {
    let cfg_option = ConfigOptions::builder()
        .file_path("./tests/files/config/mystiko_wrong_provider.json".to_string())
        .build();
    let chain_id = 1;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mystiko_cfg = Arc::new(MystikoConfig::from_options(cfg_option.clone()).await.unwrap());

    let handler = MockHandler::new();
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, mystiko_cfg.clone()).await;
    let handler = Arc::new(Box::new(handler) as Box<dyn DataHandler<FullData>>);

    let (_, mock) = EthersProvider::mocked();
    let providers = create_mock_providers(Some(&mock));
    let providers = Arc::new(Box::new(providers) as Box<dyn Providers>);
    let mut fetchers = HashMap::new();
    fetchers.insert(0, FetcherType::Sequencer as i32);
    let cfg = LoaderConfig::builder()
        .mystiko_config_options(cfg_option.clone())
        .fetchers(fetchers.clone())
        .fetcher_config(
            FetcherConfig::builder()
                .sequencer(
                    SequencerFetcherConfig::builder()
                        .skip_validation(true)
                        .options(
                            ClientOptions::builder()
                                .host("localhost".to_string())
                                .port(50051)
                                .is_ssl(true)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();
    let options = LoaderConfigOptions::<FullData>::builder()
        .chain_id(chain_id)
        .config(cfg)
        .handler(handler.clone())
        .providers(providers)
        .build();
    let loader = ChainDataLoader::<FullData>::from_config(&options).await;
    assert!(loader.is_ok());
}
