use crate::loader::create_loader;
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::loader::{DataLoader, LoadOption, ResetOptions};
use mystiko_dataloader::DataLoaderError;
use std::collections::HashSet;
use std::vec;

#[tokio::test]
async fn test_loader_success() {
    let (_, loader, fetchers, _, _) = create_loader(1, 1, 1, false).await;
    fetchers[0].set_loaded_block(Some(100)).await;
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reset_success() {
    let (_, loader, _, _, _) = create_loader(1, 1, 1, false).await;
    let options = ResetOptions::builder().chain_id(1_u64).build();
    let result = loader.reset(options).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_loader_load_error() {
    let chain_id = 123456601234566_u64;
    let (_cfg, loader, _fetchers, _, _) = create_loader(chain_id, 1, 1, false).await;
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::UnsupportedChainError(_)
    ));
}

#[tokio::test]
async fn test_loader_options() {
    let options = LoadOption::builder().build();
    let ser = serde_json::to_string(&options).unwrap();
    let des = serde_json::from_str::<LoadOption>(&ser).unwrap();
    assert_eq!(options, des);
}

#[tokio::test]
async fn test_loader_chain_loaded_block() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 1, 1, false).await;
    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

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
    fetchers[0].set_loaded_block(Some(target_block)).await;
    fetchers[0].set_fetch_result(fetcher_result).await;
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts.clone(), cfg.clone()).await;
    handler.set_all_success().await;
    let result = loader.load(None).await;
    assert!(result.is_ok());
    let chain_loaded_block = loader.chain_loaded_block(chain_id).await;
    assert_eq!(chain_loaded_block.unwrap(), Some(target_block));
    let contract_loaded_block = loader.contract_loaded_block(chain_id, contract_address1).await;
    assert_eq!(contract_loaded_block.unwrap(), Some(target_block));

    handler.set_chain_loaded_blocks_error().await;
    let chain_loaded_block = loader.chain_loaded_block(chain_id).await;
    assert!(chain_loaded_block.is_err());

    handler.set_contract_loaded_blocks_error(contract_address1).await;
    let contract_loaded_block = loader.contract_loaded_block(chain_id, contract_address1).await;
    assert!(contract_loaded_block.is_err());
}
