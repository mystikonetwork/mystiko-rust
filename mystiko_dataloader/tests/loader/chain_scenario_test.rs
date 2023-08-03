use crate::loader::loader_mock::{create_shared_loader, loader_start};
use ethers_core::types::U64;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::loader::types::StartOption;
use std::sync::Arc;

#[tokio::test]
async fn test_restart_loader() {
    let chain_id = 1_u64;
    let max_batch_block = 100;

    let (cfg, loader, fetchers, _, _, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);

    let start_block = cfg
        .find_chain(chain_id)
        .unwrap()
        .contracts_with_disabled()
        .iter()
        .map(|c| c.start_block())
        .min()
        .unwrap();

    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            format!("StartEvent-{:?}", start_block + 1),
            format!("StopEvent-{:?}", start_block),
        ]
    );

    // let block_number = U64::from(10000);
    // mock_provider.push(block_number).unwrap();
    // loader_start(loader.clone(), option.clone()).await;
    // assert!(!loader.is_loading().await);
    // assert!(!loader.is_running().await);
    // assert_eq!(
    //     listeners[0].drain_events().await,
    //     vec![
    //         format!("StartEvent-{:?}", start_block + 1),
    //         format!("StopEvent-{:?}", start_block),
    //     ]
    // );
}
//
// #[tokio::test]
// async fn test_loader_start_4_fetcher_0_validator() {
//     let (loader, fetchers, _, _, listeners) = create_shared_loader(4, 0, 1).await;
//     let mut filter = MockContractFilter::new();
//     let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
//     let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
//     filter.add_unfiltered_contracts(&[contract_address1, contract_address2]);
//     let filter: Arc<Box<dyn ContractFilter>> = Arc::new(Box::new(filter)); // Cast here
//     let option = StartOption::builder()
//         .load_interval_ms(10_u64)
//         .max_batch_block(1000000_u64)
//         .contract_filter(filter)
//         .build();
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, 123);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert!(state.contract_states.is_empty());
//
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, 123);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
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
//     let chain_id = 1_u64;
//     let end_block1 = 10_u64;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address1)
//             .start_block(1_u64)
//             .end_block(end_block1)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, end_block1);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         state.contract_states.get(contract_address1).unwrap().loaded_block,
//         end_block1
//     );
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             "StartEvent-124".to_string(),
//             "LoadEvent-124".to_string(),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block1
//             ),
//             format!("StopEvent-{:?}", end_block1),
//         ]
//     );
//
//     let end_block2 = end_block1 + 10;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![ContractData::builder()
//             .address(contract_address2)
//             .start_block(1_u64)
//             .end_block(end_block2)
//             .build()])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, end_block1);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         state.contract_states.get(contract_address1).unwrap().loaded_block,
//         end_block1
//     );
//     assert_eq!(
//         state.contract_states.get(contract_address2).unwrap().loaded_block,
//         end_block2
//     );
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
//     fetchers[0].set_error_result().await;
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, end_block1);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
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
//     let end_block3 = end_block2 - 2;
//     let fetcher_result = ChainData::builder()
//         .chain_id(chain_id)
//         .contracts_data(vec![
//             ContractData::builder()
//                 .address(contract_address1)
//                 .start_block(1_u64)
//                 .end_block(end_block3)
//                 .build(),
//             ContractData::builder()
//                 .address(contract_address2)
//                 .start_block(1_u64)
//                 .end_block(end_block3)
//                 .build(),
//         ])
//         .build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, end_block3);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         state.contract_states.get(contract_address1).unwrap().loaded_block,
//         end_block3
//     );
//     assert_eq!(
//         state.contract_states.get(contract_address2).unwrap().loaded_block,
//         end_block3
//     );
//
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block1 + 1),
//             format!("LoadEvent-{:?}", end_block1 + 1),
//             format!("LoadSuccessEvent-{:?}", end_block3),
//             format!("StopEvent-{:?}", end_block3),
//         ]
//     );
//
//     let fetcher_result = ChainData::builder().chain_id(chain_id).contracts_data(vec![]).build();
//     fetchers[0].set_result(fetcher_result).await;
//     loader_start(loader.clone(), option.clone()).await;
//     let state = loader.state().await;
//     assert_eq!(state.loaded_block, end_block3);
//     assert!(!state.is_loading);
//     assert!(!state.is_running);
//     assert_eq!(state.contract_states.keys().len(), 2);
//     assert_eq!(
//         listeners[0].drain_events().await,
//         vec![
//             format!("StartEvent-{:?}", end_block3 + 1),
//             format!("LoadEvent-{:?}", end_block3 + 1),
//             format!(
//                 "LoadFailureEvent-{:?}-loader run error failed fetch from all fetchers",
//                 end_block3
//             ),
//             format!("StopEvent-{:?}", end_block3),
//         ]
//     );
// }
