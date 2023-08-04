use crate::loader::loader_mock::{
    create_loader, create_shared_loader, loader_start, MockFetcher, MockHandler, MockListener, MockValidator,
};
use ethers_core::types::U64;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::error::DataLoaderError;
use mystiko_dataloader::loader::chain::ChainDataLoaderBuilder;
use mystiko_dataloader::loader::types::StartOption;
use std::sync::Arc;

#[tokio::test]
async fn test_builder_error() {
    let chain_id = 1_u64;
    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderInitError(_)));

    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let core_cfg = Arc::new(core_cfg);

    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderInitError(_)));

    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderInitError(_)));

    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_fetcher(Arc::new(MockFetcher::new(chain_id)))
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderInitError(_)));

    let builder: ChainDataLoaderBuilder<
        FullData,
        MockFetcher<FullData>,
        MockValidator,
        MockHandler<FullData>,
        MockListener,
    > = ChainDataLoaderBuilder::new();
    let loader = builder
        .chain_id(chain_id)
        .config(core_cfg.clone())
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build();
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderInitError(_)));
}

#[tokio::test]
async fn test_loader_start() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 987_u64;

    let loader = Arc::new(create_loader(false, true, contract_address, end_block).await);
    loader_start(loader, 100).await;
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 765_u64;
    let loader = Arc::new(create_loader(true, false, contract_address, end_block).await);
    loader_start(loader.clone(), 100).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
}

#[tokio::test]
async fn test_loader_start_meet_error() {
    let chain_id = 12345678901234567890_u64;
    let (_cfg, loader, _fetchers, _, _handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    let result = loader.start(StartOption::default()).await;
    assert_eq!(
        result.err().unwrap().to_string(),
        DataLoaderError::UnsupportedChainError(chain_id).to_string()
    );
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].drain_events().await.is_empty());

    // test build_chain_start_block meet error
    let chain_id = 1_u64;
    let max_batch_block = 1_000_u64;
    let (_cfg, loader, _fetchers, _, handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    handler.set_chain_loaded_blocks(vec![0]).await;
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["StartEvent".to_string(), "StopEvent".to_string()]
    );

    // test build_chain_target_block meet error
    let (_cfg, loader, _fetchers, _, handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    handler.set_chain_loaded_blocks(vec![]).await;
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["StartEvent".to_string(), "StopEvent".to_string()]
    );

    // test build_chain_target_block target_block is too small
    let (_cfg, loader, _fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    handler.set_chain_loaded_blocks(vec![]).await;
    mock_provider.push(U64::from(1_u64)).unwrap();
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["StartEvent".to_string(), "StopEvent".to_string()]
    );
}

#[tokio::test]
async fn test_loader_already_running() {
    let chain_id = 1_u64;
    let (_cfg, loader, _fetchers, _, _handler, _listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(100_u64)
        .delay_block(2_u64)
        .build();

    let handle = loader.start(StartOption::default()).await;
    assert!(handle.as_ref().unwrap().is_some());
    let result = loader.start(option.clone()).await;
    assert!(result.unwrap().is_none());
    assert!(loader.is_running().await);
    loader.stop().await;
    let result = futures::try_join!(handle.unwrap().unwrap());
    assert!(result.is_ok());
    assert!(!loader.is_running().await);
}

#[tokio::test]
async fn test_restart_loader() {
    let chain_id = 1_u64;
    let max_batch_block = 100;

    let (_cfg, loader, _fetchers, _, _, listeners, _mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["StartEvent".to_string(), "StopEvent".to_string()]
    );

    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["StartEvent".to_string(), "StopEvent".to_string()]
    );
}
