use crate::loader::{create_loader, loader_load};
use std::sync::Arc;

#[tokio::test]
async fn test_loader_start() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 987_u64;

    let loader = Arc::new(create_loader(true, contract_address, end_block).await);
    let result = loader_load(loader, None).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}

#[tokio::test]
async fn test_loader_start_batch_builder() {
    let contract_address = "0x932f3DD5b6C0F5fe1aEc31Cb38B7a57d01496411";
    let end_block = 765_u64;
    let delay_block = 2_u64;
    let loader = Arc::new(create_loader(false, contract_address, end_block).await);
    let result = loader_load(loader.clone(), Some(delay_block)).await;
    assert!(result.err().unwrap().to_string().contains("latest block too small"));
}
