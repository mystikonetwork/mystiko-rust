use crate::loader::loader_mock::{
    chain_data_partial_eq, create_loader, create_shared_loader, loader_start, MockFetcher, MockHandler, MockListener,
    MockValidator,
};
use ethers_core::types::U64;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::error::DataLoaderError;
use mystiko_dataloader::loader::chain::ChainDataLoaderBuilder;
use mystiko_dataloader::loader::types::StartOption;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::time::sleep;

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
    loader_start(loader.clone(), 100).await;
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
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let result = loader.start(&StartOption::default()).await;
    assert_eq!(
        result.err().unwrap().to_string(),
        DataLoaderError::LoaderRunError("contracts cannot be empty".to_string()).to_string()
    );
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].drain_events().await.is_empty());

    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    handler.set_chain_loaded_blocks(vec![0]).await;
    let result = loader.start(&StartOption::default()).await;
    assert_eq!(
        result.err().unwrap().to_string(),
        "chain loaded block error".to_string()
    );
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].drain_events().await.is_empty());
}

#[tokio::test]
async fn test_loader_already_running() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let option = StartOption::builder()
        .load_interval_ms(10_u64)
        .max_batch_block(100_u64)
        .delay_block(2_u64)
        .build();

    let loader_clone1 = loader.clone();
    let loader_clone2 = loader.clone();
    let option_clone = option.clone();

    let handle1 = tokio::spawn(async move { loader_clone1.start(&option).await });
    let _ = sleep(std::time::Duration::from_millis(10_u64)).await;
    assert!(loader.is_running().await);
    let result = loader.start(&option_clone).await;
    assert!(result.is_ok());
    assert!(loader.is_running().await);

    let handle2 = tokio::spawn(async move { loader_clone2.stop().await });
    let result = futures::try_join!(handle1);
    assert!(result.is_ok());
    let result = futures::try_join!(handle2);
    assert!(result.is_ok());
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
}

