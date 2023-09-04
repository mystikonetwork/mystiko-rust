use crate::loader::loader_mock::{
    create_loader, loader_load, ChainDataLoaderBuilderFullDataType, MockFetcher, MockHandler, MockValidator,
};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::loader::ChainDataLoaderBuilder;
use mystiko_dataloader::DataLoaderError;
use std::sync::Arc;

#[tokio::test]
async fn test_builder_error() {
    let chain_id = 1_u64;
    let builder: ChainDataLoaderBuilderFullDataType = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let core_cfg = Arc::new(core_cfg);

    let builder: ChainDataLoaderBuilderFullDataType = ChainDataLoaderBuilder::new();

    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

    let builder: ChainDataLoaderBuilderFullDataType = ChainDataLoaderBuilder::new();

    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

    let builder: ChainDataLoaderBuilderFullDataType = ChainDataLoaderBuilder::new();

    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_fetcher(Arc::new(MockFetcher::new(chain_id)))
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

    let builder: ChainDataLoaderBuilderFullDataType = ChainDataLoaderBuilder::new();

    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));
}

#[tokio::test]
async fn test_loader_start() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 987_u64;

    let loader = Arc::new(create_loader(false, true, contract_address, end_block).await);
    let result = loader_load(loader, None).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 765_u64;
    let delay_block = 2_u64;
    let loader = Arc::new(create_loader(true, false, contract_address, end_block).await);
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}
