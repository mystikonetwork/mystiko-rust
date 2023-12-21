use crate::loader::{contract_data_partial_eq, create_loader};
use mystiko_dataloader::data::ChainData;
use mystiko_dataloader::data::ContractData;
use mystiko_dataloader::loader::{DataLoader, LoadStatus};
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[tokio::test]
async fn test_loader_many_fetcher_all_success() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let mut contract_data = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];

    for _ in 0..10 {
        let mut numbers: Vec<usize> = (0..4).collect();
        numbers.shuffle(&mut rand::thread_rng());
        for i in numbers {
            let loaded_block = target_block - i as u64;
            contract_data[0].end_block = loaded_block;
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[i].set_loaded_block(Some(loaded_block)).await;
            fetchers[i].set_fetch_result(fetcher_result.clone()).await;
        }

        let result = loader.load(None).await.unwrap();
        contract_data[0].end_block = target_block;
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
async fn test_loader_many_fetcher_some_fetcher_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let mut contract_data = vec![ContractData::builder()
        .address(contract_address1)
        .start_block(start_block)
        .end_block(target_block)
        .build()];

    for skip in 0..3 {
        let mut numbers: Vec<usize> = (0..4).collect();
        numbers.shuffle(&mut rand::thread_rng());
        for i in numbers {
            let loaded_block = target_block - i as u64;
            contract_data[0].end_block = loaded_block;
            let fetcher_result = ChainData::builder()
                .chain_id(chain_id)
                .contracts_data(contract_data.clone())
                .build();
            fetchers[i].set_loaded_block(Some(loaded_block)).await;
            if i <= skip {
                fetchers[i].set_fetch_error_result().await;
            } else {
                fetchers[i].set_fetch_result(fetcher_result.clone()).await;
            }
        }

        let result = loader.load(None).await.unwrap();
        contract_data[0].end_block = target_block - skip as u64 - 1;
        assert!(contract_data_partial_eq(
            &handler.drain_data().await,
            &[contract_data.clone()].concat()
        ));
        assert_eq!(
            result,
            LoadStatus::builder()
                .chain_id(chain_id)
                .loaded_block(target_block - skip as u64 - 1)
                .target_block(target_block)
                .build()
        );
    }
}

#[tokio::test]
async fn test_loader_many_fetcher_all_error() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, handler) = create_loader(chain_id, 4, 1, false).await;
    let contract_address1 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let mut contracts = HashSet::new();
    contracts.insert(contract_address1);
    handler.set_contracts(chain_id, contracts, cfg.clone()).await;

    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    let target_block = start_block + 1000;

    let mut numbers: Vec<usize> = (0..4).collect();
    numbers.shuffle(&mut rand::thread_rng());
    for i in numbers {
        let loaded_block = target_block - i as u64;
        fetchers[i].set_loaded_block(Some(loaded_block)).await;
        fetchers[i].set_fetch_error_result().await;
    }

    let result = loader.load(None).await;
    assert!(result.is_err());
}