// #[tokio::test]
// async fn test_loader_start_shared_fetcher() {
//     let chain_id = 1_u64;
//     let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
//     let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
//     let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
//     assert!(!loader.is_loading().await);
//     assert!(!loader.is_running().await);
//
//     let start_block = cfg
//         .find_chain(chain_id)
//         .unwrap()
//         .contracts_with_disabled()
//         .iter()
//         .map(|c| c.start_block())
//         .min()
//         .unwrap();
//     let max_batch_block = 10000;
//     let end_block = start_block + max_batch_block - 1;
//
//     let target_block_number = U64::from(end_block);
//     mock_provider.push(target_block_number).unwrap();
//
//     loader_start(loader.clone(), max_batch_block).await;
//     assert!(!loader.is_loading().await);
//     assert!(!loader.is_running().await);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", start_block + 1),
//             format!("LoadEvent-{:?}", start_block + 1),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 start_block
//             ),
//             format!("StopEvent-{:?}", start_block),
//         ]
//     );
//     assert!(handler.drain_data().await.is_empty());
//
//     let end_block1 = end_block;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address1)
//             .start_block(start_block)
//             .end_block(end_block1)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result.clone()).await;
//     mock_provider.push(target_block_number).unwrap();
//     loader_start(loader.clone(), max_batch_block).await;
//     assert!(!loader.is_loading().await);
//     assert!(!loader.is_running().await);
//     assert!(chain_data_partial_eq(
//         &handler.drain_data().await,
//         &vec![fetcher_result.clone()]
//     ));
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", start_block + 1),
//             format!("LoadEvent-{:?}", start_block + 1),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 start_block
//             ),
//             format!("StopEvent-{:?}", start_block),
//         ]
//     );
//
//     let mut contracts = HashSet::new();
//     contracts.insert(contract_address1);
//     handler.set_contracts(chain_id, contracts, cfg).await;
//     handler.set_contract_loaded_block(contract_address1, end_block1).await;
//     mock_provider.push(target_block_number).unwrap();
//     loader_start(loader.clone(), max_batch_block).await;
//     assert!(!loader.is_loading().await);
//     assert!(!loader.is_running().await);
//     assert!(chain_data_partial_eq(
//         &handler.drain_data().await,
//         &vec![fetcher_result.clone()]
//     ));
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", start_block + 1),
//             format!("LoadEvent-{:?}", start_block + 1),
//             format!("LoadSuccessEvent-{:?}", end_block1),
//             format!("StopEvent-{:?}", end_block1),
//         ]
//     );
//
//     // let end_block2 = end_block1 + 10;
//     // let fetcher_result = ChainData::builder()
//     //     .chain_id(chain_id)
//     //     .contracts_data(vec![ContractData::builder()
//     //         .address(contract_address2)
//     //         .start_block(1_u64)
//     //         .end_block(end_block2)
//     //         .build()])
//     //     .build();
//     // fetchers[0].set_result(fetcher_result).await;
//     // loader_start(loader.clone(), max_batch_block).await;
//     // // assert_eq!(state.loaded_block, end_block1);
//     // // assert_eq!(state.contract_states.keys().len(), 2);
//     // // assert_eq!(
//     // //     state.contract_states.get(contract_address1).unwrap().loaded_block,
//     // //     end_block1
//     // // );
//     // // assert_eq!(
//     // //     state.contract_states.get(contract_address2).unwrap().loaded_block,
//     // //     end_block2
//     // // );
//     // assert_eq!(
//     //     listeners[0].drain_events().await,
//     //     vec![
//     //         format!("StartEvent-{:?}", end_block1 + 1),
//     //         format!("LoadEvent-{:?}", end_block1 + 1),
//     //         format!(
//     //             "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//     //             end_block1
//     //         ),
//     //         format!("StopEvent-{:?}", end_block1),
//     //     ]
//     // );
//     //
//     // fetchers[0].set_error_result().await;
//     // loader_start(loader.clone(), max_batch_block).await;
//     // // assert_eq!(state.loaded_block, end_block1);
//     // // assert_eq!(state.contract_states.keys().len(), 2);
//     // assert_eq!(
//     //     listeners[0].drain_events().await,
//     //     vec![
//     //         format!("StartEvent-{:?}", end_block1 + 1),
//     //         format!("LoadEvent-{:?}", end_block1 + 1),
//     //         format!(
//     //             "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//     //             end_block1
//     //         ),
//     //         format!("StopEvent-{:?}", end_block1),
//     //     ]
//     // );
//     //
//     // let end_block3 = end_block2 - 2;
//     // let fetcher_result = ChainData::builder()
//     //     .chain_id(chain_id)
//     //     .contracts_data(vec![
//     //         ContractData::builder()
//     //             .address(contract_address1)
//     //             .start_block(1_u64)
//     //             .end_block(end_block3)
//     //             .build(),
//     //         ContractData::builder()
//     //             .address(contract_address2)
//     //             .start_block(1_u64)
//     //             .end_block(end_block3)
//     //             .build(),
//     //     ])
//     //     .build();
//     // fetchers[0].set_result(fetcher_result).await;
//     // loader_start(loader.clone(), max_batch_block).await;
//     // // assert_eq!(state.loaded_block, end_block3);
//     // // assert_eq!(state.contract_states.keys().len(), 2);
//     // // assert_eq!(
//     // //     state.contract_states.get(contract_address1).unwrap().loaded_block,
//     // //     end_block3
//     // // );
//     // // assert_eq!(
//     // //     state.contract_states.get(contract_address2).unwrap().loaded_block,
//     // //     end_block3
//     // // );
//     //
//     // assert_eq!(
//     //     listeners[0].drain_events().await,
//     //     vec![
//     //         format!("StartEvent-{:?}", end_block1 + 1),
//     //         format!("LoadEvent-{:?}", end_block1 + 1),
//     //         format!("LoadSuccessEvent-{:?}", end_block3),
//     //         format!("StopEvent-{:?}", end_block3),
//     //     ]
//     // );
//     //
//     // let fetcher_result = ChainData::builder().chain_id(chain_id).contracts_data(vec![]).build();
//     // fetchers[0].set_result(fetcher_result).await;
//     // loader_start(loader.clone(), max_batch_block).await;
//     // // assert_eq!(state.loaded_block, end_block3);
//     // // assert_eq!(state.contract_states.keys().len(), 2);
//     // assert_eq!(
//     //     listeners[0].drain_events().await,
//     //     vec![
//     //         format!("StartEvent-{:?}", end_block3 + 1),
//     //         format!("LoadEvent-{:?}", end_block3 + 1),
//     //         format!(
//     //             "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//     //             end_block3
//     //         ),
//     //         format!("StopEvent-{:?}", end_block3),
//     //     ]
//     // );
// }
//
// #[tokio::test]
// async fn test_loader_start_two_shared_fetcher() {
//     let chain_id = 1_u64;
//     let max_batch_block = 100;
//     let (cfg, loader, fetchers, _, _, listeners, mock_provider) = create_shared_loader(chain_id, 2, 1, 1).await;
//
//     let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
//     let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
//     let chain_id = 1_u64;
//     let end_block1 = 10_u64;
//     let end_block2 = 20_u64;
//
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address1)
//             .start_block(1_u64)
//             .end_block(end_block1)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address2)
//             .start_block(1_u64)
//             .end_block(end_block2)
//             .build()])
//         .build();
//     fetchers[1].set_result(fetcher_result).await;
//
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block1);
//     // assert_eq!(state.contract_states.keys().len(), 2);
//     // assert_eq!(
//     //     state.contract_states.get(contract_address1).unwrap().loaded_block,
//     //     end_block1
//     // );
//     // assert_eq!(
//     //     state.contract_states.get(contract_address2).unwrap().loaded_block,
//     //     end_block2
//     // );
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-124"),
//             format!("LoadEvent-124"),
//             format!("LoadSuccessEvent-{:?}", end_block1),
//             format!("StopEvent-{:?}", end_block1),
//         ]
//     );
//
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![
//             ContractData::builder()
//                 .address(contract_address1)
//                 .start_block(1_u64)
//                 .end_block(end_block1)
//                 .build(),
//             ContractData::builder()
//                 .address(contract_address2)
//                 .start_block(1_u64)
//                 .end_block(end_block2)
//                 .build(),
//         ])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block1);
//     // assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block1 + 1),
//             format!("LoadEvent-{:?}", end_block1 + 1),
//             format!("LoadSuccessEvent-{:?}", end_block1),
//             format!("StopEvent-{:?}", end_block1),
//         ]
//     );
//
//     fetchers[0].set_error_result().await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block1);
//     // assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block1 + 1),
//             format!("LoadEvent-{:?}", end_block1 + 1),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block1
//             ),
//             format!("StopEvent-{:?}", end_block1),
//         ]
//     );
//
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address1)
//             .start_block(1_u64)
//             .end_block(end_block2)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block2);
//     // assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block1 + 1),
//             format!("LoadEvent-{:?}", end_block1 + 1),
//             format!("LoadSuccessEvent-{:?}", end_block2),
//             format!("StopEvent-{:?}", end_block2),
//         ]
//     );
// }
//
// #[tokio::test]
// async fn test_loader_start_shared_validator() {
//     let chain_id = 1_u64;
//     let (cfg, loader, fetchers, validators, _, listeners, mock_provider) =
//         create_shared_loader(chain_id, 1, 1, 1).await;
//     let chain_id = 1_u64;
//     let end_block = 10_u64;
//     loader_start(loader.clone(),max_batch_block).await;
//     let _ = listeners[0].drain_events().await;
//
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(
//             state
//                 .contract_states
//                 .keys()
//                 .map(|key| {
//                     ContractData::builder()
//                         .address(key)
//                         .start_block(1_u64)
//                         .end_block(end_block)
//                         .build()
//                 })
//                 .collect::<Vec<_>>(),
//         )
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     validators[0]
//         .set_result(Err(anyhow::Error::msg("error".to_string())))
//         .await;
//     loader_start(loader.clone(),max_batch_block).await;
//     // assert_eq!(state.loaded_block, 123);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
//             "StopEvent-123".to_string()
//         ]
//     );
//
//     validators[0]
//         .set_result(Err(anyhow::Error::msg("error".to_string())))
//         .await;
//     loader_start(loader.clone(),max_batch_block).await;
//     // assert_eq!(state.loaded_block, 123);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
//             "StopEvent-123".to_string()
//         ]
//     );
//
//     validators[0].set_all_success().await;
//     loader_start(loader.clone(),max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             format!("LoadSuccessEvent-{:?}", end_block),
//             format!("StopEvent-{:?}", end_block),
//         ]
//     );
// }
//
// #[tokio::test]
// async fn test_loader_start_two_shared_validator() {
//     let chain_id = 1_u64;
//     let max_batch_block = 100;
//
//     let (cfg, loader, fetchers, validators, _, listeners, mock_provider) =
//         create_shared_loader(chain_id, 1, 2, 1).await;
//     let chain_id = 1_u64;
//     let contract_address = "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01";
//     let end_block = 10_u64;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address)
//             .start_block(1_u64)
//             .end_block(end_block)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block
//             ),
//             format!("StopEvent-{:?}", end_block),
//         ]
//     );
//
//     validators[1]
//         .set_result(Err(anyhow::Error::msg("error".to_string())))
//         .await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block + 1),
//             format!("LoadEvent-{:?}", end_block + 1),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block
//             ),
//             format!("StopEvent-{:?}", end_block),
//         ]
//     );
// }
//
// #[tokio::test]
// async fn test_loader_start_shared_handler() {
//     let chain_id = 1_u64;
//     let max_batch_block = 100;
//
//     let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
//     let chain_id = 1_u64;
//     let contract_address = "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01";
//     let end_block = 10_u64;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address)
//             .start_block(1_u64)
//             .end_block(end_block)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     handler.set_result(Err(anyhow::Error::msg("error".to_string()))).await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, 123);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             "LoadFailureEvent-123-loader run error failed fetch from all fetchers".to_string(),
//             "StopEvent-123".to_string()
//         ]
//     );
//
//     handler.set_all_success().await;
//     loader_start(loader.clone(), max_batch_block).await;
//     // assert_eq!(state.loaded_block, end_block);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-124"),
//             format!("LoadEvent-124"),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block
//             ),
//             format!("StopEvent-{:?}", end_block),
//         ]
//     );
// }
