use crate::runtime;
use anyhow::Result;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use prost::Message;

pub fn create(options: &[u8]) -> Result<Vec<u8>> {
    let options = CreateWalletOptions::decode(options)?;
    runtime().block_on(internal::create(options))
}

mod internal {
    use super::*;
    use crate::instance;

    pub(crate) async fn create(options: CreateWalletOptions) -> Result<Vec<u8>> {
        let mystiko_guard = instance().read().await;
        let wallet = mystiko_guard.get()?.wallets.create(&options).await?;
        Ok(wallet.encode_to_vec())
    }
}
