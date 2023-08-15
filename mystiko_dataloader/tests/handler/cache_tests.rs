use async_trait::async_trait;
use mystiko_config::wrapper::contract::ContractConfig;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_dataloader::data::{ChainData, ChainResult, FullData, LoadedData};
use mystiko_dataloader::handler::{
    CachedDataHandler, CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption,
    QueryResult,
};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::{biguint_to_bytes, bytes_to_biguint};
use num_bigint::BigUint;
use std::cmp::min;
use std::sync::Arc;
use tokio::sync::Mutex;
use typed_builder::TypedBuilder;

#[tokio::test]
async fn test_query_loading_contracts() {
    let config = MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let contract_config = config
        .find_contract_by_address(1, "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01")
        .unwrap();
    let handler = MockHandler::<FullData>::builder()
        .chain_id(1u64)
        .loading_contracts(Some(vec![contract_config.clone()]))
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let loading_contracts = cached_handler.query_loading_contracts(1).await.unwrap();
    assert_eq!(1, loading_contracts.unwrap().len());
}

#[tokio::test]
async fn test_query_chain_loaded_block() {
    let handler = MockHandler::<FullData>::builder()
        .chain_id(1u64)
        .chain_loaded_block(Some(1000))
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let chain_loaded_block = cached_handler.query_chain_loaded_block(1).await.unwrap();
    assert_eq!(Some(1000), chain_loaded_block);
}

#[tokio::test]
async fn test_query_contract_loaded_block() {
    let handler = MockHandler::<FullData>::builder()
        .chain_id(1u64)
        .contract_loaded_block(Some(1000))
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let contract_loaded_block = cached_handler
        .query_contract_loaded_block(1, "0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01")
        .await
        .unwrap();
    assert_eq!(Some(1000), contract_loaded_block);
}

#[tokio::test]
async fn test_query_commitment() {
    let commitments = generate_commitments(100, 1000);
    let first_commitment = commitments.first().unwrap().clone();
    let handler = MockHandler::<FullData>::builder()
        .commitments(commitments.clone())
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let commitment = cached_handler
        .query_commitment(
            &CommitmentQueryOption::builder()
                .chain_id(1u64)
                .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
                .end_block(2000u64)
                .commitment_hash(vec![BigUint::from(1u32)])
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first_commitment, commitment);
}

#[tokio::test]
async fn test_query_commitments() {
    let commitments = generate_commitments(1000, 1000);
    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .commitments(commitments.clone())
            .contract_loaded_block(Some(2009u64))
            .build(),
    );
    let mut cached_handler = CachedDataHandler::new(handler.clone());

    let mut options = CommitmentQueryOption::builder()
        .chain_id(1u64)
        .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
        .end_block(2000u64)
        .build();
    let mut queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2000, queried_commitments.end_block);
    assert_eq!(3000, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.start_block = Some(1500u64);
    options.status = Some(CommitmentStatus::SrcSucceeded);
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2000, queried_commitments.end_block);
    assert_eq!(500, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.status = Some(CommitmentStatus::Queued);
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2000, queried_commitments.end_block);
    assert_eq!(500, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.status = Some(CommitmentStatus::Included);
    options.start_block = None;
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2000, queried_commitments.end_block);
    assert_eq!(991, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    let commitments = generate_commitments(1000, 2010);
    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .commitments(commitments.clone())
            .contract_loaded_block(Some(3019u64))
            .build(),
    );
    cached_handler.raw = handler.clone();
    options.status = None;
    options.end_block = 4000;
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(3019, queried_commitments.end_block);
    assert_eq!(6000, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.commitment_hash = Some(
        commitments
            .iter()
            .filter(|c| c.status == CommitmentStatus::Included as i32)
            .take(100)
            .map(|c| bytes_to_biguint(&c.commitment_hash))
            .collect::<Vec<_>>(),
    );
    options.status = Some(CommitmentStatus::Included);
    options.end_block = 3019u64;
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(3019, queried_commitments.end_block);
    assert_eq!(100, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);
}

#[tokio::test]
async fn test_count_commitments() {
    let handler = MockHandler::<FullData>::builder()
        .contract_loaded_block(Some(1800u64))
        .commitments_count(1000u64)
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let count = cached_handler
        .count_commitments(
            &CommitmentQueryOption::builder()
                .chain_id(1u64)
                .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
                .end_block(2000u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(1000, count.result);
    assert_eq!(1800, count.end_block);
}

#[tokio::test]
async fn test_query_nullifier() {
    let nullifiers = generate_nullifiers(100, 1000);
    let first_nullifier = nullifiers.first().unwrap().clone();
    let handler = MockHandler::<FullData>::builder()
        .nullifiers(nullifiers.clone())
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let nullifier = cached_handler
        .query_nullifier(
            &NullifierQueryOption::builder()
                .chain_id(1u64)
                .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
                .end_block(2000u64)
                .nullifier(vec![BigUint::from(1u32)])
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first_nullifier, nullifier);
}

#[tokio::test]
async fn test_query_nullifiers() {
    let nullifiers = generate_nullifiers(1000, 1000);
    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .nullifiers(nullifiers.clone())
            .contract_loaded_block(Some(2009u64))
            .build(),
    );
    let mut cached_handler = CachedDataHandler::new(handler.clone());

    let mut options = NullifierQueryOption::builder()
        .chain_id(1u64)
        .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
        .end_block(2000u64)
        .build();
    let mut queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(2000, queried_nullifiers.end_block);
    assert_eq!(1000, queried_nullifiers.result.len());
    assert_eq!(1, *handler.query_nullifiers_call_count.lock().await);

    options.start_block = Some(1100u64);
    options.end_block = 1500u64;
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(1500, queried_nullifiers.end_block);
    assert_eq!(401, queried_nullifiers.result.len());
    assert_eq!(1, *handler.query_nullifiers_call_count.lock().await);

    let nullifiers = generate_nullifiers(1000, 2010);
    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .nullifiers(nullifiers.clone())
            .contract_loaded_block(Some(3019u64))
            .build(),
    );
    cached_handler.raw = handler.clone();
    options.start_block = None;
    options.end_block = 4000;
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(3019, queried_nullifiers.end_block);
    assert_eq!(2000, queried_nullifiers.result.len());
    assert_eq!(1, *handler.query_nullifiers_call_count.lock().await);

    options.nullifier = Some(
        nullifiers[0..100]
            .iter()
            .map(|n| bytes_to_biguint(&n.nullifier))
            .collect::<Vec<_>>(),
    );
    options.end_block = 3019u64;
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(3019, queried_nullifiers.end_block);
    assert_eq!(100, queried_nullifiers.result.len());
    assert_eq!(1, *handler.query_nullifiers_call_count.lock().await);
}

