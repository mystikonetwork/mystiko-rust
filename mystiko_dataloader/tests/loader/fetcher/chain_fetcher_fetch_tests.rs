use crate::loader::{contract_data_partial_eq, create_loader};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::loader::{DataLoader, LoadFetcherOption, LoadOption};
use mystiko_dataloader::DataLoaderError;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_fetcher_error() {
    let chain_id = 1_u64;
    let (_, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;

    // fetch return error
    let result = loader.load(None).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // fetch return date empty
    let fetcher_result = ChainData::builder().chain_id(chain_id).contracts_data(vec![]).build();
    fetchers[0].set_fetch_result(fetcher_result.clone()).await;
    let result = loader.load(None).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_start_without_fetcher() {
    let chain_id = 1_u64;
    let (_, loader, _, _, handler) = create_loader(chain_id, 0, 1, false).await;

    // fetch return error
    let result = loader.load(None).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::QueryFetcherLoadedBlockError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_start_one_fetcher_one_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    for target_block in start_block..start_block + 5 {
        // fetch one contract success, contract loaded block < target_block
        let contract_data = vec![ContractData::builder()
            .address(contract_address1)
            .start_block(start_block)
            .end_block(target_block - 1)
            .build()];
        let fetcher_result = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        fetchers[0].set_loaded_block(Some(target_block)).await;
        fetchers[0].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await;
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

        // fetch one contract success, contract loaded block == target_block
        let contract_data = vec![ContractData::builder()
            .address(contract_address1)
            .start_block(start_block)
            .end_block(target_block)
            .build()];
        let fetcher_result = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        fetchers[0].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await;
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert!(result.is_ok());

        // fetch one contract success, contract loaded block > target_block
        let contract_data = vec![ContractData::builder()
            .address(contract_address1)
            .start_block(start_block)
            .end_block(target_block + 1)
            .build()];
        let fetcher_result = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        fetchers[0].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await;
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_loader_start_one_fetcher_two_contract() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    for target_block in start_block..start_block + 5 {
        // fetch two contract success, all contract loaded block = target_block - delay_block
        let contract_data = vec![
            ContractData::builder()
                .address(contract_address1)
                .start_block(start_block)
                .end_block(target_block)
                .build(),
            ContractData::builder()
                .address(contract_address2)
                .start_block(start_block)
                .end_block(target_block)
                .build(),
        ];
        let fetcher_result = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        fetchers[0].set_loaded_block(Some(target_block)).await;
        fetchers[0].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await;
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert!(result.is_ok());

        // fetch two contract success, one contract loaded block < target_block - delay_block
        let contract_data = vec![
            ContractData::builder()
                .address(contract_address1)
                .start_block(start_block)
                .end_block(target_block)
                .build(),
            ContractData::builder()
                .address(contract_address2)
                .start_block(start_block)
                .end_block(target_block - 1)
                .build(),
        ];
        let fetcher_result = ChainData::builder()
            .chain_id(chain_id)
            .contracts_data(contract_data.clone())
            .build();
        fetchers[0].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await;
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert!(result
            .err()
            .unwrap()
            .to_string()
            .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));
    }
}

#[tokio::test]
async fn test_loader_start_two_fetcher() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 2, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[0].set_fetch_result(fetcher1_result.clone()).await;
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_loaded_block(Some(target_block)).await;
    fetchers[1].set_fetch_result(fetcher2_result.clone()).await;
    handler.set_contracts(chain_id, HashSet::new(), cfg.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // fetch all contract success, contract loaded block == target_block
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result.is_ok());

    // fetch all contract success, contract1 loaded block < target_block - delay_block
    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - 1)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_loaded_block(Some(target_block - 1)).await;
    fetchers[0].set_fetch_result(fetcher1_result.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // fetch all contract success,  fetcher 2 return two contract data loaded block == target_block
    let contract_data2 = vec![
        ContractData::builder()
            .address(contract_address1)
            .start_block(start_block)
            .end_block(target_block)
            .build(),
        ContractData::builder()
            .address(contract_address2)
            .start_block(start_block)
            .end_block(target_block)
            .build(),
    ];
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_fetch_result(fetcher2_result.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data2));
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_loader_start_two_fetcher_with_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 2, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    // all fetch return error
    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[0].set_fetch_error_result().await;
    fetchers[1].set_loaded_block(Some(target_block)).await;
    fetchers[1].set_fetch_error_result().await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // fetcher 1 return error
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_fetch_result(fetcher2_result.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data2.clone()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // fetcher 2 return error
    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_fetch_result(fetcher1_result.clone()).await;
    fetchers[1].set_fetch_error_result().await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &contract_data1.clone()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_start_two_fetcher_with_timeout() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 2, 1, false).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let contract_address2 = "0x62121886c954d7e23077f52217b51c26ad26bE9e";

    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    contracts.insert(contract_address2);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let contract_data2 = vec![ContractData::builder()
        .address(contract_address2)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher1_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();
    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[0].set_fetch_result(fetcher1_result.clone()).await;
    let fetcher2_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_loaded_block(Some(target_block)).await;
    fetchers[1].set_fetch_result(fetcher2_result.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result.is_ok());

    let options = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().fetch_timeout_ms(20_u64).build())
        .build();
    fetchers[0].set_fetch_sleep(true).await;
    let result = loader.load(options.clone()).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data2.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    fetchers[0].set_fetch_sleep(false).await;
    fetchers[1].set_fetch_sleep(true).await;
    let result = loader.load(options.clone()).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone()].concat()
    ));
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    let options = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().fetch_timeout_ms(200_u64).build())
        .build();
    fetchers[0].set_fetch_sleep(true).await;
    fetchers[1].set_fetch_sleep(true).await;
    let result = loader.load(options).await;
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone(), contract_data2.clone()].concat()
    ));
    assert!(result.is_ok());
}
