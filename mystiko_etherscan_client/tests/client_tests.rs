extern crate ethers_providers;
extern crate mystiko_etherscan_client;

use std::str::FromStr;

use ethers_core::types::H256;
use mockito::*;
use mystiko_abi::commitment_pool::CommitmentQueuedFilter;
use mystiko_etherscan_client::{
    client::{EtherScanClient, EtherScanClientOptions, EtherScanModule, GetLogsOptions, GetOptions},
    errors::EtherScanError,
    log::Log,
};

struct TestClientSetupData {
    mocked_server: mockito::ServerGuard,
    ether_scan_client: EtherScanClient,
}

async fn setup() -> Result<TestClientSetupData, Error> {
    let mocked_server = mockito::Server::new_async().await;
    let mocked_server_url = mocked_server.url();
    let ether_scan_client = create_ether_scan_client(&mocked_server_url);
    Ok(TestClientSetupData {
        mocked_server,
        ether_scan_client,
    })
}

fn create_ether_scan_client(url: &str) -> EtherScanClient {
    let options = EtherScanClientOptions::builder()
        .chain_id(56u64)
        .api_key("test_api_key".to_owned())
        .base_url(url.to_string())
        .build();
    EtherScanClient::new(options).unwrap()
}

#[tokio::test]
async fn test_eth_call() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();
    let expect_result = "0x00000000000000000000000000000000000000000000000000601d8888141c00";
    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": expect_result
    });
    let test_to = "test_to";
    let test_function_encoded_data = "test_function_encoded_data";
    let test_block_tag = "test_block_tag";
    let params = Matcher::AllOf(vec![
        Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
        Matcher::UrlEncoded("module".into(), "proxy".into()),
        Matcher::UrlEncoded("action".into(), "eth_call".into()),
        Matcher::UrlEncoded("to".into(), test_to.into()),
        Matcher::UrlEncoded("data".into(), test_function_encoded_data.into()),
        Matcher::UrlEncoded("tag".into(), test_block_tag.to_string()),
    ]);
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client
        .eth_call(test_to, test_function_encoded_data, Some(test_block_tag))
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, expect_result);
    m.assert_async().await;

    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1
    });
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(1)
        .create_async()
        .await;
    let result = ether_scan_client
        .eth_call(test_to, test_function_encoded_data, Some(test_block_tag))
        .await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let err_msg = error.to_string();
    assert_eq!(
        err_msg,
        EtherScanError::UnknownError("eth call error".to_string()).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_get_block_number() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": "0x10"
    });

    let params = Matcher::AllOf(vec![
        Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
        Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
        Matcher::UrlEncoded("module".into(), "proxy".into()),
    ]);
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client.get_block_number().await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.as_u64(), 16);
    m.assert_async().await;

    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1
    });
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(1)
        .create_async()
        .await;
    let result = ether_scan_client.get_block_number().await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let err_msg = error.to_string();
    assert_eq!(
        err_msg,
        EtherScanError::MissingCurrentBlock("get block number error".to_string()).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_get_block_by_number() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "baseFeePerGas":"0x5cfe76044",
            "difficulty":"0x1b4ac252b8a531",
            "extraData":"0xd883010a06846765746888676f312e31362e36856c696e7578",
            "gasLimit":"0x1caa87b",
            "gasUsed":"0x5f036a",
            "hash":"0x396288e0ad6690159d56b5502a172d54baea649698b4d7af2393cf5d98bf1bb3",
            "logsBloom":"0x5020418e211832c600000411c00098852850124700800500580d406984009104010420410c00420080414b044000012202448082084560844400d00002202b1209122000812091288804302910a246e25380282000e00002c00050009038cc205a018180028225218760100040820ac12302840050180448420420b000080000410448288400e0a2c2402050004024a240200415016c105844214060005009820302001420402003200452808508401014690208808409000033264a1b0d200c1200020280000cc0220090a8000801c00b0100a1040a8110420111870000250a22dc210a1a2002409c54140800c9804304b408053112804062088bd700900120",
            "miner":"0x5a0b54d5dc17e0aadc383d2db43b0a0d3e029c4c",
            "mixHash":"0xc547c797fb85c788ecfd4f5d24651bddf15805acbaad2c74b96b0b2a2317e66c",
            "nonce":"0x04a99df972bd8412",
            "number":"0xc63251",
            "parentHash":"0xbb2d43395f93dab5c424421be22d874f8c677e3f466dc993c218fa2cd90ef120",
            "receiptsRoot":"0x3de3b59d208e0fd441b6a2b3b1c814a2929f5a2d3016716465d320b4d48cc1e5",
            "sha3Uncles":"0xee2e81479a983dd3d583ab89ec7098f809f74485e3849afb58c2ea8e64dd0930",
            "size":"0x6cb6",
            "stateRoot":"0x60fdb78b92f0e621049e0aed52957971e226a11337f633856d8b953a56399510",
            "timestamp":"0x6110bab2",
            "totalDifficulty":"0x612789b0aba90e580f8",
            "sealFields":[],
            "transactions":[],
            "transactionsRoot":"0xaceb14fcf363e67d6cdcec0d7808091b764b4428f5fd7e25fb18d222898ef779",
            "uncles":[]
        }
    });

    let block_number: u64 = 0x10d4f;
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_getBlockByNumber".into()),
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
            Matcher::UrlEncoded("tag".into(), format!("{:x}", block_number)),
            Matcher::UrlEncoded("boolean".into(), "false".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client.get_block_by_number(block_number).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.unwrap().number.unwrap().as_u64(), 0xc63251);
    m.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_by_hash() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let tx_hash = "0x9983332a52df5ad1dabf8fa81b1642e9383f302a399c532fc47ecb6a7a967166";
    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "blockHash": "0x4beb0710a78f560687a455d2f0faf4595a306dd5ce46006005eec82c4081efe0",
            "blockNumber": "0xa11595",
            "from": "0x25f1358276a9f93feaffeb5bf92e0a62554db3d3",
            "gas": "0x4e033",
            "gasPrice": "0x12a05f200",
            "hash": tx_hash,
            "input": "0xd3243f25000000000000000000000000000000000000000000000000000000000000153b000000000000000000000000000000000000000000000000000000000000091a",
            "nonce": "0x2a8",
            "to": "0x67d9b4921bc8b397d9b0c0ca7274ada50167e7d0",
            "transactionIndex": "0x10e",
            "value": "0x0",
            "type": "0x0",
            "v": "0x94",
            "r": "0x26a2b15bae27427462a4ec5830f4b17f5dd51bfaa1e898a6e61207be61d204a7",
            "s": "0x5e9347d499cee66a021ddb1a38b15dcf106efbc54fa3c8d1929f8c28bcc41188"
        }
    });

    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_getTransactionByHash".into()),
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
            Matcher::UrlEncoded("txhash".into(), tx_hash.into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client.get_transaction_by_hash(tx_hash).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.unwrap().hash, H256::from_str(tx_hash).unwrap());
    m.assert_async().await;
}

