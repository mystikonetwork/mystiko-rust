use mockito::{Matcher, Mock, ServerGuard};
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::{FullData, LiteData, LoadedData};
use mystiko_dataloader::fetcher::ether_scan::EtherScanFetcher;
use mystiko_dataloader::fetcher::{ContractFetchOptions, DataFetcher, FetchOptions};
use mystiko_etherscan_client::client::{EtherScanClient, EtherScanClientOptions};
use std::sync::Arc;

const CCC_MOCK_RESP: &str = "{\"status\":\"1\",\"message\":\"OK\",\"result\": []}";
const CQ_MOCK_RESP:&str = "{
        \"status\":\"1\",
        \"message\":\"OK\",
        \"result\": [
            {
                \"address\": \"0xCB255075f38C75EAf2DE8A72897649dba9B90299\",
                \"topics\": [
                \"0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9\",
                \"0x005175e35ea11a293b1a6e21cdb16c4b2d8520989127254becd38d58b0451f53\"
                ],
                \"data\": \"0x00000000000000000000000000000000000000000000000000000000000186b0000000000000000000000000000000000000000000000000000000000000068b000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000d1380bae1482d532bf49285d0030870754042a524946348ed30425dcd8e560ed1f9bb311188d1981b46a14f93559bf1ae042076a38d91c7214cfdb673c75ad827a2a6366fb5dc4be0a6c8597ac504010febd718e1a1b18b067373e78f0e3ddb9178d9d05ec7fc8e7e53640d0863028f2a16632f46a6a6e00cf9bc17aeed040adb2b24d8cfb684bbf1c738078b66bd4267a6bd1b8dcea19afd60ac7ebd33dd325b38f51ab84e0909fdaa3e8d995bcbd8bfbab02f849bed5e5cbf5219559159441f76438f22a8d8a6fb7b9109054172e029ed8000000000000000000000000000000\",
                \"blockNumber\": \"0x2c22021\",
                \"blockHash\": \"0x7e85a9133d7c85f8033f3b79062097a3b9b14e40095db0f144891735498b8938\",
                \"timeStamp\": \"0x64d97bed\",
                \"gasPrice\": \"0x114a038eb3\",
                \"gasUsed\": \"0x3b2f4\",
                \"logIndex\": \"0xaa\",
                \"transactionHash\": \"0x5ed6c3615373908c0cd904999fca6e1761c9a6eef88ebda06ef8f879fda11dec\",
                \"transactionIndex\": \"0x23\"
            }
        ]}";
const CI_MOCK_RESP: &str = "{
        \"status\":\"1\",
        \"message\":\"OK\",
        \"result\": [ {
                \"address\": \"0xcb255075f38c75eaf2de8a72897649dba9b90299\",
                \"topics\": [
                \"0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413\",
                \"0x005175e35ea11a293b1a6e21cdb16c4b2d8520989127254becd38d58b0451f53\"
                ],
                \"data\": \"0x\",
                \"blockNumber\": \"0x2c220a8\",
                \"blockHash\": \"0x8fc6e9a9cd92e6de3c103ac21572f106416e16f41330f2db53a0e576f8eb42be\",
                \"timeStamp\": \"0x64d97d0b\",
                \"gasPrice\": \"0x116ed4529b\",
                \"gasUsed\": \"0x50fec\",
                \"logIndex\": \"0x82\",
                \"transactionHash\": \"0x9dcff0a820dfcad5d5cb8de2e45d6f6e716d86cd4f4896859d41783c5e2cb729\",
                \"transactionIndex\": \"0x27\"
            }
        ]}";
