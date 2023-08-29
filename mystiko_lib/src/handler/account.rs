use crate::core::{MYSTIKO, MYSTIKO_RUNTIME};
use anyhow::Result;
use mystiko_protos::core::handler::v1::CreateAccountOptions;
use prost::Message;

pub fn account_create<'a>(options: &[u8]) -> Result<Vec<u8>> {
    MYSTIKO_RUNTIME.block_on(create(options))
}

async fn create<'a>(options: &[u8]) -> Result<Vec<u8>> {
    let options = CreateAccountOptions::decode(options)?;
    let mystiko_guard = MYSTIKO.read().await;
    let account = mystiko_guard.get()?.accounts.create(&options).await?;
    Ok(account.encode_to_vec())
}
