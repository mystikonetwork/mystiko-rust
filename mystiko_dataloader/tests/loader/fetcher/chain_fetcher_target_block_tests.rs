use crate::loader::create_loader_with_priority;
use mystiko_dataloader::loader::DataLoader;

#[tokio::test]
async fn test_loader_one_fetcher_loaded_block_error() {
    let chain_id = 1_u64;
    let (_, loader, fetchers, _, handler) = create_loader_with_priority(chain_id, 1, 1, false, vec![1]).await;

    // fetch return error
    fetchers[0].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await;
    assert!(handler.drain_data().await.is_empty());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_loader_many_fetcher_same_target_block_priority() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, _) = create_loader_with_priority(chain_id, 4, 1, false, vec![0; 4]).await;
    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(Some(start_block + 200)).await;
    fetchers[3].set_loaded_block(Some(start_block + 300)).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 300);

    fetchers[0].set_loaded_block(None).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(Some(start_block + 200)).await;
    fetchers[3].set_loaded_block(Some(start_block + 300)).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 300);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(Some(start_block + 200)).await;
    fetchers[3].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 200);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(None).await;
    fetchers[3].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 100);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(None).await;
    fetchers[2].set_loaded_block(None).await;
    fetchers[3].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block);
}

#[tokio::test]
async fn test_loader_many_fetcher_diff_target_block_priority() {
    let chain_id = 1_u64;
    let (cfg, loader, fetchers, _, _) = create_loader_with_priority(chain_id, 4, 1, false, vec![1, 1, 3, 2]).await;
    let start_block = cfg.find_chain(chain_id).unwrap().start_block() + 1;
    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(Some(start_block + 300)).await;
    fetchers[3].set_loaded_block(Some(start_block + 200)).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 300);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(None).await;
    fetchers[3].set_loaded_block(Some(start_block + 200)).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 200);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(Some(start_block + 100)).await;
    fetchers[2].set_loaded_block(None).await;
    fetchers[3].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block + 100);

    fetchers[0].set_loaded_block(Some(start_block)).await;
    fetchers[1].set_loaded_block(None).await;
    fetchers[2].set_loaded_block(None).await;
    fetchers[3].set_loaded_block(None).await;
    let result = loader.chain_target_block(chain_id).await.unwrap();
    assert_eq!(result.unwrap(), start_block);
}