#[tokio::test]
async fn test_get_transaction_receipt() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let tx_hash = "0x2122b2317d6cf409846f80e829c1e45ecb30306907ba0a00a02730c78890739f";
    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "blockHash": "0x552853a401a7faa54ff6f26059a1eee86323bd2140ccfaa16209679277264ddb",
            "blockNumber": "0xa11605",
            "contractAddress": null,
            "cumulativeGasUsed": "0x264dd36",
            "effectiveGasPrice": "0x12a05f200",
            "from": "0x8c0191f0e4ed98f1f72c96d06a4a207495c5e4a5",
            "gasUsed": "0x1b348",
            "logs": [{
                "address": "0x0e09fabb73bd3ade0a17ecc321fd13a19e81ce82",
                "topics": [
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                    "0x0000000000000000000000000000000000000000000000000000000000000000",
                    "0x0000000000000000000000008488cb2f54ecb9aa1cdc1bc83bc1d200bb2f216b"
                ],
                "data": "0x00000000000000000000000000000000000000000000000006c637d7bfdd83fd",
                "blockNumber": "0xa11605",
                "transactionHash": tx_hash,
                "transactionIndex": "0xe7",
                "blockHash": "0x552853a401a7faa54ff6f26059a1eee86323bd2140ccfaa16209679277264ddb",
                "logIndex": "0x340",
                "removed": false
            }],
            "logsBloom": "0x00000000100000000000000000000000000000000400000000004000000000000000000000000000000000000008000008000000000002000000000000000000008000000000000000000008000000000000000010000002000000000010000000000000020000000000000000002800000000000000020000000410000000000000000000000000000000000000000000000000000080002000000000000000000000000040000000000000000000000000000000000000000000000000000000000002000200000000000000000000000000000000000000000000000024000000000000000000000000000000000000000008400000000000000000000000",
            "status": "0x1",
            "to": "0x73feaa1ee314f8c655e354234017be2193c9e24e",
            "transactionHash": tx_hash,
            "transactionIndex": "0xe7",
            "type": "0x0"
        }
    });
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_getTransactionReceipt".into()),
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
            Matcher::UrlEncoded("txhash".into(), tx_hash.into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client.get_transaction_receipt(tx_hash).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.unwrap().transaction_hash, H256::from_str(tx_hash).unwrap());
    m.assert_async().await;
}

