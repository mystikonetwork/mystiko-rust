use crate::synchronizer::mock::{create_synchronizer, MockSyncDataLoader};
use mystiko_core::{SynchronizerError, SynchronizerHandler};
use mystiko_dataloader::DataLoaderError;

#[tokio::test]
async fn test_chain_synced_block() {
    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(None));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.chain_synced_block(1).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(Some(100)));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.chain_synced_block(1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 100);

    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(Some(100)));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.chain_synced_block(2).await;
    assert!(matches!(
        result.err().unwrap(),
        SynchronizerError::UnsupportedChainError(_)
    ));

    let mut loader = MockSyncDataLoader::new();
    loader
        .expect_chain_loaded_block()
        .returning(|_| Err(DataLoaderError::LoaderNoContractsError));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.chain_synced_block(1).await;
    assert!(matches!(result.err().unwrap(), SynchronizerError::DataLoaderError(_)));
}

#[tokio::test]
async fn test_contract_synced_block() {
    let mut loader = MockSyncDataLoader::new();
    loader.expect_contract_loaded_block().returning(|_, _| Ok(None));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.contract_synced_block(1, "0x").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    let mut loader = MockSyncDataLoader::new();
    loader.expect_contract_loaded_block().returning(|_, _| Ok(Some(100)));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.contract_synced_block(1, "0x").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 100);

    let mut loader = MockSyncDataLoader::new();
    loader.expect_contract_loaded_block().returning(|_, _| Ok(Some(100)));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.contract_synced_block(2, "0x").await;
    assert!(matches!(
        result.err().unwrap(),
        SynchronizerError::UnsupportedChainError(_)
    ));

    let mut loader = MockSyncDataLoader::new();
    loader
        .expect_contract_loaded_block()
        .returning(|_, _| Err(DataLoaderError::LoaderNoContractsError));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.contract_synced_block(1, "0x").await;
    assert!(matches!(result.err().unwrap(), SynchronizerError::DataLoaderError(_)));
}

#[tokio::test]
async fn test_status_without_contract() {
    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(None));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.status(false).await.unwrap();
    assert_eq!(result.chains.len(), 1);
    assert_eq!(result.chains[0].chain_id, 1);
    assert_eq!(result.chains[0].synced_block, 0);
    assert_eq!(result.chains[0].contracts.len(), 0);

    let mut loader1 = MockSyncDataLoader::new();
    loader1.expect_chain_loaded_block().returning(|_| Ok(Some(100)));
    let mut loader2 = MockSyncDataLoader::new();
    loader2
        .expect_chain_loaded_block()
        .returning(|_| Err(DataLoaderError::LoaderNoContractsError));
    let synchronizer = create_synchronizer(1, vec![loader1, loader2]).await;
    let result = synchronizer.status(false).await;
    assert!(matches!(result.err().unwrap(), SynchronizerError::DataLoaderError(_)));

    let mut loader1 = MockSyncDataLoader::new();
    loader1.expect_chain_loaded_block().returning(|_| Ok(Some(100)));
    let mut loader2 = MockSyncDataLoader::new();
    loader2.expect_chain_loaded_block().returning(|_| Ok(None));
    let synchronizer = create_synchronizer(1, vec![loader1, loader2]).await;
    let result = synchronizer.status(false).await.unwrap();
    assert_eq!(result.chains.len(), 2);
    for chain in result.chains.iter() {
        if chain.chain_id == 1 {
            assert_eq!(chain.synced_block, 100);
            assert_eq!(chain.contracts.len(), 0);
        } else if chain.chain_id == 2 {
            assert_eq!(chain.synced_block, 0);
            assert_eq!(chain.contracts.len(), 0);
        } else {
            unreachable!();
        }
    }
}

#[tokio::test]
async fn test_status_with_contract() {
    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(None));
    let synchronizer = create_synchronizer(1, vec![loader]).await;
    let result = synchronizer.status(true).await;
    assert!(matches!(
        result.err().unwrap(),
        SynchronizerError::UnsupportedChainError(_)
    ));

    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(None));
    loader.expect_contract_loaded_block().returning(|_, _| Ok(None));
    let synchronizer = create_synchronizer(5, vec![loader]).await;
    let result = synchronizer.status(true).await.unwrap();
    assert_eq!(result.chains.len(), 1);
    assert_eq!(result.chains[0].chain_id, 5);
    assert_eq!(result.chains[0].synced_block, 0);
    assert_eq!(result.chains[0].contracts.len(), 2);
    assert_eq!(result.chains[0].contracts[0].synced_block, 0);
    assert_eq!(result.chains[0].contracts[1].synced_block, 0);

    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(Some(200)));
    loader.expect_contract_loaded_block().returning(|_, _| Ok(Some(300)));
    let synchronizer = create_synchronizer(5, vec![loader]).await;
    let result = synchronizer.status(true).await.unwrap();
    assert_eq!(result.chains.len(), 1);
    assert_eq!(result.chains[0].chain_id, 5);
    assert_eq!(result.chains[0].synced_block, 200);
    assert_eq!(result.chains[0].contracts.len(), 2);
    assert_eq!(result.chains[0].contracts[0].synced_block, 300);
    assert_eq!(result.chains[0].contracts[1].synced_block, 300);

    let mut loader = MockSyncDataLoader::new();
    loader.expect_chain_loaded_block().returning(|_| Ok(Some(200)));
    loader
        .expect_contract_loaded_block()
        .returning(|_, _| Err(DataLoaderError::LoaderNoContractsError));
    let synchronizer = create_synchronizer(5, vec![loader]).await;
    let result = synchronizer.status(true).await;
    assert!(matches!(result.err().unwrap(), SynchronizerError::DataLoaderError(_)));
}
