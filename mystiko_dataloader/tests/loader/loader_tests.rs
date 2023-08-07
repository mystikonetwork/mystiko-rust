use crate::loader::loader_mock::{
    create_loader, create_shared_loader, loader_run, LoaderRunType, MockFetcher, MockHandler, MockListener,
    MockValidator,
};
use ethers_core::types::U64;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::error::DataLoaderError;
use mystiko_dataloader::loader::chain::ChainDataLoaderBuilder;
use mystiko_dataloader::loader::types::{LoadOption, ScheduleOption};
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
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

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
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

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
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

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
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));

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
    assert!(matches!(loader.err().unwrap(), DataLoaderError::LoaderBuildError(_)));
}

#[tokio::test]
async fn test_loader_start() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 987_u64;

    let loader = Arc::new(create_loader(false, true, contract_address, end_block).await);
    loader_run(LoaderRunType::Load, loader, None).await;
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 765_u64;
    let delay_block = 2_u64;
    let loader = Arc::new(create_loader(true, false, contract_address, end_block).await);
    loader_run(LoaderRunType::Load, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
}

#[tokio::test]
async fn test_loader_start_meet_error() {
    let chain_id = 12345678901234567890_u64;
    let delay_block = 2_u64;

    let (_cfg, loader, _fetchers, _, _handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    let result = loader
        .schedule(ScheduleOption::builder().schedule_interval_ms(5_u64).build())
        .await;
    assert_eq!(
        result.err().unwrap().to_string(),
        DataLoaderError::UnsupportedChainError(chain_id).to_string()
    );
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].is_event_empty().await);

    // test build_chain_start_block meet error
    let chain_id = 1_u64;
    let (_cfg, loader, _fetchers, _, _handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    loader_run(LoaderRunType::Schedule, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["ScheduleEvent".to_string(), "StopScheduleEvent".to_string()]
    );

    // test build_chain_target_block meet error
    let (_cfg, loader, _fetchers, _, _handler, listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    loader_run(LoaderRunType::Schedule, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["ScheduleEvent".to_string(), "StopScheduleEvent".to_string()]
    );

    // test build_chain_target_block target_block is too small
    let (_cfg, loader, _fetchers, _, _handler, listeners, mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    mock_provider.push(U64::from(1_u64)).unwrap();
    loader_run(LoaderRunType::Schedule, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["ScheduleEvent".to_string(), "StopScheduleEvent".to_string()]
    );
}

#[tokio::test]
async fn test_loader_already_running() {
    let chain_id = 1_u64;
    let (_cfg, loader, _fetchers, _, _handler, _listeners, _mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    let option = ScheduleOption::builder()
        .schedule_interval_ms(5_u64)
        .load_option(LoadOption::builder().delay_block(10_u64).build())
        .build();

    let handle = loader.schedule(option.clone()).await;
    assert!(handle.as_ref().unwrap().is_some());
    let result = loader.schedule(option.clone()).await;
    assert!(result.unwrap().is_none());
    assert!(loader.is_running().await);
    loader.stop_schedule().await;
    let result = futures::try_join!(handle.unwrap().unwrap());
    assert!(result.is_ok());
    assert!(!loader.is_running().await);
}

#[tokio::test]
async fn test_restart_loader() {
    let chain_id = 1_u64;
    let delay_block = 2_u64;

    let (_cfg, loader, _fetchers, _, _, listeners, _mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;

    loader_run(LoaderRunType::Schedule, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["ScheduleEvent".to_string(), "StopScheduleEvent".to_string()]
    );

    loader_run(LoaderRunType::Schedule, loader.clone(), Some(delay_block)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec!["ScheduleEvent".to_string(), "StopScheduleEvent".to_string()]
    );
}
