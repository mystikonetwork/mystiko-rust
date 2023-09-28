use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers_core::types::Address;
use log::LevelFilter;
use mockall::mock;
use mystiko_config::MystikoConfig;
use mystiko_dataloader::data::{DataType, FullData, LiteData};
use mystiko_dataloader::fetcher::{
    ChainLoadedBlockOptions, ContractFetchOptions, DataFetcher, FetchOptions, SequencerFetcher,
};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse, FetchContractResponse};
use mystiko_sequencer_client::v1::SequencerClientError;
use mystiko_utils::{address::string_address_to_bytes, convert::biguint_str_to_bytes, hex::decode_hex};

mock! {
    #[derive(Debug)]
    SequencerClient {}
    #[async_trait]
    impl mystiko_sequencer_client::SequencerClient<FetchChainRequest, FetchChainResponse> for SequencerClient {
        type Error = SequencerClientError;
        async fn chain_loaded_block(&self, chain_id: u64) ->Result<u64, SequencerClientError>;
        async fn contract_loaded_block(&self, chain_id: u64, contract_address: &Address) -> Result<u64, SequencerClientError>;
        async fn fetch_chain(&self, request: FetchChainRequest) -> Result<FetchChainResponse, SequencerClientError>;
        async fn health_check(&self) -> Result<(), SequencerClientError>;
    }
}

#[tokio::test]
async fn test_fulldata_fetch() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();
    let test_chain_id = 137;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let test_start_block: u64 = 1100000;
    let sequencer_fetch_size = mystiko_config.find_chain(test_chain_id).unwrap().sequencer_fetch_size();
    let test_end_block: u64 = test_start_block + sequencer_fetch_size - 1;
    let test_commitment_hash =
        biguint_str_to_bytes("16729047340041860107670974487432292244530476292857805032774393627440911777131").unwrap();
    let test_nullifier =
        biguint_str_to_bytes("12211436598431488691766057295382431251629992667648389012648413950694818133703").unwrap();
    let test_transaction_hash =
        decode_hex("0xd699146e822e2826ebb3e0ecd4abae15f1fc07096bf6fdcd2af5705df0aff753").unwrap();
    let response = build_fetch_chain_response(
        test_chain_id,
        vec![build_fetch_contract_response(
            test_address,
            test_start_block,
            test_end_block - 1,
            test_commitment_hash.clone(),
            test_transaction_hash.clone(),
            Some(test_nullifier.clone()),
            DataType::Full,
        )],
    );
    let mut client = MockSequencerClient::new();
    client
        .expect_fetch_chain()
        .returning(move |_| Ok(response.clone()))
        .times(1);
    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();

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
        .target_block(test_end_block + 1)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();

    let fetch_results1 = fetcher.fetch(&fetch_options).await;

    assert!(fetch_results1.is_ok());
    let result1 = fetch_results1.unwrap();
    assert_eq!(result1.chain_id, test_chain_id);
    assert_eq!(result1.contract_results.len(), 1);
    assert_eq!(result1.contract_results[0].address, test_address);
    assert_eq!(
        result1.contract_results[0].result.as_ref().unwrap().end_block,
        test_end_block - 1
    );
    let data = result1.contract_results[0]
        .result
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap();
    assert_eq!(data.commitments[0].commitment_hash, test_commitment_hash.clone());
    assert_eq!(
        data.commitments[0].queued_transaction_hash.clone().unwrap(),
        test_transaction_hash.clone()
    );
    assert_eq!(data.nullifiers.len(), 1);
    assert_eq!(data.nullifiers[0].nullifier, test_nullifier.clone());

    //loop fetch
    let test_start_block2: u64 = test_end_block + 1;
    let test_end_block2: u64 = test_end_block + sequencer_fetch_size;
    let response1 = build_fetch_chain_response(
        test_chain_id,
        vec![build_fetch_contract_response(
            test_address,
            test_start_block,
            test_end_block - 1,
            test_commitment_hash.clone(),
            test_transaction_hash.clone(),
            Some(test_nullifier.clone()),
            DataType::Full,
        )],
    );
    let response2 = build_fetch_chain_response(
        test_chain_id,
        vec![build_fetch_contract_response(
            test_address,
            test_start_block2,
            test_end_block2 - 1,
            test_commitment_hash.clone(),
            test_transaction_hash.clone(),
            Some(test_nullifier.clone()),
            DataType::Full,
        )],
    );
    let mut client = MockSequencerClient::new();
    client
        .expect_fetch_chain()
        .returning(move |rerquest| {
            if rerquest.start_block == test_start_block {
                Ok(response1.clone())
            } else {
                Ok(response2.clone())
            }
        })
        .times(2);
    let contract_fetch_option = vec![ContractFetchOptions::builder()
        .contract_config(contract_config.clone())
        .start_block(test_start_block)
        .target_block(test_end_block2)
        .build()];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block2 + 1)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();
    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();
    let fetch_results2 = fetcher.fetch(&fetch_options).await;
    assert!(fetch_results2.is_ok());
    let result1 = fetch_results2.unwrap();
    assert_eq!(result1.chain_id, test_chain_id);
    assert_eq!(result1.contract_results.len(), 1);
    assert_eq!(result1.contract_results[0].address, test_address);
    let contract_result = result1.contract_results[0].result.as_ref().unwrap();
    assert_eq!(contract_result.end_block, test_end_block2 - 1);
    let data = contract_result.data.as_ref().unwrap();
    assert_eq!(data.commitments.len(), 2);
    assert_eq!(data.nullifiers.len(), 2);
}

