use anyhow::Result;
use ethers_core::types::Address;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_datapacker_client::v1::DataPackerClient as DataPackerClientV1;
use mystiko_datapacker_client::v1::DataPackerClientOptions;
use mystiko_datapacker_client::{ChainQuery, DataPackerClient};
use mystiko_datapacker_common::v1::PathSchema as PathSchemaV1;
use mystiko_datapacker_common::{CheckSum, Compression, PathSchema, Sha512CheckSum, ZstdCompression};
use mystiko_protos::data::v1::{ChainData, Commitment, CommitmentStatus, ContractData, Nullifier};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::BigUint;
use prost::Message;
use std::str::FromStr;
use std::sync::Arc;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_fetch_from_beginning() {
    let (mut server, client) = setup().await.unwrap();
    let mocks = vec![
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(256000u64)
            .start_block(10000000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(8192u64)
                .queued_commitments_count(8192u64)
                .included_commitments_count(8192u64)
                .nullifiers_count(8192u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(64000u64)
            .start_block(10256000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(4096u64)
                .queued_commitments_count(4096u64)
                .included_commitments_count(4096u64)
                .nullifiers_count(4096u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(32000u64)
            .start_block(10320000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(2048u64)
                .queued_commitments_count(2048u64)
                .included_commitments_count(2048u64)
                .nullifiers_count(2048u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(16000u64)
            .start_block(10352000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(1024u64)
                .queued_commitments_count(1024u64)
                .included_commitments_count(1024u64)
                .nullifiers_count(1024u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(8000u64)
            .start_block(10368000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(512u64)
                .queued_commitments_count(512u64)
                .included_commitments_count(512u64)
                .nullifiers_count(512u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(4000u64)
            .start_block(10376000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(256u64)
                .queued_commitments_count(256u64)
                .included_commitments_count(256u64)
                .nullifiers_count(256u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(2000u64)
            .start_block(10380000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(128u64)
                .queued_commitments_count(128u64)
                .included_commitments_count(128u64)
                .nullifiers_count(128u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(64000u64)
            .start_block(10320000u64)
            .data_not_found(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(32000u64)
            .start_block(10352000u64)
            .checksum_mismatch(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(16000u64)
            .start_block(10368000u64)
            .checksum_not_found(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(8000u64)
            .start_block(10376000u64)
            .data_not_found(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(4000u64)
            .start_block(10380000u64)
            .checksum_not_found(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(2000u64)
            .start_block(10382000u64)
            .data_not_found(true)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let chain_data = client
        .query_chain(&ChainQuery::builder().chain_id(1u64).target_block(10383999u64).build())
        .await
        .unwrap()
        .chain_data
        .unwrap();
    assert_eq!(chain_data.start_block, 10000000);
    assert_eq!(chain_data.end_block, 10381999);
    assert_eq!(chain_data.contract_data.len(), 1);
    let contract_data = chain_data.contract_data.first().unwrap();
    assert_eq!(contract_data.commitments.len(), 48768);
    assert_eq!(contract_data.nullifiers.len(), 16256);
    for mock in mocks.into_iter() {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn test_fetch_from_middle() {
    let (mut server, client) = setup().await.unwrap();
    let mocks = vec![
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(4000u64)
            .start_block(10012000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(2048u64)
                .queued_commitments_count(2048u64)
                .included_commitments_count(2048u64)
                .nullifiers_count(2048u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(8000u64)
            .start_block(10016000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(1024u64)
                .queued_commitments_count(1024u64)
                .included_commitments_count(1024u64)
                .nullifiers_count(1024u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(2000u64)
            .start_block(10024000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(512u64)
                .queued_commitments_count(512u64)
                .included_commitments_count(512u64)
                .nullifiers_count(512u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(2000u64)
            .start_block(10026000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .src_succeeded_commitments_count(256u64)
                .queued_commitments_count(256u64)
                .included_commitments_count(256u64)
                .nullifiers_count(256u64)
                .build()])
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(2000u64)
            .start_block(10028000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .checksum_not_found(true)
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
        MockChainOptions::builder()
            .chain_id(1u64)
            .granularity(4000u64)
            .start_block(10024000u64)
            .contracts(vec![MockContractOptions::builder()
                .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
                .build()])
            .data_not_found(true)
            .build()
            .into_mock(&mut server)
            .await
            .unwrap(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let chain_data = client
        .query_chain(
            &ChainQuery::builder()
                .chain_id(1u64)
                .start_block(10013000u64)
                .target_block(10028499u64)
                .build(),
        )
        .await
        .unwrap()
        .chain_data
        .unwrap();
    assert_eq!(chain_data.start_block, 10013000);
    assert_eq!(chain_data.end_block, 10027999);
    assert_eq!(chain_data.contract_data.len(), 1);
    let contract_data = chain_data.contract_data.first().unwrap();
    assert_eq!(contract_data.commitments.len(), 8520);
    assert_eq!(contract_data.nullifiers.len(), 2840);
    for mock in mocks.into_iter() {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn test_fetch_unsupported_chain() {
    let (_, client) = setup().await.unwrap();
    assert!(client
        .query_chain(
            &ChainQuery::builder()
                .chain_id(1024u64)
                .target_block(10000000u64)
                .build()
        )
        .await
        .is_err());
}

#[tokio::test]
async fn test_fetch_too_small_start_block() {
    let (_, client) = setup().await.unwrap();
    assert!(client
        .query_chain(
            &ChainQuery::builder()
                .chain_id(1u64)
                .target_block(20000000u64)
                .start_block(10u64)
                .build()
        )
        .await
        .is_err());
}

#[tokio::test]
async fn test_fetch_too_big_start_block() {
    let (_, client) = setup().await.unwrap();
    assert!(client
        .query_chain(
            &ChainQuery::builder()
                .chain_id(1u64)
                .target_block(20000000u64)
                .start_block(30000000u64)
                .build()
        )
        .await
        .is_err());
}

#[tokio::test]
async fn test_fetch_non_retryable_error() {
    let (mut server, client) = setup().await.unwrap();
    let mocks = MockChainOptions::builder()
        .chain_id(1u64)
        .granularity(64000u64)
        .start_block(10000000u64)
        .contracts(vec![MockContractOptions::builder()
            .contract_address("0xDede369C8444324cFd75038F1F2A39C4E44F6035")
            .build()])
        .data_error(true)
        .build()
        .into_mock(&mut server)
        .await
        .unwrap();
    assert!(client
        .query_chain(
            &ChainQuery::builder()
                .chain_id(1u64)
                .start_block(10000000u64)
                .target_block(10063999u64)
                .build(),
        )
        .await
        .is_err());
    for mock in mocks.into_iter() {
        mock.assert_async().await;
    }
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockContractOptions {
    contract_address: String,
    nullifiers_count: u64,
    src_succeeded_commitments_count: u64,
    queued_commitments_count: u64,
    included_commitments_count: u64,
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockChainOptions {
    chain_id: u64,
    start_block: u64,
    granularity: u64,
    contracts: Vec<MockContractOptions>,
    #[builder(default = false)]
    data_not_found: bool,
    #[builder(default = false)]
    data_error: bool,
    #[builder(default = false)]
    checksum_not_found: bool,
    #[builder(default = false)]
    checksum_error: bool,
    #[builder(default = false)]
    checksum_mismatch: bool,
}

impl MockContractOptions {
    pub(crate) fn into_data(self, start_block: u64) -> ContractData {
        let commitments = vec![
            generate_commitments(
                self.src_succeeded_commitments_count,
                start_block,
                CommitmentStatus::SrcSucceeded,
            ),
            generate_commitments(self.queued_commitments_count, start_block, CommitmentStatus::Queued),
            generate_commitments(self.included_commitments_count, start_block, CommitmentStatus::Included),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let nullifiers = generate_nullifier(self.nullifiers_count, start_block);
        ContractData::builder()
            .contract_address(Address::from_str(&self.contract_address).unwrap().to_fixed_bytes())
            .commitments(commitments)
            .nullifiers(nullifiers)
            .build()
    }
}

impl MockChainOptions {
    pub(crate) fn into_data(self) -> ChainData {
        let contracts = self
            .contracts
            .into_iter()
            .map(|c| c.into_data(self.start_block))
            .collect::<Vec<_>>();
        ChainData::builder()
            .start_block(self.start_block)
            .end_block(self.start_block + self.granularity - 1)
            .contract_data(contracts)
            .build()
    }

    pub(crate) async fn into_compressed_data(self) -> Result<(Vec<u8>, String)> {
        let chain_data = self.into_data();
        let uncompressed_data = chain_data.encode_to_vec();
        let compressed_data = ZstdCompression.compress(&uncompressed_data).await?;
        let checksum = Sha512CheckSum.checksum(&compressed_data);
        Ok((compressed_data, checksum))
    }

    pub(crate) async fn into_mock(self, server: &mut mockito::ServerGuard) -> Result<Vec<mockito::Mock>> {
        let schema = PathSchemaV1::default();
        let data_path = schema
            .data_path(self.chain_id, self.granularity, self.start_block)
            .to_string_lossy()
            .to_string();
        let checksum_path = schema
            .checksum_path(self.chain_id, self.granularity, self.start_block)
            .to_string_lossy()
            .to_string();
        let data_not_found = self.data_not_found;
        let data_error = self.data_error;
        let checksum_not_found = self.checksum_not_found;
        let checksum_error = self.checksum_error;
        let checksum_mismatch = self.checksum_mismatch;
        let (compressed_data, checksum) = self.into_compressed_data().await?;
        let mut mocks = vec![];
        if checksum_not_found {
            mocks.push(
                server
                    .mock("GET", checksum_path.as_str())
                    .with_status(404)
                    .create_async()
                    .await,
            );
        } else {
            mocks.push(
                server
                    .mock("GET", checksum_path.as_str())
                    .with_status(if checksum_error { 500 } else { 200 })
                    .with_body(if checksum_mismatch {
                        String::from("mismatch")
                    } else {
                        checksum
                    })
                    .create_async()
                    .await,
            );
            if data_not_found {
                mocks.push(
                    server
                        .mock("GET", data_path.as_str())
                        .with_status(404)
                        .create_async()
                        .await,
                );
            } else {
                mocks.push(
                    server
                        .mock("GET", data_path.as_str())
                        .with_status(if data_error { 500 } else { 200 })
                        .with_body(compressed_data)
                        .create_async()
                        .await,
                );
            }
        }
        Ok(mocks)
    }
}

fn generate_commitments(count: u64, start_block: u64, status: CommitmentStatus) -> Vec<Commitment> {
    let mut commitments = vec![];
    for index in 0..count {
        match status {
            CommitmentStatus::SrcSucceeded => commitments.push(
                Commitment::builder()
                    .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + index)))
                    .status(status)
                    .block_number(start_block + index)
                    .src_chain_block_number(Some(start_block + index))
                    .src_chain_transaction_hash(Some(vec![0; 32]))
                    .build(),
            ),
            CommitmentStatus::Queued => commitments.push(
                Commitment::builder()
                    .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + index)))
                    .status(status)
                    .block_number(start_block + index)
                    .queued_transaction_hash(Some(vec![0; 32]))
                    .build(),
            ),
            CommitmentStatus::Included => commitments.push(
                Commitment::builder()
                    .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + index)))
                    .status(status)
                    .block_number(start_block + index)
                    .included_block_number(Some(start_block + index))
                    .included_transaction_hash(Some(vec![0; 32]))
                    .build(),
            ),
            _ => continue,
        }
    }
    commitments
}

fn generate_nullifier(count: u64, start_block: u64) -> Vec<Nullifier> {
    let mut nullifiers = vec![];
    for index in 0..count {
        nullifiers.push(
            Nullifier::builder()
                .nullifier(biguint_to_bytes(&BigUint::from(start_block + index)))
                .block_number(start_block + index)
                .transaction_hash(vec![0; 32])
                .build(),
        )
    }
    nullifiers
}

async fn setup() -> Result<(mockito::ServerGuard, DataPackerClientV1)> {
    let _ = env_logger::builder()
        .filter_module("mystiko_datapacker_client", log::LevelFilter::Debug)
        .try_init();
    let config = Arc::new(MystikoConfig::from_json_file("tests/files/v1/client/config.json").await?);
    let server = mockito::Server::new_async().await;
    let options = DataPackerClientOptions::builder()
        .url(server.url())
        .config(config)
        .build();
    let client = DataPackerClientV1::new(options);
    Ok((server, client))
}
