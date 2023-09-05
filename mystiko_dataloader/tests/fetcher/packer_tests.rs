use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use mockall::mock;
use mystiko_config::{create_raw_from_file, MystikoConfig, RawMystikoConfig, RawPackerConfig};
use mystiko_dataloader::data::{FullData, LiteData, LoadedData};
use mystiko_dataloader::fetcher::{
    ContractFetchOptions, DataFetcher, DataPackerFetcher, DataPackerFetcherV1, FetchOptions,
};
use mystiko_datapacker_client::{ChainQuery, ChainResponse};
use mystiko_protos::data::v1::{ChainData, Commitment, CommitmentStatus, ContractData, Nullifier};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

mock! {
    #[derive(Debug)]
    DataPackerClient {}

    #[async_trait]
    impl mystiko_datapacker_client::DataPackerClient<ChainData> for DataPackerClient {
        async fn query_chain(&self, query: &ChainQuery) -> Result<ChainResponse<ChainData>>;
    }
}

#[tokio::test]
async fn test_packer_fetch_all_contracts() {
    let chain_data = generate_data(
        &TestOptions::builder()
            .chain_id(1u64)
            .start_block(10000000u64)
            .end_block(10000400u64)
            .commitments_count(300u64)
            .nullifiers_count(300u64)
            .build(),
        vec![
            "0x32a026860E7a5957a3d98960239af12d92F76441",
            "0xDede369C8444324cFd75038F1F2A39C4E44F6035",
        ],
    );
    let mut client = MockDataPackerClient::new();
    client.expect_query_chain().return_once(move |_| Ok(chain_data));
    let (config, fetcher) = setup::<FullData>(client).await;
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000100u64)
        .target_block(10000199u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await.unwrap();
    assert_eq!(result.chain_id, 1);
    let mut contracts_results = result.contract_results;
    contracts_results.sort_by_key(|c| c.address.clone());
    assert_eq!(contracts_results.len(), 2);
    assert_eq!(
        contracts_results[0].address,
        "0x32a026860E7a5957a3d98960239af12d92F76441"
    );
    assert_eq!(
        contracts_results[1].address,
        "0xDede369C8444324cFd75038F1F2A39C4E44F6035"
    );
    for contract_data in contracts_results.into_iter() {
        let contract_result = contract_data.result;
        assert!(contract_result.is_ok());
        let contract_data = contract_result.unwrap();
        assert_eq!(contract_data.start_block, 10000100u64);
        assert_eq!(contract_data.end_block, 10000199u64);
        let data = contract_data.data.unwrap();
        assert_eq!(data.commitments.len(), 100);
        assert_eq!(data.nullifiers.len(), 100);
    }
}

#[tokio::test]
async fn test_fetch_lite_data() {
    let chain_data = generate_data(
        &TestOptions::builder()
            .chain_id(1u64)
            .start_block(10000000u64)
            .end_block(10000400u64)
            .commitments_count(300u64)
            .nullifiers_count(300u64)
            .build(),
        vec![
            "0x32a026860E7a5957a3d98960239af12d92F76441",
            "0xDede369C8444324cFd75038F1F2A39C4E44F6035",
        ],
    );
    let mut client = MockDataPackerClient::new();
    client.expect_query_chain().return_once(move |_| Ok(chain_data));
    let (config, fetcher) = setup::<LiteData>(client).await;
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000100u64)
        .target_block(10000199u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await.unwrap();
    let mut contracts_results = result.contract_results;
    contracts_results.sort_by_key(|c| c.address.clone());
    assert_eq!(contracts_results.len(), 2);
    assert_eq!(
        contracts_results[0].address,
        "0x32a026860E7a5957a3d98960239af12d92F76441"
    );
    assert_eq!(
        contracts_results[1].address,
        "0xDede369C8444324cFd75038F1F2A39C4E44F6035"
    );
    for contract_data in contracts_results.into_iter() {
        let contract_result = contract_data.result;
        assert!(contract_result.is_ok());
        let contract_data = contract_result.unwrap();
        assert_eq!(contract_data.start_block, 10000100u64);
        assert_eq!(contract_data.end_block, 10000199u64);
        let data = contract_data.data.unwrap();
        assert_eq!(data.commitments.len(), 100);
    }
}

