use crate::common::env_init;
use mystiko_roller::chain::explorer::ExplorerStub;
use mystiko_roller::chain::ChainDataGiver;
use mystiko_roller::config::roller::ChainDataSource;

#[tokio::test]
pub async fn test_explorer_stub() {
    env_init();

    let stub = ExplorerStub::new("http://localhost");
    let data_source = stub.data_source();
    assert_eq!(data_source, ChainDataSource::Explorer);
    let result = stub.get_latest_block_number(1, "0x00").await;
    assert!(result.is_err());
    let result = stub.get_included_count(1, "0x00").await;
    assert!(result.is_err());
    let result = stub.get_queued_commitments(1, "0x00", 1, 2).await;
    assert!(result.is_err());
}
