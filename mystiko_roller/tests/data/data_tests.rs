use crate::context::mock_context::{create_mock_context, get_pool_contracts, indexer_server_port, MockContext};
use crate::test_files::load::load_commitments;
use ethers_core::types::U256;
use mystiko_roller::common::error::RollerError;
use mystiko_roller::config::roller::ChainDataSource;
use mystiko_roller::context::ContextTrait;
use mystiko_roller::data::handler::{DataHandler, RollupPlan};
use std::sync::Arc;

#[tokio::test]
pub async fn test_insert_commitments() {
    let test_chain_id = 100;
    let (mut data, c) = create_data_handle(test_chain_id).await;
    let pool_contract = get_pool_contracts(&c);

    assert_eq!(data.get_next_sync_block(), 0);
    data.update_next_sync_block(2);
    assert_eq!(data.get_next_sync_block(), 2);
    assert_eq!(data.get_included_count(), 0);
    assert_eq!(data.get_commitments_queue_count(), 0);

    data.init().await.unwrap();
    assert_eq!(data.get_next_sync_block(), pool_contract.start_block());
    assert_eq!(data.get_included_count(), 0);
    assert_eq!(data.get_commitments_queue_count(), 0);

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(pool_contract.address()),
    )
    .await;

    data.insert_commitments(cms.as_slice()).await.unwrap();
    let db_cms = c
        .db()
        .await
        .find_all_commitment(test_chain_id, pool_contract.address())
        .await;
    assert_eq!(cms.len(), db_cms.len());
    assert_eq!(data.get_included_count(), 0);
    assert_eq!(data.get_commitments_queue_count(), cms.len());

    data.insert_commitments(cms.as_slice()).await.unwrap();
    let db_cms = c
        .db()
        .await
        .find_all_commitment(test_chain_id, pool_contract.address())
        .await;
    assert_eq!(cms.len(), db_cms.len());
    assert_eq!(data.get_included_count(), 0);
    assert_eq!(data.get_commitments_queue_count(), cms.len());

    let mut cm1 = cms[0].clone();
    cm1.leaf_index = (cms.len() + 1000) as u64;
    let cms1 = vec![cm1];
    let result = data.insert_commitments(cms1.as_slice()).await;
    assert!(matches!(result.err().unwrap(), RollerError::CommitmentMissing));

    let context_trait2: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let mut data2 = DataHandler::new(test_chain_id, &pool_contract, context_trait2).await;
    data2.init().await.unwrap();
    let db_cms = c
        .db()
        .await
        .find_all_commitment(test_chain_id, pool_contract.address())
        .await;
    assert_eq!(cms.len(), db_cms.len());
    assert_eq!(data2.get_next_sync_block(), cms[cms.len() - 1].block_number);
    assert_eq!(data2.get_included_count(), 0);
    assert_eq!(data2.get_commitments_queue_count(), cms.len());
}

#[tokio::test]
pub async fn test_generate_plan() {
    let test_chain_id = 101;
    let (mut data, c) = create_data_handle(test_chain_id).await;
    let pool_contract = get_pool_contracts(&c);

    let plan = data.generate_plan(0, 100).unwrap();
    assert_eq!(
        plan,
        RollupPlan {
            sizes: vec![],
            total_fee: U256::zero(),
            force: false,
        }
    );

    let cms = load_commitments(
        "tests/test_files/data/commitments.json",
        Some(test_chain_id),
        Some(pool_contract.address()),
    )
    .await;

    let (cms1, cms2) = cms.split_at(1);

    data.insert_commitments(cms1).await.unwrap();
    let plan = data.generate_plan(0, 100).unwrap();
    assert_eq!(
        plan,
        RollupPlan {
            sizes: vec![1],
            total_fee: U256::from_str_radix(&cms1[0].rollup_fee, 10).unwrap(),
            force: false,
        }
    );
    assert_eq!(data.get_included_count(), 0);
    assert_eq!(data.get_commitments_queue_count(), 1);

    let (cms3, _) = cms2.split_at(3);
    data.insert_commitments(cms3).await.unwrap();
    let plan = data.generate_plan(1, 100).unwrap();
    let fee = cms3
        .iter()
        .map(|cm| U256::from_str_radix(&cm.rollup_fee, 10).unwrap())
        .fold(U256::zero(), |acc, x| acc + x);
    assert_eq!(
        plan,
        RollupPlan {
            sizes: vec![1, 2],
            total_fee: fee,
            force: false,
        }
    );
    assert_eq!(data.get_included_count(), 1);
    assert_eq!(data.get_commitments_queue_count(), 4);

    let (_, cms32) = cms3.split_at(1);
    let plan = data.generate_plan(2, 100).unwrap();
    let fee = cms32
        .iter()
        .map(|cm| U256::from_str_radix(&cm.rollup_fee, 10).unwrap())
        .fold(U256::zero(), |acc, x| acc + x);
    assert_eq!(
        plan,
        RollupPlan {
            sizes: vec![2],
            total_fee: fee,
            force: false,
        }
    );
    assert_eq!(data.get_included_count(), 2);
    assert_eq!(data.get_commitments_queue_count(), 4);
}

#[tokio::test]
pub async fn test_giver_check_counter() {
    let test_chain_id = 102;
    let (mut data, _) = create_data_handle(test_chain_id).await;
    assert_eq!(data.get_giver_check_counter(&ChainDataSource::Indexer).unwrap(), 0);
    data.inc_giver_check_counter(&ChainDataSource::Indexer);
    assert_eq!(data.get_giver_check_counter(&ChainDataSource::Indexer).unwrap(), 1);
    data.reset_giver_check_counter(&ChainDataSource::Indexer, 100);
    assert_eq!(data.get_giver_check_counter(&ChainDataSource::Indexer).unwrap(), 100);
}

async fn create_data_handle(test_chain_id: u64) -> (DataHandler, Arc<MockContext>) {
    let c = create_mock_context(indexer_server_port(test_chain_id)).await;
    let c = Arc::new(c);
    let pool_contract = get_pool_contracts(&c);

    let context_trait: Arc<dyn ContextTrait + Send> = Arc::clone(&c) as Arc<dyn ContextTrait + Send>;
    let data = DataHandler::new(test_chain_id, &pool_contract, context_trait).await;
    (data, c)
}