#[tokio::test]
async fn test_count_nullifiers() {
    let handler = MockHandler::<FullData>::builder()
        .contract_loaded_block(Some(1800u64))
        .nullifiers_count(1000u64)
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let count = cached_handler
        .count_nullifiers(
            &NullifierQueryOption::builder()
                .chain_id(1u64)
                .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
                .end_block(2000u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(1000, count.result);
    assert_eq!(1800, count.end_block);
}

#[tokio::test]
async fn test_handle() {
    let config = MystikoConfig::from_json_file("tests/files/config/mystiko.json")
        .await
        .unwrap();
    let handler = Arc::new(MockHandler::<FullData>::builder().build());
    let cached_handler = CachedDataHandler::new(handler.clone());
    let _ = cached_handler
        .handle(
            &ChainData::builder().chain_id(1u64).contracts_data(vec![]).build(),
            &HandleOption::builder().config(config).build(),
        )
        .await
        .unwrap();
    assert!(*handler.handled.lock().await);
}

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
struct MockHandler<R: LoadedData> {
    chain_id: u64,
    loading_contracts: Option<Vec<ContractConfig>>,
    chain_loaded_block: Option<u64>,
    contract_loaded_block: Option<u64>,
    commitments_count: u64,
    nullifiers_count: u64,
    commitments: Vec<Commitment>,
    nullifiers: Vec<Nullifier>,
    handled: Mutex<bool>,
    query_commitments_call_count: Mutex<u32>,
    query_nullifiers_call_count: Mutex<u32>,
    _phantom: std::marker::PhantomData<R>,
}

#[async_trait]
impl<R> DataHandler<R> for MockHandler<R>
where
    R: LoadedData,
{
    async fn query_loading_contracts(
        &self,
        _chain_id: u64,
    ) -> mystiko_dataloader::handler::Result<Option<Vec<ContractConfig>>> {
        Ok(self.loading_contracts.clone())
    }

    async fn query_chain_loaded_block(&self, _chain_id: u64) -> mystiko_dataloader::handler::Result<Option<u64>> {
        Ok(self.chain_loaded_block)
    }

    async fn query_contract_loaded_block(
        &self,
        _chain_id: u64,
        _contract_address: &str,
    ) -> mystiko_dataloader::handler::Result<Option<u64>> {
        Ok(self.contract_loaded_block)
    }

    async fn query_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<Vec<Commitment>>> {
        let mut call_count = self.query_commitments_call_count.lock().await;
        *call_count += 1;
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(self.commitments.clone())
            .build())
    }

    async fn count_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<u64>> {
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(self.commitments_count)
            .build())
    }

    async fn query_nullifiers(
        &self,
        option: &NullifierQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<Vec<Nullifier>>> {
        let mut call_count = self.query_nullifiers_call_count.lock().await;
        *call_count += 1;
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(self.nullifiers.clone())
            .build())
    }

    async fn count_nullifiers(
        &self,
        option: &NullifierQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<u64>> {
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(self.nullifiers_count)
            .build())
    }

    async fn handle(&self, _data: &ChainData<R>, _option: &HandleOption) -> HandleResult {
        let mut handled = self.handled.lock().await;
        *handled = true;
        Ok(ChainResult::builder()
            .chain_id(self.chain_id)
            .contract_results(vec![])
            .build())
    }
}

fn generate_commitments(count: u64, start_block: u64) -> Vec<Commitment> {
    let mut commitments = vec![];
    for i in 0..count {
        commitments.push(
            Commitment::builder()
                .block_number(start_block + i)
                .status(CommitmentStatus::SrcSucceeded)
                .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + 3 * i)))
                .src_chain_block_number(start_block + i)
                .build(),
        );
        commitments.push(
            Commitment::builder()
                .block_number(start_block + i)
                .status(CommitmentStatus::Queued)
                .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + 3 * i + 1)))
                .build(),
        );
        commitments.push(
            Commitment::builder()
                .block_number(start_block + i)
                .status(CommitmentStatus::Included)
                .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + 3 * i + 2)))
                .included_block_number(start_block + i + 10)
                .build(),
        );
    }
    commitments
}

fn generate_nullifiers(count: u64, start_block: u64) -> Vec<Nullifier> {
    let mut nullifiers = vec![];
    for i in 0..count {
        nullifiers.push(
            Nullifier::builder()
                .block_number(start_block + i)
                .nullifier(biguint_to_bytes(&BigUint::from(start_block + i)))
                .build(),
        );
    }
    nullifiers
}