#[tokio::test]
async fn test_fulldata_fetch_all() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();

    let test_chain_id = 137;
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let sequencer_fetch_size = mystiko_config.find_chain(test_chain_id).unwrap().sequencer_fetch_size();
    let test_start_block: u64 = 1100000;
    let test_end_block: u64 = test_start_block + sequencer_fetch_size - 1;

    let test_address1 = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_address2 = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let test_commitment_hash =
        biguint_str_to_bytes("16729047340041860107670974487432292244530476292857805032774393627440911777131").unwrap();
    let test_transaction_hash =
        decode_hex("0xd699146e822e2826ebb3e0ecd4abae15f1fc07096bf6fdcd2af5705df0aff753").unwrap();
    let test_nullifier =
        biguint_str_to_bytes("12211436598431488691766057295382431251629992667648389012648413950694818133703").unwrap();
    let contract1 = build_fetch_contract_response(
        test_address1,
        test_start_block,
        test_end_block - 1,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Full,
    );
    let contract2 = build_fetch_contract_response(
        test_address2,
        test_start_block,
        test_end_block - 10,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Full,
    );
    let response = build_fetch_chain_response(test_chain_id, vec![contract1, contract2]);

    let test_start_block2: u64 = test_end_block + 1;
    let test_end_block2: u64 = test_end_block + sequencer_fetch_size;
    let test_commitment_hash =
        biguint_str_to_bytes("16739047340041860107670974487432292244530476292857805032774393627440911777131").unwrap();
    let test_transaction_hash =
        decode_hex("0xd799146e822e2826ebb3e0ecd4abae15f1fc07096bf6fdcd2af5705df0aff753").unwrap();
    let test_nullifier =
        biguint_str_to_bytes("12221436598431488691766057295382431251629992667648389012648413950694818133703").unwrap();
    let contract1 = build_fetch_contract_response(
        test_address1,
        test_start_block2,
        test_end_block2 - 1,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Full,
    );
    let contract2 = build_fetch_contract_response(
        test_address2,
        test_start_block2,
        test_end_block2 - 10,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Full,
    );
    let response2 = build_fetch_chain_response(test_chain_id, vec![contract1, contract2]);

    let mut client = MockSequencerClient::new();
    client.expect_fetch_chain().returning(move |request| {
        if request.start_block == test_start_block {
            Ok(response.clone())
        } else {
            Ok(response2.clone())
        }
    });
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block2)
        .config(Arc::clone(&mystiko_config))
        .contract_options(None)
        .build();

    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();
    let fetch_results = fetcher.fetch(&fetch_options).await;

    assert!(fetch_results.is_ok());
    let result = fetch_results.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 2);
    let data1 = result.contract_results[0]
        .result
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap();
    assert_eq!(data1.commitments.len(), 2);
    assert_eq!(data1.nullifiers.len(), 2);
    let data2 = result.contract_results[1]
        .result
        .as_ref()
        .unwrap()
        .data
        .as_ref()
        .unwrap();
    assert_eq!(data2.commitments.len(), 2);
    assert_eq!(data2.nullifiers.len(), 2);
}

