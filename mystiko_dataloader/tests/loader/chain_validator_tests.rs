use crate::loader::loader_mock::{contract_data_partial_eq, create_shared_loader, loader_start};
use ethers_core::types::U64;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use mystiko_dataloader::data::result::{ChainResult, ContractResult};
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_without_validator() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler, listeners, mock_provider) =
        create_shared_loader(chain_id, 1, 0, 1).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(validators.is_empty());

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let delay_block = 2;
    let target_block = start_block + 2000;

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_result(fetcher_result).await;
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block),
            "StopEvent".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_one_validator() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler, listeners, mock_provider) =
        create_shared_loader(chain_id, 1, 1, 1).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let delay_block = 2;
    let target_block = start_block + 2000;

    // validator all contract data success
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
    fetchers[0].set_result(fetcher_result).await;
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
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
            "StopEvent".to_string()
        ]
    );

    // validator two contract data success
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
            "StopEvent".to_string()
        ]
    );

    // validator meet error
    mock_provider.push(U64::from(target_block)).unwrap();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string())))
        .await;
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(handler.drain_data().await.is_empty());
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
            "StopEvent".to_string()
        ]
    );

    // validator contract1 error
    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    validators[0]
        .set_result(Ok(ChainResult::builder()
            .chain_id(chain_id)
            .contract_results(vec![
                ContractResult::builder()
                    .address(contract_address1.to_string())
                    .result(Ok(()))
                    .build(),
                ContractResult::builder()
                    .address(contract_address2.to_string())
                    .result(Err(anyhow::Error::msg("error".to_string())))
                    .build(),
            ])
            .build()))
        .await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), Some(2)).await;
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
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
            "StopEvent".to_string()
        ]
    );

    // validator contract2 error
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block - delay_block)
        .build()];
    validators[0]
        .set_result(Ok(ChainResult::builder()
            .chain_id(chain_id)
            .contract_results(vec![
                ContractResult::builder()
                    .address(contract_address2.to_string())
                    .result(Ok(()))
                    .build(),
                ContractResult::builder()
                    .address(contract_address1.to_string())
                    .result(Err(anyhow::Error::msg("error".to_string())))
                    .build(),
            ])
            .build()))
        .await;
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
            format!(
                "LoadFailureEvent-{:?}-{:?}-loader run error failed fetch from all fetchers",
                start_block,
                target_block - delay_block
            ),
            "StopEvent".to_string()
        ]
    );
}

#[tokio::test]
async fn test_loader_start_two_validator() {
    // todo
}
