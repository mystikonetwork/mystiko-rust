use crate::loader::loader_mock::{contract_data_partial_eq, create_shared_loader, loader_start};
use ethers_core::types::U64;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_shared_fetcher_error() {
    let chain_id = 1_u64;
    let (cfg, loader, _fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let delay_block = 2;

    // fetch return error
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                start_block - 1
            ),
            "StopEvent".to_string(),
        ]
    );
    assert!(handler.drain_data().await.is_empty());
}

#[tokio::test]
async fn test_loader_start_one_shared_fetcher_one_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    for delay_block in 0..5 {
        for target_block in start_block + delay_block..start_block + delay_block + 5 {
            // fetch one contract success, contract loaded block < target_block - delay_block
            let contract_data = vec![ContractData::builder()
                .address(contract_address1)
                .start_block(start_block)
                .end_block(target_block - delay_block - 1)
                .build()];
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[0].set_result(fetcher_result.clone()).await;
            mock_provider.push(U64::from(target_block)).unwrap();
            loader_start(loader.clone(), Some(delay_block)).await;
            assert!(!loader.is_loading().await);
            assert!(!loader.is_running().await);
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert_eq!(
                listeners[0].drain_events().await,
                vec![
                    "StartEvent".to_string(),
                    format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    format!(
                        "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                        start_block,
                        target_block - delay_block - 1
                    ),
                    "StopEvent".to_string(),
                ]
            );

            // fetch one contract success, contract loaded block == target_block - delay_block
            let contract_data = vec![ContractData::builder()
                .address(contract_address1)
                .start_block(start_block)
                .end_block(target_block - delay_block)
                .build()];
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[0].set_result(fetcher_result.clone()).await;
            mock_provider.push(U64::from(target_block)).unwrap();
            loader_start(loader.clone(), Some(delay_block)).await;
            assert!(!loader.is_loading().await);
            assert!(!loader.is_running().await);
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert_eq!(
                listeners[0].drain_events().await,
                vec![
                    "StartEvent".to_string(),
                    format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    "StopEvent".to_string(),
                ]
            );

            // fetch one contract success, contract loaded block > target_block - delay_block
            let contract_data = vec![ContractData::builder()
                .address(contract_address1)
                .start_block(start_block)
                .end_block(target_block)
                .build()];
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[0].set_result(fetcher_result.clone()).await;
            mock_provider.push(U64::from(target_block)).unwrap();
            loader_start(loader.clone(), Some(delay_block)).await;
            assert!(!loader.is_loading().await);
            assert!(!loader.is_running().await);
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert_eq!(
                listeners[0].drain_events().await,
                vec![
                    "StartEvent".to_string(),
                    format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block),
                    "StopEvent".to_string(),
                ]
            );
        }
    }
}

#[tokio::test]
async fn test_loader_start_one_shared_fetcher_two_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);

    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    for delay_block in 0..5 {
        for target_block in start_block + delay_block..start_block + delay_block + 5 {
            // fetch two contract success, all contract loaded block = target_block - delay_block
            let contract_data = vec![
                ContractData::builder()
                    .address(contract_address1)
                    .start_block(start_block)
                    .end_block(target_block - delay_block)
                    .build(),
                ContractData::builder()
                    .address(contract_address2)
                    .start_block(start_block)
                    .end_block(target_block - delay_block)
                    .build(),
            ];
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[0].set_result(fetcher_result.clone()).await;
            mock_provider.push(U64::from(target_block)).unwrap();
            loader_start(loader.clone(), Some(delay_block)).await;
            assert!(!loader.is_loading().await);
            assert!(!loader.is_running().await);
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert_eq!(
                listeners[0].drain_events().await,
                vec![
                    "StartEvent".to_string(),
                    format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    "StopEvent".to_string(),
                ]
            );

            // fetch two contract success, one contract loaded block < target_block - delay_block
            let contract_data = vec![
                ContractData::builder()
                    .address(contract_address1)
                    .start_block(start_block)
                    .end_block(target_block - delay_block)
                    .build(),
                ContractData::builder()
                    .address(contract_address2)
                    .start_block(start_block)
                    .end_block(target_block - delay_block - 1)
                    .build(),
            ];
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[0].set_result(fetcher_result.clone()).await;
            mock_provider.push(U64::from(target_block)).unwrap();
            loader_start(loader.clone(), Some(delay_block)).await;
            assert!(!loader.is_loading().await);
            assert!(!loader.is_running().await);
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert_eq!(
                listeners[0].drain_events().await,
                vec![
                    "StartEvent".to_string(),
                    format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
                    format!(
                        "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                        start_block,
                        target_block - delay_block - 1
                    ),
                    "StopEvent".to_string(),
                ]
            );
        }
    }
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 2, 1, 1).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let delay_block = 2;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_result(fetcher1_result.clone()).await;
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_result(fetcher2_result.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                target_block - delay_block
            ),
            "StopEvent".to_string(),
        ]
    );

    // fetch all contract success, contract loaded block == target_block - delay_block
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block - delay_block),
            "StopEvent".to_string(),
        ]
    );

    // fetch all contract success, contract1 loaded block < target_block - delay_block
    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - delay_block - 1)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_result(fetcher1_result.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                target_block - delay_block - 1
            ),
            "StopEvent".to_string(),
        ]
    );

    // fetch all contract success,  fetcher 2 return two contract data loaded block == target_block - delay_block
    let contract_data2 = vec![
        ContractData::builder()
            .address(contract_address1)
            .start_block(start_block)
            .end_block(target_block - delay_block)
            .build(),
        ContractData::builder()
            .address(contract_address2)
            .start_block(start_block)
            .end_block(target_block - delay_block)
            .build(),
    ];
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_result(fetcher2_result.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data2));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block - delay_block),
            "StopEvent".to_string(),
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher_with_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 2, 1, 1).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let delay_block = 2;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    // all fetch return error
    fetchers[0].set_error_result().await;
    fetchers[1].set_error_result().await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                start_block - 1
            ),
            "StopEvent".to_string(),
        ]
    );

    // fetcher 1 return error
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_result(fetcher2_result.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data2.clone()
    ));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                target_block - delay_block
            ),
            "StopEvent".to_string(),
        ]
    );

    // fetcher 2 return error
    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_result(fetcher1_result.clone()).await;
    fetchers[1].set_error_result().await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data1.clone()
    ));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                target_block - delay_block
            ),
            "StopEvent".to_string(),
        ]
    );
}
