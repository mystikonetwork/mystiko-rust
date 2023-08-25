use log::LevelFilter;
use mockito::*;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::data::LiteData;
use mystiko_dataloader::fetcher::{ContractFetchOptions, DataFetcher, FetchOptions, IndexerFetcher};
use mystiko_indexer_client::builder::IndexerClientBuilder;
use mystiko_indexer_client::types::commitment_spent::DataLoaderRequest;
use mystiko_utils::convert::biguint_str_to_bytes;
use mystiko_utils::hex::decode_hex;
use std::sync::Arc;

#[tokio::test]
async fn test_fulldata_fetch() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();
    let mut mocked_server = Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let indexer_client = IndexerClientBuilder::new(&mocked_server_url).build().unwrap();
    let test_chain_id = 137;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_commitment_hash =
        "16729047340041860107670974487432292244530476292857805032774393627440911777131".to_string();
    let test_transaction_hash = "0xd699146e822e2826ebb3e0ecd4abae15f1fc07096bf6fdcd2af5705df0aff753".to_string();
    let test_nullifier = "12211436598431488691766057295382431251629992667648389012648413950694818133703".to_string();
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
                    "commitmentHash": test_commitment_hash,
                    "status": "succeeded",
                    "blockNumber": 1111111,
                    "includedBlockNumber": 1111111,
                    "srcChainBlockNumber": null,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": test_transaction_hash,
                    "srcChainTransactionHash": null
                  }
                ],
                "nullifiers": [
                  {
                    "nullifier": test_nullifier,
                    "blockNumber": 1222222,
                    "transactionHash": test_transaction_hash
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
    let indexer_fetcher: IndexerFetcher<FullData> = IndexerFetcher::<FullData>::builder()
        .indexer_client(indexer_client)
        .build();
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
        biguint_str_to_bytes(&test_commitment_hash).unwrap()
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
            .included_transaction_hash
            .as_ref()
            .unwrap(),
        decode_hex(&test_transaction_hash).as_ref().unwrap()
    );
    m.assert_async().await;
    // fetch with response error
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
        format!("fetcher contract with error {}", block_error_msg)
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
    let test_address1 = "0xCB255075f38C75EAf2DE8A72897649dba9B90299".to_string();
    let test_address2 = "0x433dD8dBb9e631fbA277c91D09133Ae2616833Fa".to_string();
    let test_commitment_hash =
        "7018260368219958685591690287020455882335116439695048193945341664494747090540".to_string();
    let test_transaction_hash = "0xdde18155633283b4e45fa9b8db3add12b0ca4172c21b33eacceee5b41295fe2b".to_string();
    let test_encrypted_note = "0xef93f085e094191fccd9c138be158b99044a4707b2105e01cfae1f1e230fb5d9b36339208042c7aa60310557812eca77d06bbfddadb8de446d22c20addf64267b6faa993f790685fc279d91550bab7b97d63f931795f47567c61d8f2ed9051e56a0367c05a615c14a48ef96a568d9c29399e207636b8f02bd90a56f9f60a313a3c726703d38745921c86f8125c013fed0ec5dc6d6721791ae6be639ba757dd35c860cb5f8ffe4d9621d9dbc1c3d05de47faf558bc65749e67cc11f4f747958071124760088afccc82626a8684074f33492".to_string();
    let mock_resp = serde_json::json!({
        "code": 0,
        "result": [{
            "contractAddress": test_address1,
            "startBlock": test_start_block,
            "actualEndBlock": 2000000,
            "errorMsg": null,
            "error": false,
            "commitments": [
                {
                    "commitmentHash": test_commitment_hash,
                    "status": "srcSucceeded",
                    "blockNumber": 1666666,
                    "includedBlockNumber": null,
                    "srcChainBlockNumber": 1666666,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": test_transaction_hash,
                    "includedTransactionHash": null,
                    "srcChainTransactionHash": null
                },
                {
                    "commitmentHash": test_commitment_hash,
                    "status": "Queued",
                    "blockNumber": 1888888,
                    "includedBlockNumber": null,
                    "srcChainBlockNumber": null,
                    "leafIndex": 0,
                    "rollupFee": "200000000000000000",
                    "encryptedNote": test_encrypted_note,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": test_transaction_hash,
                    "srcChainTransactionHash": null
                },
                {
                    "commitmentHash": test_commitment_hash,
                    "status": "succeeded",
                    "blockNumber": 1777777,
                    "includedBlockNumber": 1777777,
                    "srcChainBlockNumber": null,
                    "leafIndex": null,
                    "rollupFee": null,
                    "encryptedNote": null,
                    "queuedTransactionHash": null,
                    "includedTransactionHash": null,
                    "srcChainTransactionHash": test_transaction_hash
                }
            ]
        },
        {
            "contractAddress": test_address2,
            "startBlock": test_start_block,
            "actualEndBlock": 2000000,
            "errorMsg": null,
            "error": false,
            "commitments": [
                {
                    "commitmentHash": "test_commitment_hash",
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
                }
            ]
        }]
    });
    let path = format!("/chains/{}/lite-data", test_chain_id);
    let request1 = vec![
        DataLoaderRequest::builder()
            .contract_address(test_address1.to_string())
            .start_block(Some(test_start_block))
            .end_block(Some(test_end_block))
            .build(),
        DataLoaderRequest::builder()
            .contract_address(test_address2.to_string())
            .start_block(Some(test_start_block))
            .end_block(Some(test_end_block))
            .build(),
    ];
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
    let indexer_fetcher: IndexerFetcher<LiteData> = IndexerFetcher::<LiteData>::builder()
        .indexer_client(indexer_client)
        .build();
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, &test_address1)
        .unwrap();
    let contract_config2 = mystiko_config
        .find_contract_by_address(test_chain_id, &test_address2)
        .unwrap();
    let contract_fetch_option = vec![
        ContractFetchOptions::builder()
            .contract_config(contract_config.clone())
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build(),
        ContractFetchOptions::builder()
            .contract_config(contract_config2.clone())
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build(),
    ];
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
    assert_eq!(result1.contract_results.len(), 2);
    assert_eq!(result1.contract_results[0].address, test_address1.to_string());
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
            .commitments[2]
            .encrypted_note
            .as_ref()
            .unwrap(),
        decode_hex(&test_encrypted_note).as_ref().unwrap()
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
            .rollup_fee
            .as_ref()
            .unwrap(),
        biguint_str_to_bytes("200000000000000000").as_ref().unwrap()
    );
    assert!(result1.contract_results[1].result.is_err());
    m.assert_async().await;
}
