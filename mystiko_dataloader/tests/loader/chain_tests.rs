use crate::loader::loader_mock::{
    create_loader, create_shared_loader, loader_start, MockContractFilter, MockFetcher, MockHandler, MockListener,
    MockValidator,
};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::filter::ContractFilter;
use mystiko_dataloader::loader::chain::ChainDataLoaderBuilder;
use mystiko_dataloader::loader::types::StartOption;
use std::sync::Arc;

#[tokio::test]
async fn test_loader_start() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 987_u64;
    let mut filter = MockContractFilter::new();
    filter.add_unfiltered_contract(contract_address);
    let filter: Arc<Box<dyn ContractFilter>> = Arc::new(Box::new(filter)); // Cast here
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .contract_filter(filter)
        .build();
    let loader = Arc::new(create_loader(false, true, contract_address, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 987);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);
    assert_eq!(
        state.contract_states.get(contract_address).unwrap().loaded_block,
        end_block
    );
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 765_u64;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    let loader = Arc::new(create_loader(true, false, contract_address, end_block).await);
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!loader.is_loading().await);
    assert!(!state.is_running);
    assert!(!loader.is_running().await);
    assert_eq!(
        state.contract_states.get(contract_address).unwrap().loaded_block,
        16690439
    );
}

#[tokio::test]
async fn test_loader_start_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(1, 1, 1).await;
    let mut filter = MockContractFilter::new();
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    filter.add_unfiltered_contracts(&[contract_address1, contract_address2]);
    let filter: Arc<Box<dyn ContractFilter>> = Arc::new(Box::new(filter)); // Cast here
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .contract_filter(filter)
        .build();
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert!(state.contract_states.is_empty());

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    let chain_id = 1_u64;
    let end_block1 = 10_u64;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address1)
            .start_block(1_u64)
            .end_block(end_block1)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        state.contract_states.get(contract_address1).unwrap().loaded_block,
        end_block1
    );
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block1
            ),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    let end_block2 = end_block1 + 10;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address2)
            .start_block(1_u64)
            .end_block(end_block2)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        state.contract_states.get(contract_address1).unwrap().loaded_block,
        end_block1
    );
    assert_eq!(
        state.contract_states.get(contract_address2).unwrap().loaded_block,
        end_block2
    );
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block1
            ),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    fetchers[0].set_error_result().await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block1
            ),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    let end_block3 = end_block2 - 2;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![
            ContractData::builder()
                .address(contract_address1)
                .start_block(1_u64)
                .end_block(end_block3)
                .build(),
            ContractData::builder()
                .address(contract_address2)
                .start_block(1_u64)
                .end_block(end_block3)
                .build(),
        ])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        state.contract_states.get(contract_address1).unwrap().loaded_block,
        end_block3
    );
    assert_eq!(
        state.contract_states.get(contract_address2).unwrap().loaded_block,
        end_block3
    );

    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!("LoadSuccessEvent-{:?}", end_block3),
            format!("StopEvent-{:?}", end_block3),
        ]
    );

    let fetcher_result = ChainData::builder().chain_id(chain_id).contracts_data(vec![]).build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block3);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block3 + 1),
            format!("LoadEvent-{:?}", end_block3 + 1),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block3
            ),
            format!("StopEvent-{:?}", end_block3),
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher() {
    let (loader, fetchers, _, _, listeners) = create_shared_loader(2, 1, 1).await;

    let mut filter = MockContractFilter::new();
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    filter.add_unfiltered_contracts(&[contract_address1, contract_address2]);
    let filter: Arc<Box<dyn ContractFilter>> = Arc::new(Box::new(filter)); // Cast here
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .contract_filter(filter)
        .build();
    let chain_id = 1_u64;
    let end_block1 = 10_u64;
    let end_block2 = 20_u64;

    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address1)
            .start_block(1_u64)
            .end_block(end_block1)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address2)
            .start_block(1_u64)
            .end_block(end_block2)
            .build()])
        .build();
    fetchers[1].set_result(fetcher_result).await;

    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        state.contract_states.get(contract_address1).unwrap().loaded_block,
        end_block1
    );
    assert_eq!(
        state.contract_states.get(contract_address2).unwrap().loaded_block,
        end_block2
    );
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-124"),
            format!("LoadEvent-124"),
            format!("LoadSuccessEvent-{:?}", end_block1),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![
            ContractData::builder()
                .address(contract_address1)
                .start_block(1_u64)
                .end_block(end_block1)
                .build(),
            ContractData::builder()
                .address(contract_address2)
                .start_block(1_u64)
                .end_block(end_block2)
                .build(),
        ])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!("LoadSuccessEvent-{:?}", end_block1),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    fetchers[0].set_error_result().await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block1);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block1
            ),
            format!("StopEvent-{:?}", end_block1),
        ]
    );

    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address1)
            .start_block(1_u64)
            .end_block(end_block2)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block2);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(state.contract_states.keys().len(), 2);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block1 + 1),
            format!("LoadEvent-{:?}", end_block1 + 1),
            format!("LoadSuccessEvent-{:?}", end_block2),
            format!("StopEvent-{:?}", end_block2),
        ]
    );
}

