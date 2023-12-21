use crate::loader::{contract_data_partial_eq, create_loader};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::loader::{DataLoader, LoadFetcherOption, LoadOption, LoadStatus};
use mystiko_dataloader::DataLoaderError;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_one_fetcher_loaded_block_error() {
    let chain_id = 1_u64;
    let (_, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;

    // fetch return error
    fetchers[0].set_loaded_block(None).await;
    let result = loader.load(None).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::QueryLoadedBlocksError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_many_fetcher_loaded_block_all_success() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let contract_data = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();
    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[1].set_loaded_block(Some(target_block)).await;
    fetchers[2].set_loaded_block(Some(target_block)).await;
    fetchers[3].set_loaded_block(Some(target_block)).await;

    fetchers[0].set_fetch_result(fetcher_result.clone()).await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert!(result.is_ok());

    for i in 0..3 {
        fetchers[i].set_fetch_error_result().await;
        fetchers[i + 1].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await.unwrap();
        assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
        assert_eq!(
            result,
            LoadStatus::builder()
                .chain_id(chain_id)
                .loaded_block(target_block)
                .target_block(target_block)
                .build()
        );
    }

    fetchers[3].set_fetch_error_result().await;
    let result = loader.load(None).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_many_fetcher_loaded_block_some_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let contract_data = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data.clone())
        .build();

    fetchers[0].set_loaded_block(None).await;
    fetchers[1].set_loaded_block(Some(target_block)).await;
    fetchers[2].set_loaded_block(Some(target_block)).await;
    fetchers[3].set_loaded_block(Some(target_block)).await;

    fetchers[0].set_fetch_result(fetcher_result.clone()).await;
    let result = loader.load(None).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    for i in 1..4 {
        fetchers[i - 1].set_fetch_error_result().await;
        fetchers[i].set_fetch_result(fetcher_result.clone()).await;
        let result = loader.load(None).await.unwrap();
        assert!(contract_data_partial_eq(
            &handler.drain_data().await,
            &[contract_data.clone()].concat()
        ));
        assert_eq!(
            result,
            LoadStatus::builder()
                .chain_id(chain_id)
                .loaded_block(target_block)
                .target_block(target_block)
                .build()
        );
    }
}

#[tokio::test]
async fn test_loader_many_fetcher_loaded_block_different() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - 3)
        .build()];
    let fetcher_result1 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();

    fetchers[0].set_loaded_block(Some(target_block - 3)).await;
    fetchers[1].set_loaded_block(Some(target_block - 2)).await;
    fetchers[2].set_loaded_block(Some(target_block - 1)).await;
    fetchers[3].set_loaded_block(Some(target_block)).await;

    fetchers[0].set_fetch_result(fetcher_result1.clone()).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 3)
            .target_block(target_block)
            .build()
    );

    let contract_data2 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result2 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_fetch_result(fetcher_result2.clone()).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data2.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block)
            .target_block(target_block)
            .build()
    );

    let contract_data2 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block - 2)
        .build()];
    let fetcher_result2 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    let contract_data3 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result3 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data3.clone())
        .build();
    fetchers[1].set_fetch_result(fetcher_result2.clone()).await;
    fetchers[2].set_fetch_result(fetcher_result3.clone()).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data3.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block)
            .target_block(target_block)
            .build()
    );
}

#[tokio::test]
async fn test_loader_many_fetcher_loaded_block_smaller_than_start_block() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 2, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result1 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();

    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[1].set_loaded_block(Some(target_block)).await;

    fetchers[0].set_fetch_result(fetcher_result1.clone()).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block)
            .target_block(target_block)
            .build()
    );

    fetchers[0].set_loaded_block(Some(start_block)).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block)
            .target_block(target_block)
            .build()
    );

    fetchers[0].set_loaded_block(Some(start_block - 1)).await;
    let result = loader.load(None).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    fetchers[1].set_loaded_block(Some(target_block + 1)).await;
    let result = loader.load(None).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    let contract_data2 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block + 1)
        .build()];
    let fetcher_result2 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data2.clone())
        .build();
    fetchers[1].set_fetch_result(fetcher_result2).await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data2.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block + 1)
            .target_block(target_block + 1)
            .build()
    );
}

#[tokio::test]
async fn test_loader_one_fetcher_loaded_block_timeout() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let contract_data1 = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];
    let fetcher_result1 = ChainData::builder()
        .chain_id(chain_id)
        .contracts_data(contract_data1.clone())
        .build();

    fetchers[0].set_loaded_block(Some(0)).await;
    fetchers[0].set_fetch_result(fetcher_result1.clone()).await;
    let options = LoadOption::builder()
        .fetcher(
            LoadFetcherOption::builder()
                .query_loaded_block_timeout_ms(10_u64)
                .build(),
        )
        .build();
    let result = loader.load(Some(options)).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::QueryLoadedBlocksError.to_string().as_str()));

    let options = LoadOption::builder()
        .fetcher(
            LoadFetcherOption::builder()
                .query_loaded_block_timeout_ms(200_u64)
                .build(),
        )
        .build();
    let result = loader.load(Some(options)).await.unwrap();
    assert!(contract_data_partial_eq(
        &handler.drain_data().await,
        &[contract_data1.clone()].concat()
    ));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block)
            .target_block(u64::MAX)
            .build()
    );
}