#[tokio::test]
async fn test_fetch_event_logs() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let block = 12878196u64;
    let address = "0xbd3531da5cf5857e7cfaa92426877b022e612cf8".to_string();

    let tx_hash = "0x4ffd22d986913d33927a392fe4319bcd2b62f3afe1c15a2c59f77fc2cc4c20a9";
    let mock_resp = serde_json::json!({
        "status":"1",
        "message":"OK",
        "result": [
            {
                "address": &address,
                "topics": [
                    "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
                    "0x1b47c6719dfe20ead0230eb1b0ad3dc00b2de7d1fe6738749e1d1916d73ee00b"
                ],
                "data": "0x000000000000000000000000000000000000000000000000002386f26fc100000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d15c6a50d7f116ae1dc4b1f865e6e7b194048c7cf2151da8a2a910e7ad6d9ba8c31b9e153d49aead0343c10e9ec025af17b6470e648704e54227e88593fe08a9dbd6f5b2c3d655114c44b53bf4c404bbae532456dce2de0b9574122f9984e2b36fe9f3de867aa2d2ecbe11d95338eeb8d674654d09df41be2f5b06b2fd462352542e9c8ac56e5907cc325cb75a24164523899629c2c1a45a655152bd73f91b8b20a91a4b484283efc3cf02a780f1d4e48d125e6b6303e7bbdb253eaf59a8ec6499a4e3898f525b0f842cffd3dad592bb7d71000000000000000000000000000000",
                "blockNumber": "0xc48174",
                "blockHash": "0x837e109ab8b1b40ec7d1032bff82397325d85e719b97d900fa0d9aa9745b2c27",
                "timeStamp": "0x60f9ce56",
                "gasPrice": "0x2e90edd000",
                "gasUsed": "0x247205",
                "logIndex": "0xfe",
                "transactionHash": tx_hash,
                "transactionIndex": "0x69"
            },
            {
                "address": &address,
                "topics": [
                    "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
                    "0x1f58c8a20224935226d4adc3ee1e8feeb29339b17a172c1a7751cbe68939ef3a"
                ],
                "data": "0x000000000000000000000000000000000000000000000000002386f26fc100000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d15c6a50d7f116ae1dc4b1f865e6e7b194048c7cf2151da8a2a910e7ad6d9ba8c31b9e153d49aead0343c10e9ec025af17b6470e648704e54227e88593fe08a9dbd6f5b2c3d655114c44b53bf4c404bbae532456dce2de0b9574122f9984e2b36fe9f3de867aa2d2ecbe11d95338eeb8d674654d09df41be2f5b06b2fd462352542e9c8ac56e5907cc325cb75a24164523899629c2c1a45a655152bd73f91b8b20a91a4b484283efc3cf02a780f1d4e48d125e6b6303e7bbdb253eaf59a8ec6499a4e3898f525b0f842cffd3dad592bb7d71000000000000000000000000000000",
                "blockNumber": "0xc48174",
                "blockHash": "0x837e109ab8b1b40ec7d1032bff82397325d85e719b97d900fa0d9aa9745b2c27",
                "timeStamp": "0x60f9ce56",
                "gasPrice": "0x2e90edd000",
                "gasUsed": "0x247205",
                "logIndex": "0x13f",
                "transactionHash": tx_hash,
                "transactionIndex": "0x7c"
            }
        ]
    });
    let options = GetLogsOptions {
        address: "0xbd3531da5cf5857e7cfaa92426877b022e612cf8".to_string(),
        from_block: block,
        to_block: block,
    };
    let params = Matcher::AllOf(vec![
        Matcher::UrlEncoded("action".into(), "getLogs".into()),
        Matcher::UrlEncoded("module".into(), "logs".into()),
        Matcher::UrlEncoded("fromBlock".into(), block.to_string()),
        Matcher::UrlEncoded("toBlock".into(), block.to_string()),
        Matcher::UrlEncoded("offset".into(), ether_scan_client.offset.to_string()),
        Matcher::UrlEncoded("address".into(), address),
        Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
    ]);
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let result = ether_scan_client
        .fetch_event_logs::<CommitmentQueuedFilter>(options.clone())
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].raw.leaf_index.as_u64(), 1);
    assert_eq!(result[1].metadata.transaction_hash, H256::from_str(tx_hash).unwrap());
    m.assert_async().await;

    // logs.is_empty()
    let mock_resp = serde_json::json!({
        "status":"1",
        "message":"OK",
        "result":[]
    });
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(1)
        .create_async()
        .await;
    let result = ether_scan_client
        .fetch_event_logs::<CommitmentQueuedFilter>(options.clone())
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.len(), 0);
    m.assert_async().await;

    // get return None
    let mock_resp = serde_json::json!({
        "status":"1",
        "message":"OK"
    });
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(params.clone())
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(1)
        .create_async()
        .await;
    let result = ether_scan_client
        .fetch_event_logs::<CommitmentQueuedFilter>(options.clone())
        .await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.len(), 0);
    m.assert_async().await;
}

