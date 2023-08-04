use crate::loader::loader_mock::create_shared_loader;
use mystiko_dataloader::error::DataLoaderError;

#[tokio::test]
async fn test_loader_load_error() {
    let chain_id = 123456601234566_u64;
    let (_cfg, loader, _fetchers, _, _, listeners, _mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let result = loader.load(None).await;
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::UnsupportedChainError(_)
    ));
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].drain_events().await.is_empty());

    let chain_id = 1_u64;
    let (_cfg, loader, _fetchers, _, _, listeners, _mock_provider) = create_shared_loader(chain_id, 1, 1, 1).await;
    let result = loader.load(None).await;
    assert!(matches!(result.err().unwrap(), DataLoaderError::ProviderError(_)));
    assert!(!loader.is_loading().await);
    assert!(!loader.is_running().await);
    assert!(listeners[0].drain_events().await.is_empty());
}