#[tokio::test]
async fn test_fetch_partial_contracts() {
    let chain_data = generate_data(
        &TestOptions::builder()
            .chain_id(1u64)
            .start_block(10000000u64)
            .end_block(10000200u64)
            .commitments_count(100u64)
            .nullifiers_count(100u64)
            .build(),
        vec![
            "0xDede369C8444324cFd75038F1F2A39C4E44F6035",
            "0x32a026860E7a5957a3d98960239af12d92F76441",
        ],
    );
    let mut client = MockDataPackerClient::new();
    client.expect_query_chain().return_once(move |_| Ok(chain_data));
    let (config, fetcher) = setup::<FullData>(client).await;
    let contract_options = ContractFetchOptions::builder()
        .contract_config(
            config
                .find_contract_by_address(1, "0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .unwrap(),
        )
        .start_block(Some(10000000u64))
        .target_block(Some(10000099u64))
        .build();
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000100u64)
        .target_block(10000199u64)
        .config(config)
        .contract_options(vec![contract_options])
        .build();
    let result = fetcher.fetch(&options).await.unwrap();
    assert_eq!(result.chain_id, 1);
    let mut contracts_results = result.contract_results;
    assert_eq!(contracts_results.len(), 1);
    let contract_result = contracts_results.pop().unwrap();
    assert_eq!(contract_result.address, "0xDede369C8444324cFd75038F1F2A39C4E44F6035");
    let contract_result = contract_result.result;
    assert!(contract_result.is_ok());
    let contract_data = contract_result.unwrap();
    assert_eq!(contract_data.start_block, 10000000u64);
    assert_eq!(contract_data.end_block, 10000099u64);
    let data = contract_data.data.unwrap();
    assert_eq!(data.commitments.len(), 100);
    assert_eq!(data.nullifiers.len(), 100);
}

#[tokio::test]
async fn test_fetch_missing_contract() {
    let chain_data = generate_data(
        &TestOptions::builder()
            .chain_id(1u64)
            .start_block(10000000u64)
            .end_block(10000500u64)
            .commitments_count(300u64)
            .nullifiers_count(300u64)
            .build(),
        vec!["0xDede369C8444324cFd75038F1F2A39C4E44F6035"],
    );
    let mut client = MockDataPackerClient::new();
    client.expect_query_chain().return_once(move |_| Ok(chain_data));
    let (config, fetcher) = setup::<FullData>(client).await;
    let contracts_options = vec![
        ContractFetchOptions::builder()
            .contract_config(
                config
                    .find_contract_by_address(1, "0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                    .unwrap(),
            )
            .start_block(Some(10000000u64))
            .target_block(Some(10000099u64))
            .build(),
        ContractFetchOptions::builder()
            .contract_config(
                config
                    .find_contract_by_address(1, "0x32a026860E7a5957a3d98960239af12d92F76441")
                    .unwrap(),
            )
            .build(),
    ];

    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000100u64)
        .target_block(10000199u64)
        .config(config)
        .contract_options(contracts_options)
        .build();
    let result = fetcher.fetch(&options).await.unwrap();

    assert_eq!(result.chain_id, 1);
    let mut contracts_results = result.contract_results;
    assert_eq!(contracts_results.len(), 2);
    contracts_results.sort_by_key(|c| c.address.clone());

    let contract_result = contracts_results.pop().unwrap();
    assert_eq!(contract_result.address, "0xDede369C8444324cFd75038F1F2A39C4E44F6035");
    let contract_result = contract_result.result;
    assert!(contract_result.is_ok());
    let contract_data = contract_result.unwrap();
    assert_eq!(contract_data.start_block, 10000000u64);
    assert_eq!(contract_data.end_block, 10000099u64);
    let data = contract_data.data.unwrap();
    assert_eq!(data.commitments.len(), 100);
    assert_eq!(data.nullifiers.len(), 100);

    let contract_result = contracts_results.pop().unwrap();
    assert_eq!(contract_result.address, "0x32a026860E7a5957a3d98960239af12d92F76441");
    let contract_result = contract_result.result;
    assert!(contract_result.is_err());
    assert_eq!(
        contract_result.unwrap_err().to_string(),
        "missing data of contract 0x32a026860E7a5957a3d98960239af12d92F76441"
    );
}

