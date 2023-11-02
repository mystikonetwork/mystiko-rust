use crate::synchronizer::loader::create_sync_loader_handler;
use crate::synchronizer::loader::mock::MockSyncDataHandler;
use mystiko_dataloader::data::FullData;
use mystiko_dataloader::handler::{CommitmentQueryOption, DataHandler, NullifierQueryOption, QueryResult};

#[tokio::test]
async fn test_query_from_handler() {
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_loading_contracts().returning(|_| Ok(None));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_loading_contracts(5).await;
    assert!(result.is_ok());

    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_chain_loaded_block().returning(|_| Ok(None));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_chain_loaded_block(5).await;
    assert!(result.is_ok());

    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_contract_loaded_block().returning(|_, _| Ok(None));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_contract_loaded_block(5, "0x").await;
    assert!(result.is_ok());

    let option = CommitmentQueryOption::builder()
        .chain_id(5_u64)
        .contract_address("0x".to_string())
        .end_block(100_u64)
        .build();
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_commitment().returning(|_| Ok(None));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_commitment(&option).await;
    assert!(result.is_ok());

    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_commitments()
        .returning(|_| Ok(QueryResult::builder().end_block(1_u64).result(vec![]).build()));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_commitments(&option).await;
    assert!(result.is_ok());

    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_count_commitments()
        .returning(|_| Ok(QueryResult::builder().end_block(1_u64).result(0_u64).build()));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.count_commitments(&option).await;
    assert!(result.is_ok());

    let option = NullifierQueryOption::builder()
        .chain_id(5_u64)
        .contract_address("0x".to_string())
        .end_block(100_u64)
        .build();
    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_query_nullifier().returning(|_| Ok(None));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.query_nullifier(&option).await;
    assert!(result.is_ok());

    let mut mock = MockSyncDataHandler::<FullData>::default();
    mock.expect_count_nullifiers()
        .returning(|_| Ok(QueryResult::builder().end_block(1_u64).result(0_u64).build()));
    let (handler, _, _) = create_sync_loader_handler(mock).await;
    let result = handler.count_nullifiers(&option).await;
    assert!(result.is_ok());
}
