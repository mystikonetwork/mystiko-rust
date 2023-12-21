use crate::loader::{contract_data_partial_eq, create_loader};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::fetcher::DataFetcher;
use mystiko_dataloader::loader::LoadValidatorOption;
use mystiko_dataloader::loader::LoadValidatorSkipOption;
use mystiko_dataloader::loader::{DataLoader, LoadFetcherOption, LoadOption};
use mystiko_dataloader::loader::{LoadFetcherSkipOption, LoadStatus};
use mystiko_dataloader::validator::DataValidator;
use mystiko_dataloader::DataLoaderError;
use std::collections::HashMap;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_start_one_fetcher_skip_fetch() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 2;
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
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_fetch(Some(false)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    let result = loader.load(option).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_fetch(Some(true)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    let result = loader.load(option).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(result.err().unwrap(), DataLoaderError::LoaderNoFetchersError));
}

#[tokio::test]
async fn test_loader_start_one_fetcher_only_skip_data_validation() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 2;
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
    fetchers[0].set_fetch_result(fetcher_result.clone()).await;
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_validation(Some(false)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(option).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_validation(Some(true)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(option).await.unwrap();
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

#[tokio::test]
async fn test_loader_start_one_fetcher_skip_validation_priority() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler) = create_loader(chain_id, 1, 1, true).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 2;
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
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_validation(Some(false)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(option).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let mut skip_fetcher = HashMap::new();
    skip_fetcher.insert(
        fetchers[0].name().to_string(),
        LoadFetcherSkipOption::builder().skip_validation(Some(true)).build(),
    );
    let option = LoadOption::builder()
        .fetcher(LoadFetcherOption::builder().skips(skip_fetcher).build())
        .build();
    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(option).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );
}

#[tokio::test]
async fn test_loader_start_one_fetcher_skip_validator() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, validators, handler) = create_loader(chain_id, 1, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 2;
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
    let result = loader.load(None).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );

    validators[0]
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let mut skip_validator = HashMap::new();
    skip_validator.insert(
        validators[0].name().to_string(),
        LoadValidatorSkipOption::builder().skip_validation(Some(false)).build(),
    );
    let option = LoadOption::builder()
        .validator(LoadValidatorOption::builder().skips(skip_validator).build())
        .build();
    let result = loader.load(option).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &vec![]));
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::LoaderFetchersExhaustedError
    ));

    let mut skip_validator = HashMap::new();
    skip_validator.insert(
        validators[0].name().to_string(),
        LoadValidatorSkipOption::builder().skip_validation(Some(true)).build(),
    );
    let option = LoadOption::builder()
        .validator(LoadValidatorOption::builder().skips(skip_validator).build())
        .build();
    let result = loader.load(option).await.unwrap();
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data));
    assert_eq!(
        result,
        LoadStatus::builder()
            .chain_id(chain_id)
            .loaded_block(target_block - 1)
            .target_block(target_block)
            .build()
    );
}
