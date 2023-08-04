use crate::loader::loader_mock::{contract_data_partial_eq, create_shared_loader, loader_start};
use ethers_core::types::U64;
use mystiko_dataloader::data::chain::ChainData;
use mystiko_dataloader::data::contract::ContractData;
use std::collections::HashSet;
use std::vec;

#[tokio::test]
async fn test_loader_start_shared_handler() {
    let chain_id = 1_u64;
    let max_batch_block = 100;
    let delay_block = 2;

    let (cfg, loader, fetchers, _, handler, listeners, mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + max_batch_block - 1;

    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
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
    handler
        .set_chain_loaded_blocks(vec![target_block - delay_block, start_block - 1])
        .await;
    handler.set_result(Err(anyhow::Error::msg("error".to_string()))).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_running().await);
    assert!(!loader.is_loading().await);
    assert!(handler.drain_data().await.is_empty());
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

    handler.set_all_success().await;
    handler
        .set_chain_loaded_blocks(vec![target_block - delay_block, start_block - 1])
        .await;
    mock_provider.push(U64::from(target_block)).unwrap();
    loader_start(loader.clone(), max_batch_block).await;
    assert!(!loader.is_running().await);
    assert!(!loader.is_loading().await);
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
    assert_eq!(
        listeners[0].drain_events().await,
        vec![
            "StartEvent".to_string(),
            format!("LoadEvent-{:?}-{:?}", start_block, target_block - delay_block),
            format!("LoadSuccessEvent-{:?}-{:?}", start_block, target_block - delay_block),
            "StopEvent".to_string()
        ]
    );
}