#[tokio::test]
async fn test_failed_for_max_rate_limit_reached() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let mock_resp = serde_json::json!({
        "status": "0",
        "message": "NOTOK",
        "result": "Max rate limit reached, please use API Key for higher rate limit"
    });
    //normal
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "normal".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(5)
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=normal&apikey=test_api_key",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::Normal)
        .build();
    let result = ether_scan_client.get::<String, String>(options).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual_error_msg = error.to_string();
    let expect_error_msg = mock_resp.to_string();
    assert_eq!(
        actual_error_msg,
        EtherScanError::ResponseError(format!("request failed: {}", expect_error_msg)).to_string()
    );
    m.assert_async().await;

    //JsonRpcProxy
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .expect(5)
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=proxy&apikey=test_api_key",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::JsonRpcProxy)
        .build();
    let result = ether_scan_client.get::<String, String>(options).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual_error_msg = error.to_string();
    assert_eq!(
        actual_error_msg,
        EtherScanError::ResponseError(format!("request failed with message: {}", expect_error_msg)).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_handle_response_failed_for_content_type() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": "success"
    });

    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/text")
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=proxy&apikey=test_api_key",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::JsonRpcProxy)
        .build();
    let result = ether_scan_client.handle_response::<String, String>(options).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual_error_msg = error.to_string();
    assert_eq!(
        actual_error_msg,
        EtherScanError::UnexpectedContentTypeError("application/text".to_string()).to_string()
    );
    m.assert_async().await;

    //content-type is None
    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=proxy&apikey=test_api_key",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::JsonRpcProxy)
        .build();
    let result = ether_scan_client.handle_response::<String, String>(options).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual_error_msg = error.to_string();
    assert_eq!(
        actual_error_msg,
        EtherScanError::UnexpectedContentTypeError(String::new()).to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_handle_response_failed_for_json_rpc_error() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();

    let expect_error_msg = String::from("invalid argument 0: xxxx");
    let mock_resp = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32602,
            "message": expect_error_msg
        }
    });

    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=proxy&apikey=test_api_key",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::JsonRpcProxy)
        .build();
    let result = ether_scan_client.handle_response::<String, String>(options).await;
    assert!(result.is_err());
    let error = result.unwrap_err();
    let actual_error_msg = error.to_string();
    assert_eq!(
        actual_error_msg,
        EtherScanError::JsonRpcError(ethers_providers::JsonRpcError {
            code: -32602,
            message: expect_error_msg,
            data: None,
        })
        .to_string()
    );
    m.assert_async().await;
}

#[tokio::test]
async fn test_handle_logs_with_empty_response() {
    let TestClientSetupData {
        mut mocked_server,
        ether_scan_client,
    } = setup().await.unwrap();
    let mock_resp = serde_json::json!({
        "status": "0",
        "message": "No records found",
        "result": []
    });

    let m = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("apikey".into(), "test_api_key".into()),
            Matcher::UrlEncoded("module".into(), "logs".into()),
            Matcher::UrlEncoded("action".into(), "getLogs".into()),
            Matcher::UrlEncoded("address".into(), "address".into()),
            Matcher::UrlEncoded("fromBlock".into(), "12878100".into()),
            Matcher::UrlEncoded("toBlock".into(), "12878100".into()),
        ]))
        .with_status(200)
        .with_body(serde_json::to_string(&mock_resp).unwrap())
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let options = GetOptions::<String>::builder()
        .url(format!(
            "{}/api?module=logs&apikey=test_api_key&action=getLogs&address=address&fromBlock=12878100&toBlock=12878100",
            &ether_scan_client.base_url
        ))
        .module(EtherScanModule::Normal)
        .build();
    let result = ether_scan_client.handle_response::<Vec<Log>, String>(options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_none());
    m.assert_async().await;
}