const CS_MOCK_RESP: &str = "{\
        \"status\":\"1\",\
        \"message\":\"OK\",\
        \"result\": [{
                \"address\": \"0xcb255075f38c75eaf2de8a72897649dba9b90299\",
                \"topics\": [
                \"0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf\",
                \"0x01049581d05638f4bf5b6ec9cd87309a45a520361a2cc46caa57664258a7c8c8\",
                \"0x1c894c65e0c15a4368d65c531f05de668f95d3e4d0840eae38fb2f36e64b08dc\"
                ],
                \"data\": \"0x\",
                \"blockNumber\": \"0x2be1ae2\",
                \"blockHash\": \"0x4f0b7c8bcf473044c683df57ea59414083a5dc91b264e372f2e5a9e4b0dad065\",
                \"timeStamp\": \"0x64d0b900\",
                \"gasPrice\": \"0x15c6d448e4\",
                \"gasUsed\": \"0x819c9\",
                \"logIndex\": \"0x8a\",
                \"transactionHash\": \"0x1554e19f39b543257f52771ad24016f73a5cbeb31d5c5f6f28ec852daddecc11\",
                \"transactionIndex\": \"0x11\"
            }
        ]}";
#[tokio::test]
async fn test_ether_scan_full_data_fetch() {
    let mut mocked_server = mockito::Server::new_async().await;
    let test_chain_id = 137u64;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_start_block: u64 = 46013154;
    let test_end_block: u64 = 46276776;
    let test_offset = 1000u64;
    let test_api_key = "test_api_key";

    let _m0 = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            Matcher::UrlEncoded("apikey".into(), test_api_key.into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body("{\"jsonrpc\": \"2.0\",\"id\": 1,\"result\": \"0x2f999f9\"}")
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let params = build_request_params(test_start_block, test_end_block, test_api_key, test_offset);
    let _m1 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
        CCC_MOCK_RESP,
    )
    .await;
    let _m2 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
        CQ_MOCK_RESP,
    )
    .await;
    let _m3 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
        CI_MOCK_RESP,
    )
    .await;
    let _m4 = build_mock_request(
        &mut mocked_server,
        &params,
        "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
        CS_MOCK_RESP,
    )
    .await;
    let ether_scan_fetcher =
        build_ether_scan_fetcher::<FullData>(&mocked_server.url(), test_chain_id, test_offset, test_api_key);
    let fetch_options = build_fetch_options(test_address, test_chain_id, test_start_block, test_end_block).await;
    let result = ether_scan_fetcher.fetch(&fetch_options).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, test_address);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_eq!(result.end_block, test_end_block);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.commitments[0].block_number, 46276641);
    assert_eq!(data.nullifiers.len(), 1);
    assert_eq!(data.nullifiers[0].block_number, 46013154);
}

#[tokio::test]
async fn test_ether_scan_full_data_fetch_no_contract_request() {
    let mut mocked_server = mockito::Server::new_async().await;
    let test_chain_id = 137u64;
    let test_address = "";
    let test_start_block: u64 = 46013154;
    let test_end_block: u64 = 46276776;
    let test_offset = 1000u64;
    let test_api_key = "test_api_key";

    let _m0 = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            Matcher::UrlEncoded("apikey".into(), test_api_key.into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body("{\"jsonrpc\": \"2.0\",\"id\": 1,\"result\": \"0x2f999f9\"}")
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let params = build_request_params(test_start_block, test_end_block, test_api_key, test_offset);
    let _m1 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
        CCC_MOCK_RESP,
    )
    .await;
    let _m2 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
        CQ_MOCK_RESP,
    )
    .await;
    let _m3 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
        CI_MOCK_RESP,
    )
    .await;
    let _m4 = build_mock_request(
        &mut mocked_server,
        &params,
        "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
        CS_MOCK_RESP,
    )
    .await;
    let ether_scan_fetcher =
        build_ether_scan_fetcher::<FullData>(&mocked_server.url(), test_chain_id, test_offset, test_api_key);
    let fetch_options = build_fetch_options(test_address, test_chain_id, test_start_block, test_end_block).await;
    let result = ether_scan_fetcher.fetch(&fetch_options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_ne!(result.contract_results.len(), 1);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_eq!(result.end_block, test_end_block);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.nullifiers.len(), 1);
}