#[tokio::test]
async fn test_packer_wrong_start_block() {
    let chain_data = generate_data(
        &TestOptions::builder()
            .chain_id(1u64)
            .start_block(10000010u64)
            .end_block(10000400u64)
            .commitments_count(300u64)
            .nullifiers_count(300u64)
            .build(),
        vec!["0x32a026860E7a5957a3d98960239af12d92F76441"],
    );
    let mut client = MockDataPackerClient::new();
    client.expect_query_chain().return_once(move |_| Ok(chain_data));
    let (config, fetcher) = setup::<FullData>(client).await;
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000000u64)
        .target_block(10000199u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await.unwrap();
    assert_eq!(result.chain_id, 1);
    let mut contracts_results = result.contract_results;
    assert_eq!(contracts_results.len(), 1);
    let contract_result = contracts_results.pop().unwrap().result;
    assert!(contract_result.is_err());
    assert_eq!(
        contract_result.unwrap_err().to_string(),
        "start block 10000010 is bigger than end block 10000000"
    );
}

#[tokio::test]
async fn test_packer_raised_error() {
    let mut client = MockDataPackerClient::new();
    client
        .expect_query_chain()
        .return_once(move |_| Err(anyhow::anyhow!("query chain returned error")));
    let (config, fetcher) = setup::<FullData>(client).await;
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000000u64)
        .target_block(20000000u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await;
    assert!(result.is_err());
    assert_eq!("query chain returned error", result.unwrap_err().to_string());
}

#[tokio::test]
async fn test_packer_returned_none() {
    let mut client = MockDataPackerClient::new();
    client
        .expect_query_chain()
        .return_once(move |_| Ok(ChainResponse::builder().chain_id(1u64).build()));
    let (config, fetcher) = setup::<FullData>(client).await;
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000000u64)
        .target_block(20000000u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("no chain data found"));
}

#[tokio::test]
async fn test_packer_with_v1_client() {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock(
            "GET",
            "/chains/1/granularities/2000/blocks/10000000/data_checksum.sha512",
        )
        .with_status(500)
        .create_async()
        .await;
    let mut raw_config = create_raw_from_file::<RawMystikoConfig>("tests/files/fetcher/packer/config.json")
        .await
        .unwrap();
    raw_config.packer = Some(Arc::new(RawPackerConfig::builder().url(server.url()).build()));
    let config = Arc::new(MystikoConfig::from_raw(raw_config).unwrap());
    let fetcher = DataPackerFetcherV1::<FullData>::from(config.clone());
    let options = FetchOptions::builder()
        .chain_id(1u64)
        .start_block(10000000u64)
        .target_block(10001999u64)
        .config(config)
        .build();
    let result = fetcher.fetch(&options).await;
    mock.assert_async().await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("500 Internal Server Error"));
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct TestOptions {
    chain_id: u64,
    start_block: u64,
    end_block: u64,
    commitments_count: u64,
    nullifiers_count: u64,
}

async fn setup<R: LoadedData>(
    client: MockDataPackerClient,
) -> (Arc<MystikoConfig>, DataPackerFetcher<R, MockDataPackerClient>) {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", log::LevelFilter::Debug)
        .try_init();
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/fetcher/packer/config.json")
            .await
            .unwrap(),
    );
    let client = DataPackerFetcher::<R, MockDataPackerClient>::from(Arc::new(client));
    (config, client)
}

fn generate_data(options: &TestOptions, config_addresses: Vec<&str>) -> ChainResponse<ChainData> {
    let mut contracts_data = vec![];
    for contract_address in config_addresses.into_iter() {
        let mut commitments = vec![];
        let mut nullifiers = vec![];
        for i in 0..options.commitments_count {
            commitments.push(
                Commitment::builder()
                    .block_number(options.start_block + i)
                    .status(CommitmentStatus::Included as i32)
                    .commitment_hash(biguint_to_bytes(&BigUint::from(options.start_block + i)))
                    .build(),
            );
        }
        for i in 0..options.nullifiers_count {
            nullifiers.push(
                Nullifier::builder()
                    .block_number(options.start_block + i)
                    .transaction_hash(vec![0u8; 32])
                    .build(),
            );
        }
        contracts_data.push(
            ContractData::builder()
                .contract_address(Address::from_str(contract_address).unwrap().to_fixed_bytes())
                .commitments(commitments)
                .nullifiers(nullifiers)
                .build(),
        );
    }
    ChainResponse::builder()
        .chain_id(options.chain_id)
        .chain_data(
            ChainData::builder()
                .start_block(options.start_block)
                .end_block(options.end_block)
                .contract_data(contracts_data)
                .build(),
        )
        .build()
}
