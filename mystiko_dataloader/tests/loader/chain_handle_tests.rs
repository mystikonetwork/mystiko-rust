use crate::loader::{contract_data_partial_eq, create_loader};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::loader::DataLoader;
use mystiko_dataloader::DataLoaderError;
use std::collections::HashSet;
use std::vec;

#[tokio::test]
async fn test_loader_start_handler() {
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
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    // handle error
    handler
        .set_result(Err(anyhow::Error::msg("error".to_string()).into()))
        .await;
    let result = loader.load(None).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));

    // handle success
    handler.set_all_success().await;
    let result = loader.load(None).await;
    assert!(contract_data_partial_eq(&handler.drain_data().await, &contract_data1));
    assert!(result.is_ok());
}