#[tokio::test]
async fn test_ether_scan_lite_data_fetch() {
    let mut mocked_server = mockito::Server::new_async().await;
    let test_chain_id = 137u64;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_start_block: u64 = 46013154;
    let test_end_block: u64 = 46276776;
    let test_offset = 1000u64;
    let test_api_key = "test_api_key";
    //target_block > current_block_num
    let current_block_num = test_end_block - 1;
    let _m0 = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            Matcher::UrlEncoded("apikey".into(), test_api_key.into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body("{\"jsonrpc\": \"2.0\",\"id\": 1,\"result\": \"0x2c220a7\"}")
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let params = build_request_params(test_start_block, current_block_num, test_api_key, test_offset);
    let _m1 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
        CCC_MOCK_RESP,
    )
    .await;
    let _m2 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
        CQ_MOCK_RESP,
    )
    .await;
    let _m3 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
        CI_MOCK_RESP,
    )
    .await;
    let _m4 = build_mock_request(
        &mut mocked_server,
        &params,
        "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
        CS_MOCK_RESP,
    )
    .await;
    let ether_scan_fetcher =
        build_ether_scan_fetcher::<LiteData>(&mocked_server.url(), test_chain_id, test_offset, test_api_key);
    let fetch_options = build_fetch_options(test_address, test_chain_id, test_start_block, test_end_block).await;
    let result = ether_scan_fetcher.fetch(&fetch_options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 1);
    assert_eq!(result.contract_results[0].address, test_address);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_ne!(result.end_block, test_end_block);
    assert_eq!(result.address, test_address);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.commitments[1].block_number, 46276776);
}

#[tokio::test]
async fn test_ether_scan_lite_data_fetch_no_contract_request() {
    let mut mocked_server = mockito::Server::new_async().await;
    let test_chain_id = 137u64;
    let test_address = "";
    let test_start_block: u64 = 46013154;
    let test_end_block: u64 = 46276776;
    let test_offset = 1000u64;
    let test_api_key = "test_api_key";
    //target_block > current_block_num
    let current_block_num = test_end_block - 1;
    let _m0 = mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            Matcher::UrlEncoded("apikey".into(), test_api_key.into()),
            Matcher::UrlEncoded("module".into(), "proxy".into()),
        ]))
        .with_status(200)
        .with_body("{\"jsonrpc\": \"2.0\",\"id\": 1,\"result\": \"0x2c220a7\"}")
        .with_header("content-type", "application/json")
        .create_async()
        .await;
    let params = build_request_params(test_start_block, current_block_num, test_api_key, test_offset);
    let ccc_mock_resp: &str = "{
        \"status\":\"1\",
        \"message\":\"OK\",
        \"result\": [
            {
                \"address\": \"0xdc15e18859d9e790144f29e8612ce329036e5eb4\",
                \"topics\": [
                \"0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe\",
                \"0x13bb2050b0b9de719537a04813ba124874d5d60ddc21a0ab3a8316cd63bb6b8b\"
                ],
                \"data\": \"0x\",
                \"blockNumber\": \"0x126c905\",
                \"blockHash\": \"0x1882f125044db59cbf15cb83908226f0f84753a7f7ee95222cad0f03e2e5d34b\",
                \"timeStamp\": \"0x628098b5\",
                \"gasPrice\": \"0x28fa6ae00\",
                \"gasUsed\": \"0x201d6\",
                \"logIndex\": \"0x6\",
                \"transactionHash\": \"0x71b037d077b025344da74e9882b5d641dd506a45819bc3f919ea656d327bb75a\",
                \"transactionIndex\": \"0x3\"
            }
        ]}";
    let _m1 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xd106eb38b3368b7c294e36fae5513fdefe880be5abfad529b37b044f2fdd2dbe",
        ccc_mock_resp,
    )
    .await;
    let _m2 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xf533f9705aac5020e21695ea3553ac7b6881070d2b6900ab2b1e3050304b5bf9",
        CQ_MOCK_RESP,
    )
    .await;
    let _m3 = build_mock_request(
        &mut mocked_server,
        &params,
        "0xfe6b097b46a78e08506a3143b6337c2505ba77df76fe05c3663a987395d63413",
        CI_MOCK_RESP,
    )
    .await;
    let _m4 = build_mock_request(
        &mut mocked_server,
        &params,
        "0x3c2372ab6130817bd6b8fc6dbaecae947e84201b49535d358debaa6c34c23ecf",
        CS_MOCK_RESP,
    )
    .await;
    let ether_scan_fetcher =
        build_ether_scan_fetcher::<LiteData>(&mocked_server.url(), test_chain_id, test_offset, test_api_key);
    let fetch_options = build_fetch_options(test_address, test_chain_id, test_start_block, test_end_block).await;
    let result = ether_scan_fetcher.fetch(&fetch_options).await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_ne!(result.contract_results.len(), 1);
    assert!(result.contract_results[0].result.is_ok());
    let result = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(result.start_block, test_start_block);
    assert_ne!(result.end_block, test_end_block);
    assert!(result.data.is_some());
    let data = result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 3);
}

