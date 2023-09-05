use async_trait::async_trait;
use mystiko_config::{ContractConfig, MystikoConfig};
use mystiko_dataloader::data::{ChainData, ChainResult, FullData, LoadedData};
use mystiko_dataloader::handler::{
    CachedDataHandler, CommitmentQueryOption, DataHandler, HandleOption, HandleResult, NullifierQueryOption,
    QueryResult,
};
use mystiko_protos::data::v1::{Commitment, CommitmentStatus, Nullifier};
use mystiko_utils::convert::{biguint_to_bytes, bytes_to_biguint};
use num_bigint::BigUint;
use std::cmp::min;
use std::collections::HashSet;
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
    let commitments = generate_commitments(100, 1000, CommitmentStatus::Included);
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
                .commitment_hash(vec![first_commitment.commitment_hash_as_biguint()])
                .build(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(first_commitment, commitment);
}

#[tokio::test]
async fn test_query_commitments() {
    let commitments = vec![
        generate_commitments(1000, 1000, CommitmentStatus::SrcSucceeded),
        generate_commitments(1000, 2000, CommitmentStatus::Included),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .commitments(commitments.clone())
            .contract_loaded_block(Some(2999u64))
            .build(),
    );
    let cached_handler = CachedDataHandler::new(handler.clone());

    let mut options = CommitmentQueryOption::builder()
        .chain_id(1u64)
        .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
        .end_block(1499u64)
        .build();
    let mut queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(1499, queried_commitments.end_block);
    assert_eq!(500, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.start_block = Some(1500u64);
    options.end_block = 2999u64;
    options.status = Some(CommitmentStatus::SrcSucceeded);
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2999, queried_commitments.end_block);
    assert_eq!(500, queried_commitments.result.len());
    assert_eq!(2, *handler.query_commitments_call_count.lock().await);

    options.status = Some(CommitmentStatus::Included);
    options.start_block = None;
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2999, queried_commitments.end_block);
    assert_eq!(1000, queried_commitments.result.len());
    assert_eq!(2, *handler.query_commitments_call_count.lock().await);

    options.status = None;
    options.commitment_hash = Some(
        commitments[0..100]
            .iter()
            .map(|c| c.commitment_hash_as_biguint())
            .collect(),
    );
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2999, queried_commitments.end_block);
    assert_eq!(100, queried_commitments.result.len());
    assert_eq!(2, *handler.query_commitments_call_count.lock().await);
}

#[tokio::test]
async fn test_query_commitments_with_queued() {
    let commitments = vec![
        generate_commitments(1000, 1000, CommitmentStatus::Included),
        generate_commitments(100, 2000, CommitmentStatus::Queued),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .commitments(commitments.clone())
            .contract_loaded_block(Some(2099u64))
            .build(),
    );
    let mut cached_handler = CachedDataHandler::new(handler.clone());
    let mut options = CommitmentQueryOption::builder()
        .chain_id(1u64)
        .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
        .end_block(1999u64)
        .build();

    let mut queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(1999u64, queried_commitments.end_block);
    assert_eq!(1000, queried_commitments.result.len());
    assert_eq!(1, *handler.query_commitments_call_count.lock().await);

    options.status = Some(CommitmentStatus::Queued);
    options.end_block = 2099u64;
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2099u64, queried_commitments.end_block);
    assert_eq!(100, queried_commitments.result.len());
    assert_eq!(2, *handler.query_commitments_call_count.lock().await);

    let handler = Arc::new(
        MockHandler::<FullData>::builder()
            .query_commitments_call_count(Mutex::new(2))
            .commitments(generate_commitments(100, 2000, CommitmentStatus::Included))
            .contract_loaded_block(Some(2099u64))
            .build(),
    );
    cached_handler.raw = handler.clone();

    options.status = Some(CommitmentStatus::Included);
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2099u64, queried_commitments.end_block);
    assert_eq!(1100, queried_commitments.result.len());
    assert_eq!(3, *handler.query_commitments_call_count.lock().await);

    options.status = Some(CommitmentStatus::Queued);
    queried_commitments = cached_handler.query_commitments(&options).await.unwrap();
    assert_eq!(2099u64, queried_commitments.end_block);
    assert_eq!(0, queried_commitments.result.len());
    assert_eq!(3, *handler.query_commitments_call_count.lock().await);
}

#[tokio::test]
async fn test_count_commitments() {
    let commitments = generate_commitments(800, 1000, CommitmentStatus::Included);
    let handler = MockHandler::<FullData>::builder()
        .commitments(commitments)
        .contract_loaded_block(Some(1800u64))
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
    assert_eq!(800, count.result);
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
                .nullifier(vec![first_nullifier.nullifier_as_biguint()])
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
    let cached_handler = CachedDataHandler::new(handler.clone());

    let mut options = NullifierQueryOption::builder()
        .chain_id(1u64)
        .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
        .end_block(1499u64)
        .build();
    let mut queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(1499, queried_nullifiers.end_block);
    assert_eq!(500, queried_nullifiers.result.len());
    assert_eq!(1, *handler.query_nullifiers_call_count.lock().await);

    options.start_block = Some(1100u64);
    options.end_block = 2000u64;
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(2000, queried_nullifiers.end_block);
    assert_eq!(900, queried_nullifiers.result.len());
    assert_eq!(2, *handler.query_nullifiers_call_count.lock().await);

    options.start_block = None;
    options.end_block = 2000;
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(2000, queried_nullifiers.end_block);
    assert_eq!(1000, queried_nullifiers.result.len());
    assert_eq!(2, *handler.query_nullifiers_call_count.lock().await);

    options.nullifier = Some(
        nullifiers[0..100]
            .iter()
            .map(|n| bytes_to_biguint(&n.nullifier))
            .collect::<Vec<_>>(),
    );
    queried_nullifiers = cached_handler.query_nullifiers(&options).await.unwrap();
    assert_eq!(2000, queried_nullifiers.end_block);
    assert_eq!(100, queried_nullifiers.result.len());
    assert_eq!(2, *handler.query_nullifiers_call_count.lock().await);
}

