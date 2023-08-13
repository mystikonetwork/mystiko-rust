use log::LevelFilter;
use mockito::*;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::types::FullData;
use mystiko_dataloader::data::types::LiteData;
use mystiko_dataloader::fetcher::types::ContractFetchOptions;
use mystiko_dataloader::fetcher::types::FetchOptions;
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
        "result": [
              {
                "contractAddress": test_address,
                "startBlock": 1000000,
                "actualEndBlock": 2000000,
                "errorMsg": null,
                "error": false,
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
                ],
                "nullifiers": [
                  {
                    "nullifier": test_nullifier,
                    "blockNumber": 1222222,
                    "transactionHash": "transactionHash"
                  }
                ]
              }
        ]
    });
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = 3000000;
    let path = format!("/chains/{}/full-data", test_chain_id);
    let request1 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(Some(test_start_block))
        .end_block(Some(test_end_block))
        .build()];
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "1000000".into()),
            Matcher::UrlEncoded("endBlock".into(), "3000000".into()),
        ]))
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
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address)
        .unwrap();
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();
    let fetch_results1 = indexer_fetcher.fetch(&fetch_options).await;
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
    // fetch with error
    let block_error_msg = String::from("The endBlock cannot be smaller than startBlock of contract.");
    let test_start_block2: u64 = 4000000;
    let request2 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(Some(test_start_block2))
        .end_block(Some(test_end_block))
        .build()];
    let mock_resp2 = serde_json::json!({
        "code": 0,
        "result": [
            {
              "contractAddress": test_address.to_string(),
              "startBlock": 4000000,
              "actualEndBlock": 3000000,
              "commitments": [],
              "nullifiers": [],
              "errorMsg": block_error_msg,
              "error": true
            },
        ]
    });
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "4000000".into()),
            Matcher::UrlEncoded("endBlock".into(), "3000000".into()),
        ]))
        .match_body(Matcher::JsonString(serde_json::to_string(&request2).unwrap()))
        .with_body(serde_json::to_string(&mock_resp2).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let contract_fetch_option2 = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block2)
        .target_block(test_end_block)
        .build()];
    let fetch_options2 = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block2)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option2))
        .build();
    let fetch_results2 = indexer_fetcher.fetch(&fetch_options2).await;
    let result2 = fetch_results2.unwrap();
    assert_eq!(result2.contract_results.len(), 1);
    assert!(result2.contract_results[0].result.is_err());
    assert_eq!(
        result2.contract_results[0].result.as_ref().err().unwrap().to_string(),
        format!("fetcher contract with error: {}", block_error_msg.to_string())
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_litedata_fetch() {
    let mut mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = IndexerClientBuilder::new(&mocked_server_url).build().unwrap();
    let test_chain_id = 137;
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = 2500000;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let mock_resp = serde_json::json!({
        "code": 0,
        "result": [{
            "contractAddress": test_address,
            "startBlock": test_start_block,
            "actualEndBlock": 2000000,
            "errorMsg": null,
            "error": false,
            "commitments": [
                {
                    "commitmentHash": "test_commitment_hash1",
                    "status": "srcSucceeded",
                    "blockNumber": 1666666,
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
                    "commitmentHash": "test_commitment_hash2",
                    "status": "Queued",
                    "blockNumber": 1888888,
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
                    "commitmentHash": "test_commitment_hash3",
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
    let path = format!("/chains/{}/lite-data", test_chain_id);
    let request1 = vec![DataLoaderRequest::builder()
        .contract_address(test_address.to_string())
        .start_block(Some(test_start_block))
        .end_block(Some(test_end_block))
        .build()];
    let m = mocked_server
        .mock("post", path.as_str())
        .with_status(200)
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("startBlock".into(), "1000000".into()),
            Matcher::UrlEncoded("endBlock".into(), "2500000".into()),
        ]))
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
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address)
        .unwrap();
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block)
        .build()];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();
    let fetch_results1 = indexer_fetcher.fetch(&fetch_options).await;
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
            .commitments[1]
            .commitment_hash,
        String::from("test_commitment_hash3").as_bytes()
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
    assert_eq!(
        result1.contract_results[0]
            .result
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .commitments[2]
            .rollup_fee,
        Some("200000000000000000".as_bytes().to_vec())
    );
    m.assert_async().await;
}
