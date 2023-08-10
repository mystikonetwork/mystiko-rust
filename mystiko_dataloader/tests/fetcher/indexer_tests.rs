use log::LevelFilter;
use mockito::*;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::data::types::LiteData;
use mystiko_dataloader::error::DataLoaderError;
use mystiko_dataloader::fetcher::types::ChainFetchOption;
use mystiko_dataloader::fetcher::types::ContractFetchOption;
use mystiko_dataloader::fetcher::types::FetchOption;
use mystiko_dataloader::fetcher::{indexer::IndexerFetcher, types::DataFetcher};
use mystiko_indexer_client::builder::IndexerClientBuilder;
use mystiko_indexer_client::types::commitment_spent::DataLoaderRequest;
use std::sync::Arc;

#[tokio::test]
async fn test_fulldata_fetch() {
    let _ = env_logger::builder().filter_module("", LevelFilter::Debug).try_init();
    let mut mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = IndexerClientBuilder::new(&mocked_server_url).build().unwrap();
    let test_chain_id = 137;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_nullifier = "nullifier";
    let mock_resp = serde_json::json!({
        "code": 0,
        "result": {
            "commitmentResponses": [
              {
                "contractAddress": test_address,
                "currentSyncBlockNum": 2000000,
                "commitments": [
                  {
                    "commitmentHash": "commitmentHash",
                    "status": "succeeded",
                    "blockNumber": 1111111,
                    "includedBlockNumber": 1111111,
                    "srcChainBlockNumber": null,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": "includedTransactionHash",
                    "srcChainTransactionHash": null
                  }
                ]
              }
            ],
            "nullifierResponses": [
              {
                "contractAddress": test_address,
                "currentSyncBlockNum": 2000000,
                "nullifiers": [
                  {
                    "nullifier": test_nullifier,
                    "blockNumber": 1222222,
                    "transactionHash": "transactionHash"
                  }
                ]
              }
            ]
        }
    });
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = 2000000;
    let path = format!("/chains/{}/full-data", test_chain_id);
    let request1 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(test_start_block)
        .end_block(test_end_block)
        .build()];
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_body(Matcher::JsonString(serde_json::to_string(&request1).unwrap()))
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let indexer_fetcher: IndexerFetcher<FullData> = IndexerFetcher::<FullData>::new(indexer_client);
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    // FetchOption::Contracts
    let contracts_options = vec![ContractFetchOption::builder()
        .address(test_address.to_string())
        .chain_id(test_chain_id)
        .config(Arc::clone(&mystiko_config))
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let contracts_fetch_option = FetchOption::Contracts(&contracts_options);
    let fetch_results1 = indexer_fetcher.fetch(&contracts_fetch_option).await;

    assert!(fetch_results1.is_ok());
    let result1 = fetch_results1.unwrap();
    assert_eq!(result1.chain_id, test_chain_id);
    assert_eq!(result1.contract_results.len(), 1);
    assert_eq!(result1.contract_results[0].address, test_address);
    assert_eq!(result1.contract_results[0].result.as_ref().unwrap().end_block, 2000000);
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments[0]
            .commitment_hash,
        String::from("commitmentHash").as_bytes()
    );
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .nullifiers[0]
            .nullifier,
        String::from("nullifier").as_bytes()
    );
    m.assert_async().await;
    let test_end_block2: u64 = 3000000;
    let test_contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address)
        .unwrap();
    // FetchOption::Chain
    let request2 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(test_start_block)
        .end_block(test_end_block2)
        .build()];
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_body(Matcher::JsonString(serde_json::to_string(&request2).unwrap()))
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let chain_option = ChainFetchOption::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block2)
        .config(Arc::clone(&mystiko_config))
        .contracts(vec![test_contract_config])
        .build();
    let chain_fetch_option = FetchOption::Chain(&chain_option);
    let fetch_results2 = indexer_fetcher.fetch(&chain_fetch_option).await;
    let result2 = fetch_results2.unwrap();
    assert_eq!(result2.chain_id, test_chain_id);
    assert_eq!(result2.contract_results.len(), 1);
    assert_eq!(result2.contract_results[0].address, test_address);
    assert_eq!(result2.contract_results[0].result.as_ref().unwrap().end_block, 2000000);
    assert_eq!(
        result2.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments[0]
            .commitment_hash,
        String::from("commitmentHash").as_bytes()
    );
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .nullifiers[0]
            .nullifier,
        String::from("nullifier").as_bytes()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_litedata_fetch() {
    let mut mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = IndexerClientBuilder::new(&mocked_server_url).build().unwrap();
    let test_chain_id = 1;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let mock_resp = serde_json::json!({
        "code": 0,
        "result": [{
            "contractAddress": test_address,
            "currentSyncBlockNum": 1888888,
            "commitments": [
                {
                    "commitmentHash": "test_commitment_hash3",
                    "status": "srcSucceeded",
                    "blockNumber": 1888888,
                    "includedBlockNumber": null,
                    "srcChainBlockNumber": 1666666,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": null,
                    "srcChainTransactionHash": "srcChainTransactionHash"
                },
                {
                    "commitmentHash": "test_commitment_hash1",
                    "status": "Queued",
                    "blockNumber": 1666666,
                    "includedBlockNumber": null,
                    "srcChainBlockNumber": null,
                    "leafIndex": 0,
                    "rollupFee": "200000000000000000",
                    "encryptedNote": "encryptedNote",
                    "queuedTransactionHash": "queuedTransactionHash",
                    "includedTransactionHash": null,
                    "srcChainTransactionHash": null
                },
                {
                    "commitmentHash": "test_commitment_hash2",
                    "status": "succeeded",
                    "blockNumber": 1777777,
                    "includedBlockNumber": 1777777,
                    "srcChainBlockNumber": null,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": "includedTransactionHash",
                    "srcChainTransactionHash": null
                }
            ]
        }]
    });
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = 2000000;
    let path = format!("/chains/{}/lite-data", test_chain_id);
    let request1 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(test_start_block)
        .end_block(test_end_block)
        .build()];
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_body(Matcher::JsonString(serde_json::to_string(&request1).unwrap()))
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let indexer_fetcher: IndexerFetcher<LiteData> = IndexerFetcher::<LiteData>::new(indexer_client);
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    // FetchOption::Contracts
    let contracts_options = vec![ContractFetchOption::builder()
        .address(test_address.to_string())
        .chain_id(test_chain_id)
        .config(Arc::clone(&mystiko_config))
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let contracts_fetch_option = FetchOption::Contracts(&contracts_options);
    let fetch_results1 = indexer_fetcher.fetch(&contracts_fetch_option).await;
    assert!(fetch_results1.is_ok());
    let result1 = fetch_results1.unwrap();
    assert_eq!(result1.chain_id, test_chain_id);
    assert_eq!(result1.contract_results.len(), 1);
    assert_eq!(result1.contract_results[0].address, test_address);
    assert_eq!(result1.contract_results[0].result.as_ref().unwrap().end_block, 1888888);
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments
            .len(),
        3
    );
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments[0]
            .commitment_hash,
        String::from("test_commitment_hash1").as_bytes()
    );
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments[1]
            .block_number,
        1777777
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_error_fetch() {
    let indexer_client = IndexerClientBuilder::new("http://test-url.com").build().unwrap();
    let test_chain_id: u64 = 1;
    let test_chain_id2: u64 = 1666666666;
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = 2000000;
    let test_address = String::from("address");
    let indexer_fetcher: IndexerFetcher<LiteData> = IndexerFetcher::<LiteData>::new(indexer_client);
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let chain_option = ChainFetchOption::builder()
        .chain_id(test_chain_id2)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contracts(vec![])
        .build();
    // FetchOption::Chain
    let chain_fetch_option = FetchOption::Chain(&chain_option);
    let err_results1 = indexer_fetcher.fetch(&chain_fetch_option).await;
    assert!(err_results1.is_err());
    assert_eq!(
        err_results1.err().unwrap().to_string(),
        DataLoaderError::FetcherParamsError("chain(id=1666666666) config can not found in mystiko config".to_string())
            .to_string()
    );
    // FetchOption::Contracts
    let contracts_options1 = vec![
        ContractFetchOption::builder()
            .address(test_address.to_string())
            .chain_id(test_chain_id)
            .config(Arc::clone(&mystiko_config))
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build(),
        ContractFetchOption::builder()
            .address(test_address.to_string())
            .chain_id(test_chain_id2)
            .config(Arc::clone(&mystiko_config))
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build(),
    ];
    let contracts_fetch_option1 = FetchOption::Contracts(&contracts_options1);
    let err_results2 = indexer_fetcher.fetch(&contracts_fetch_option1).await;
    assert!(err_results2.is_err());
    assert_eq!(
        err_results2.err().unwrap().to_string(),
        DataLoaderError::FetcherParamsError("contract fetch options has different chain id".to_string()).to_string()
    );
    let contracts_options2 = vec![];
    let contracts_fetch_option2 = FetchOption::Contracts(&contracts_options2);
    let err_results3 = indexer_fetcher.fetch(&contracts_fetch_option2).await;
    assert!(err_results3.is_err());
    assert_eq!(
        err_results3.err().unwrap().to_string(),
        DataLoaderError::FetcherParamsError(
            "Fetcher found ContractFetcherOptions is empty, will do nothing".to_string()
        )
        .to_string()
    );
}
