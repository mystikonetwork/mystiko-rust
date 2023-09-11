use crate::loader::create_loader;
use mystiko_dataloader::loader::DataLoader;
use mystiko_dataloader::DataLoaderError;

#[tokio::test]
async fn test_loader_load_error() {
    let chain_id = 123456601234566_u64;
    let (_cfg, loader, _fetchers, _, _, _mock_provider) = create_loader(chain_id, 1, 1, false).await;
    let result = loader.load(None).await;
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::UnsupportedChainError(_)
    ));

    let chain_id = 1_u64;
    let (_cfg, loader, _fetchers, _, _, _mock_provider) = create_loader(chain_id, 1, 1, false).await;
    let result = loader.load(None).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains("failed to get latest block number"));
}