#[tokio::test]
async fn test_loader_start_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 1, 1).await;
    let chain_id = 1_u64;
    let end_block = 10_u64;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    let _ = listeners[0].drain_events().await;

    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(
            state
                .contract_states
                .keys()
                .map(|key| {
                    ContractData::builder()
                        .address(key)
                        .start_block(1_u64)
                        .end_block(end_block)
                        .build()
                })
                .collect::<Vec<_>>(),
        )
        .build();
    fetchers[0].set_result(fetcher_result).await;
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    validators[0].set_all_success().await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            format!("LoadSuccessEvent-{:?}", end_block),
            format!("StopEvent-{:?}", end_block),
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_validator() {
    let (loader, fetchers, validators, _, listeners) = create_shared_loader(1, 2, 1).await;
    let chain_id = 1_u64;
    let contract_address = "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01";
    let end_block = 10_u64;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address)
            .start_block(1_u64)
            .end_block(end_block)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;

    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(1000000_u64)
        .build();
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block
            ),
            format!("StopEvent-{:?}", end_block),
        ]
    );

    validators[1]
        .set_result(Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", end_block + 1),
            format!("LoadEvent-{:?}", end_block + 1),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block
            ),
            format!("StopEvent-{:?}", end_block),
        ]
    );
}

#[tokio::test]
async fn test_loader_start_shared_handler() {
    let (loader, fetchers, _, handler, listeners) = create_shared_loader(1, 1, 1).await;
    let chain_id = 1_u64;
    let contract_address = "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01";
    let end_block = 10_u64;
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(vec![ContractData::builder()
            .address(contract_address)
            .start_block(1_u64)
            .end_block(end_block)
            .build()])
        .build();
    fetchers[0].set_result(fetcher_result).await;
    handler.set_result(Err(anyhow::Error::msg("error".to_string()))).await;
    let option = StartOption {
        load_interval_ms: Some(10_u64),
        max_batch_block: Some(1000000_u64),
        contract_filter: None,
    };
    loader_start(loader.clone(), option.clone()).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent-124".to_string(),
            "LoadEvent-124".to_string(),
            "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
            "StopEvent-123".to_string()
        ]
    );

    handler.set_all_success().await;
    loader_start(loader.clone(), option).await;
    let state = loader.state().await;
    assert_eq!(state.loaded_block, end_block);
    assert!(!state.is_loading);
    assert!(!state.is_running);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-124"),
            format!("LoadEvent-124"),
            format!(
                "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
                end_block
            ),
            format!("StopEvent-{:?}", end_block),
        ]
    );
}

#[tokio::test]
async fn test_add_shared_handler() {
    let chain_id = 1_u64;
    let builder: ChainDataLoaderBuilder<FullData, MockFetcher<FullData>, MockValidator, MockHandler, MockListener> =
        ChainDataLoaderBuilder::new();
    let core_cfg = MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
        .await
        .unwrap();
    let loader = builder
        .chain_id(chain_id)
        .initial_block(123_u64)
        .config(Arc::new(core_cfg))
        .add_shared_fetcher(Arc::new(MockFetcher::new(chain_id)))
        .add_shared_validator(Arc::new(MockValidator::new()))
        .shared_handler(Arc::new(MockHandler::new()))
        .add_shared_listener(Arc::new(MockListener::default()))
        .build()
        .unwrap();
    let state = loader.state().await;
    assert_eq!(state.loaded_block, 123);
    assert!(!state.is_loading);
    assert!(!state.is_running);
}
