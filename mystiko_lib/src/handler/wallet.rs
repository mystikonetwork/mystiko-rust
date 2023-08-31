use crate::core::{instance, runtime};
use anyhow::Result;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use prost::Message;

pub fn wallet_create(options: &[u8]) -> Result<Vec<u8>> {
    let options = CreateWalletOptions::decode(options)?;
    runtime().block_on(create(options))
}

async fn create(options: CreateWalletOptions) -> Result<Vec<u8>> {
    let mystiko_guard = instance().read().await;
    let wallet = mystiko_guard.get()?.wallets.create(&options).await?;
    Ok(wallet.encode_to_vec())
}
