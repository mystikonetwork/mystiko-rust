use crate::loader::{contract_data_partial_eq, create_loader, loader_load};
use ethers_core::types::U64;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::data::{ChainResult, ContractResult};
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_without_validator() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler, mock_provider) = create_loader(chain_id, 1, 0, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_loader_start_one_validator() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler, mock_provider) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

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

    // validator two contract data success
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();
    fetchers[0].set_result(fetcher_result).await;
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
    validators[0].set_all_success().await;
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert!(result.is_ok());

    // validator meet error
    mock_provider.push(U64::from(target_block)).unwrap();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data2));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));
}

#[tokio::test]
async fn test_loader_skip_validation() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler, mock_provider) = create_loader(chain_id, 1, 1, true).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

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

    // validator two contract data success
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();
    fetchers[0].set_result(fetcher_result).await;
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
    validators[0].set_all_success().await;
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert!(result.is_ok());

    // validator meet error
    mock_provider.push(U64::from(target_block)).unwrap();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert!(result.is_ok());

    // validator contract1 error
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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_loader_start_two_validator() {
    // todo
}
