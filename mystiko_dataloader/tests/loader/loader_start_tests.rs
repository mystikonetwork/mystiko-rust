use crate::loader::{create_loader, loader_load};
use ethers_core::types::U64;

#[tokio::test]
async fn test_loader_start() {
    let (_, loader, _, _, _, mock) = create_loader(1, 1, 1, false).await;
    let block_number = U64::from(100);
    mock.push(block_number).unwrap();
    let result = loader_load(loader, None).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let delay_block = 2_u64;
    let (_, loader, _, _, _, mock) = create_loader(1, 1, 1, false).await;
    let block_number = U64::from(100);
    mock.push(block_number).unwrap();
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}
