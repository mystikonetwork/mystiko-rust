use crate::loader::create_loader;
use mystiko_dataloader::loader::{DataLoader, LoadOption};
use mystiko_dataloader::DataLoaderError;
#[tokio::test]
async fn test_loader_start() {
    let (_, loader, fetchers, _, _) = create_loader(1, 1, 1, false).await;
    fetchers[0].set_loaded_block(Some(100)).await;
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(result
        .err()
        .unwrap()
        .to_string()
        .contains(DataLoaderError::LoaderFetchersExhaustedError.to_string().as_str()));
}

#[tokio::test]
async fn test_loader_load_error() {
    let chain_id = 123456601234566_u64;
    let (_cfg, loader, _fetchers, _, _) = create_loader(chain_id, 1, 1, false).await;
    let result = loader.load(Some(LoadOption::default())).await;
    assert!(matches!(
        result.err().unwrap(),
        DataLoaderError::UnsupportedChainError(_)
    ));
}
