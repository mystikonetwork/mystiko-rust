use crate::synchronizer::mock::{create_synchronizer, MockSyncDataLoader};
use mystiko_core::SynchronizerHandler;
use mystiko_protos::core::synchronizer::v1::{ResetChainOptions, ResetOptions};

#[tokio::test]
async fn test_chain_reset() {
    let mut loader = MockSyncDataLoader::new();
    loader.expect_reset().returning(|_| Ok(()));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let reset_options = ResetOptions::default();
    let result = synchronizer.reset(reset_options).await;
    assert!(result.is_ok());

    let chain = ResetChainOptions::builder().chain_id(12345678_u64).build();
    let reset_options = ResetOptions::builder().chains(vec![chain]).build();
    let result = synchronizer.reset(reset_options).await;
    assert!(result.is_ok());
}