#[tokio::test]
async fn test_count_nullifiers() {
    let nullifiers = generate_nullifiers(1800, 1000);
    let handler = MockHandler::<FullData>::builder()
        .nullifiers(nullifiers.clone())
        .contract_loaded_block(Some(2800u64))
        .build();
    let cached_handler = CachedDataHandler::new(Arc::new(handler));
    let count = cached_handler
        .count_nullifiers(
            &NullifierQueryOption::builder()
                .chain_id(1u64)
                .contract_address("0x7F88F2A3Cf18E96844E14CaE59EC97B908734C01".to_string())
                .end_block(3000u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(1800, count.result);
    assert_eq!(2800, count.end_block);
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
            .result(filter_commitments(&self.commitments, option))
            .build())
    }

    async fn count_commitments(
        &self,
        option: &CommitmentQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<u64>> {
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(filter_commitments(&self.commitments, option).len() as u64)
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
            .result(filter_nullifiers(&self.nullifiers, option))
            .build())
    }

    async fn count_nullifiers(
        &self,
        option: &NullifierQueryOption,
    ) -> mystiko_dataloader::handler::Result<QueryResult<u64>> {
        Ok(QueryResult::builder()
            .end_block(min(option.end_block, self.contract_loaded_block.unwrap_or_default()))
            .result(filter_nullifiers(&self.nullifiers, option).len() as u64)
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

fn generate_commitments(count: u64, start_block: u64, status: CommitmentStatus) -> Vec<Commitment> {
    let mut commitments = vec![];
    for i in 0..count {
        match status {
            CommitmentStatus::SrcSucceeded => {
                commitments.push(
                    Commitment::builder()
                        .block_number(start_block + i)
                        .status(CommitmentStatus::SrcSucceeded)
                        .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + i)))
                        .src_chain_block_number(start_block + i)
                        .build(),
                );
            }
            CommitmentStatus::Queued => {
                commitments.push(
                    Commitment::builder()
                        .block_number(start_block + i)
                        .status(CommitmentStatus::Queued)
                        .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + i)))
                        .build(),
                );
            }
            CommitmentStatus::Included => {
                commitments.push(
                    Commitment::builder()
                        .block_number(start_block + i)
                        .status(CommitmentStatus::Included)
                        .commitment_hash(biguint_to_bytes(&BigUint::from(start_block + i)))
                        .included_block_number(start_block + i)
                        .build(),
                );
            }
            _ => continue,
        }
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

fn filter_commitments(commitments: &[Commitment], option: &CommitmentQueryOption) -> Vec<Commitment> {
    let mut filtered_commitments = vec![];
    for commitment in commitments.iter() {
        let block_number = if commitment.status == CommitmentStatus::Queued as i32 {
            Some(commitment.block_number)
        } else if commitment.status == CommitmentStatus::Included as i32 {
            commitment.included_block_number
        } else {
            commitment.src_chain_block_number
        };
        if let Some(block_number) = block_number {
            if block_number > option.end_block {
                continue;
            }
            if let Some(start_block) = option.start_block {
                if block_number < start_block {
                    continue;
                }
            }
            if let Some(commitment_hashes) = option.commitment_hash.clone() {
                let commitment_hashes = commitment_hashes.into_iter().collect::<HashSet<_>>();
                if !commitment_hashes.is_empty()
                    && !commitment_hashes.contains(&commitment.commitment_hash_as_biguint())
                {
                    continue;
                }
            }
            if let Some(status) = option.status {
                if commitment.status != status as i32 {
                    continue;
                }
            }
            filtered_commitments.push(commitment.clone());
        }
    }
    filtered_commitments
}

fn filter_nullifiers(nullifiers: &[Nullifier], option: &NullifierQueryOption) -> Vec<Nullifier> {
    let mut filtered_nullifiers = vec![];
    for nullifer in nullifiers.iter() {
        if nullifer.block_number > option.end_block {
            continue;
        }
        if let Some(start_block) = option.start_block {
            if nullifer.block_number < start_block {
                continue;
            }
        }
        if let Some(nullifier_hashes) = option.nullifier.clone() {
            let nullifier_hashes = nullifier_hashes.into_iter().collect::<HashSet<_>>();
            if !nullifier_hashes.is_empty() && !nullifier_hashes.contains(&nullifer.nullifier_as_biguint()) {
                continue;
            }
        }
        filtered_nullifiers.push(nullifer.clone());
    }
    filtered_nullifiers
}