async fn build_mock_request(mocked_server: &mut ServerGuard, params: &[Matcher], topic: &str, resp: &str) -> Mock {
    let mut params_with_topic = params.to_owned();
    params_with_topic.push(Matcher::UrlEncoded("topic0".into(), topic.into()));
    mocked_server
        .mock("GET", "/api")
        .match_query(Matcher::AllOf(params_with_topic))
        .with_status(200)
        .with_body(resp)
        .with_header("content-type", "application/json")
        .create_async()
        .await
}

fn build_request_params(
    test_start_block: u64,
    test_end_block: u64,
    test_api_key: &str,
    test_offset: u64,
) -> Vec<Matcher> {
    let params = vec![
        Matcher::UrlEncoded("action".into(), "getLogs".into()),
        Matcher::UrlEncoded("module".into(), "logs".into()),
        Matcher::UrlEncoded("fromBlock".into(), test_start_block.to_string()),
        Matcher::UrlEncoded("toBlock".into(), test_end_block.to_string()),
        Matcher::UrlEncoded("offset".into(), test_offset.to_string()),
        Matcher::UrlEncoded("apikey".into(), test_api_key.into()),
    ];
    params
}

fn build_ether_scan_fetcher<R: LoadedData + std::fmt::Debug>(
    mocked_server_url: &str,
    test_chain_id: u64,
    test_offset: u64,
    test_api_key: &str,
) -> EtherScanFetcher<R> {
    let ether_scan_client = EtherScanClient::new(
        EtherScanClientOptions::builder()
            .chain_id(test_chain_id)
            .offset(test_offset)
            .base_url(mocked_server_url)
            .api_key(test_api_key.to_string())
            .build(),
    )
    .unwrap();

    let ether_scan_fetcher: EtherScanFetcher<R> = EtherScanFetcher::<R>::builder()
        .ether_scan_client(ether_scan_client)
        .build();
    ether_scan_fetcher
}

async fn build_fetch_options(
    test_address: &str,
    test_chain_id: u64,
    test_start_block: u64,
    test_end_block: u64,
) -> FetchOptions {
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    if test_address.is_empty() {
        FetchOptions::builder()
            .chain_id(test_chain_id)
            .start_block(test_start_block)
            .target_block(test_end_block)
            .config(Arc::clone(&mystiko_config))
            .contract_options(None)
            .build()
    } else {
        let contract_config = mystiko_config
            .find_contract_by_address(test_chain_id, test_address)
            .unwrap();
        let contract_fetch_option = vec![ContractFetchOptions::builder()
            .contract_config(contract_config.clone())
            .start_block(test_start_block)
            .target_block(test_end_block)
            .build()];
        FetchOptions::builder()
            .chain_id(test_chain_id)
            .start_block(test_start_block)
            .target_block(test_end_block)
            .config(Arc::clone(&mystiko_config))
            .contract_options(Some(contract_fetch_option))
            .build()
    }
}