#[tokio::test]
async fn test_fulldata_fetch_error() {
    let _ = env_logger::builder()
        .filter_module("mystiko_dataloader", LevelFilter::Debug)
        .try_init();

    let test_chain_id = 137;
    let test_address = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";

    // fetch with response error
    let block_error_msg = "Unknown error of response";
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let sequencer_fetch_size = mystiko_config.find_chain(test_chain_id).unwrap().sequencer_fetch_size();
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = test_start_block + sequencer_fetch_size - 1;

    let mut client = MockSequencerClient::new();
    client
        .expect_fetch_chain()
        .returning(move |_| Err(SequencerClientError::UnknownError(block_error_msg.to_string())));
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

    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();
    let fetch_results = fetcher.fetch(&fetch_options).await;
    let result = fetch_results.unwrap();
    assert_eq!(result.contract_results.len(), 1);
    assert!(result.contract_results[0].result.is_err());
    assert_eq!(
        result.contract_results[0].result.as_ref().err().unwrap().to_string(),
        format!("unknown error: {}", block_error_msg)
    );

    let test_start_block2 = test_end_block + 1;
    let fetch_options2 = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block2)
        .target_block(test_end_block)
        .config(Arc::clone(&mystiko_config))
        .contract_options(None)
        .build();

    let fetch_results2 = fetcher.fetch(&fetch_options2).await;
    assert!(fetch_results2.is_err());
    assert_eq!(
        fetch_results2.err().unwrap().to_string(),
        format!(
            "start block {} is bigger than end block {}",
            test_start_block2, test_end_block
        )
    );
}

#[tokio::test]
async fn test_lite_data_fetch() {
    let test_chain_id = 137;
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("./tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let sequencer_fetch_size = mystiko_config.find_chain(test_chain_id).unwrap().sequencer_fetch_size();
    let test_start_block: u64 = 1000000;
    let test_end_block: u64 = test_start_block + sequencer_fetch_size - 1;

    let test_address1 = "0xCB255075f38C75EAf2DE8A72897649dba9B90299";
    let test_address2 = "0x433dD8dBb9e631fbA277c91D09133Ae2616833Fa";
    let test_commitment_hash =
        biguint_str_to_bytes("7018260368219958685591690287020455882335116439695048193945341664494747090540").unwrap();
    let test_transaction_hash =
        decode_hex("0xdde18155633283b4e45fa9b8db3add12b0ca4172c21b33eacceee5b41295fe2b").unwrap();
    let test_nullifier =
        biguint_str_to_bytes("12211436598431488691766057295382431251629992667648389012648413950694818133703").unwrap();
    let contract = build_fetch_contract_response(
        test_address1,
        test_start_block,
        test_end_block - 10,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Lite,
    );
    let response = build_fetch_chain_response(test_chain_id, vec![contract]);

    let test_start_block2: u64 = test_end_block + 1;
    let test_end_block2: u64 = test_end_block + sequencer_fetch_size;
    let test_commitment_hash =
        biguint_str_to_bytes("16739047340041860107670974487432292244530476292857805032774393627440911777131").unwrap();
    let test_transaction_hash =
        decode_hex("0xd799146e822e2826ebb3e0ecd4abae15f1fc07096bf6fdcd2af5705df0aff753").unwrap();
    let test_nullifier =
        biguint_str_to_bytes("12221436598431488691766057295382431251629992667648389012648413950694818133703").unwrap();
    let contract = build_fetch_contract_response(
        test_address1,
        test_start_block2,
        test_end_block2 - 10,
        test_commitment_hash.clone(),
        test_transaction_hash.clone(),
        Some(test_nullifier.clone()),
        DataType::Lite,
    );
    let response2 = build_fetch_chain_response(test_chain_id, vec![contract]);

    let mut client = MockSequencerClient::new();
    client.expect_fetch_chain().returning(move |request| {
        if request.start_block == test_start_block {
            Ok(response.clone())
        } else {
            Ok(response2.clone())
        }
    });

    let contract_config = mystiko_config
        .find_contract_by_address(test_chain_id, test_address1)
        .unwrap();
    let contract_config2 = mystiko_config
        .find_contract_by_address(test_chain_id, test_address2)
        .unwrap();
    let contract_fetch_option = vec![
        ContractFetchOptions::builder()
            .contract_config(contract_config.clone())
            .start_block(test_start_block2)
            .target_block(test_end_block2)
            .build(),
        ContractFetchOptions::builder()
            .contract_config(contract_config2.clone())
            .start_block(test_start_block)
            .target_block(test_end_block2)
            .build(),
    ];
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(test_start_block)
        .target_block(test_end_block2)
        .config(Arc::clone(&mystiko_config))
        .contract_options(Some(contract_fetch_option))
        .build();

    let fetcher: SequencerFetcher<LiteData, MockSequencerClient> = Arc::new(client).into();

    let fetch_results = fetcher.fetch(&fetch_options).await;
    assert!(fetch_results.is_ok());
    let result = fetch_results.unwrap();
    assert_eq!(result.chain_id, test_chain_id);
    assert_eq!(result.contract_results.len(), 2);

    assert_eq!(result.contract_results[0].address, test_address1.to_string());
    let contract_result1 = result.contract_results[0].result.as_ref().unwrap();
    assert_eq!(contract_result1.start_block, test_start_block2);
    let contract1_commitments = &contract_result1.data.as_ref().unwrap().commitments;
    assert_eq!(contract1_commitments.len(), 1);
    assert_eq!(contract1_commitments[0].block_number, test_end_block2 - 10);
    assert_eq!(contract1_commitments[0].commitment_hash, test_commitment_hash);

    let contract_result2 = result.contract_results[1].result.as_ref().unwrap();
    assert_eq!(contract_result2.start_block, test_start_block);
    assert_eq!(contract_result2.data.as_ref().unwrap().commitments.len(), 2);
}

#[tokio::test]
async fn test_chain_loaded_block() {
    let expect_block_number: u64 = 10000100;
    let mut client = MockSequencerClient::new();
    client
        .expect_chain_loaded_block()
        .returning(move |_| Ok(expect_block_number));
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let options = ChainLoadedBlockOptions::builder()
        .chain_id(1u64)
        .config(config.clone())
        .build();

    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();
    assert_eq!(fetcher.chain_loaded_block(&options).await.unwrap(), expect_block_number);
}

#[tokio::test]
async fn test_unsupported_chain_error() {
    let test_chain_id: u64 = 1000;
    let client = MockSequencerClient::new();
    let mystiko_config = Arc::new(
        MystikoConfig::from_json_file("tests/files/config/mystiko.json")
            .await
            .unwrap(),
    );
    let fetch_options = FetchOptions::builder()
        .chain_id(test_chain_id)
        .start_block(1000000u64)
        .target_block(2000000u64)
        .config(Arc::clone(&mystiko_config))
        .contract_options(None)
        .build();
    let fetcher: SequencerFetcher<FullData, MockSequencerClient> = Arc::new(client).into();

    let result = fetcher.fetch(&fetch_options).await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        format!("no chain config found for chain id: {}", test_chain_id)
    );
}

