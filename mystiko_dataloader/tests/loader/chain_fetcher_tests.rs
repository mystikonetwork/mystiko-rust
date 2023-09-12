use crate::loader::{contract_data_partial_eq, create_loader, loader_load};
use ethers_core::types::U64;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_shared_fetcher_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, mock_provider) = create_loader(chain_id, 1, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let delay_block = 2;

    // fetch return error
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

    // fetch return date empty
    let fetcher_result = ChainData::builder().chain_id(chain_id).contracts_data(vec![]).build();
    fetchers[0].set_result(fetcher_result.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));
}

#[tokio::test]
async fn test_loader_start_without_fetcher() {
    let chain_id = 1_u64;
    let (cfg, loader, _, _, handler, mock_provider) = create_loader(chain_id, 0, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let delay_block = 2;

    // fetch return error
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));
}

#[tokio::test]
async fn test_loader_start_one_shared_fetcher_one_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, mock_provider) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
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
            let result = loader_load(loader.clone(), Some(delay_block)).await;
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert!(result
                .err()
                .unwrap()
                .to_string()
                .contains("failed to fetch data from all fetchers"));

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
            let result = loader_load(loader.clone(), Some(delay_block)).await;
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert!(result.is_ok());

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
            let result = loader_load(loader.clone(), Some(delay_block)).await;
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_loader_start_one_shared_fetcher_two_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, mock_provider) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

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
            let result = loader_load(loader.clone(), Some(delay_block)).await;
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert!(result.is_ok());

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
            let result = loader_load(loader.clone(), Some(delay_block)).await;
            assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
            assert!(result
                .err()
                .unwrap()
                .to_string()
                .contains("failed to fetch data from all fetchers"));
        }
    }
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, mock_provider) = create_loader(chain_id, 2, 1, false).await;

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
    handler.set_contracts(chain_id, HashSet::new(), cfg.clone()).await;
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

    // fetch all contract success, contract loaded block == target_block - delay_block
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result.is_ok());

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data2));
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_loader_start_two_shared_fetcher_with_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler, mock_provider) = create_loader(chain_id, 2, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let delay_block = 2;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    // all fetch return error
    fetchers[0].set_error_result().await;
    fetchers[1].set_error_result().await;
    mock_provider.push(U64::from(target_block)).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data2.clone()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));

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
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data1.clone()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to fetch data from all fetchers"));
}