fn build_fetch_chain_response(chain_id: u64, contracts: Vec<FetchContractResponse>) -> FetchChainResponse {
    FetchChainResponse::builder()
        .chain_id(chain_id)
        .contracts(contracts)
        .build()
}

fn build_fetch_contract_response(
    address: &str,
    start_block: u64,
    end_block: u64,
    commitment_hash: Vec<u8>,
    transaction_hash: Vec<u8>,
    nullifier: Option<Vec<u8>>,
    data_type: DataType,
) -> FetchContractResponse {
    match data_type {
        DataType::Full => FetchContractResponse::builder()
            .contract_address(string_address_to_bytes(address).unwrap())
            .start_block(start_block)
            .end_block(end_block)
            .commitments(vec![Commitment::builder()
                .commitment_hash(commitment_hash.clone())
                .status(CommitmentStatus::Queued as i32)
                .block_number(end_block)
                .leaf_index(Some(10u64))
                .encrypted_note(Some(vec![1u8, 2u8, 3u8]))
                .queued_transaction_hash(Some(transaction_hash))
                .build()])
            .nullifiers(vec![Nullifier::builder()
                .nullifier(nullifier.unwrap())
                .block_number(end_block)
                .transaction_hash(vec![7u8, 8u8, 9u8])
                .build()])
            .build(),
        DataType::Lite => FetchContractResponse::builder()
            .contract_address(string_address_to_bytes(address).unwrap())
            .start_block(start_block)
            .end_block(end_block)
            .commitments(vec![Commitment::builder()
                .commitment_hash(commitment_hash.clone())
                .status(CommitmentStatus::Queued as i32)
                .block_number(end_block)
                .leaf_index(Some(10u64))
                .encrypted_note(Some(vec![1u8, 2u8, 3u8]))
                .queued_transaction_hash(Some(transaction_hash))
                .build()])
            .build(),
    }
}
