use crate::core::{MYSTIKO, MYSTIKO_RUNTIME};
use anyhow::Result;

pub fn config_version<'a>() -> Result<String> {
    MYSTIKO_RUNTIME.block_on(version())
}

async fn version<'a>() -> Result<String> {
    let mystiko_guard = MYSTIKO.read().await;
    let version: &str = mystiko_guard.get()?.config.version();
    Ok(version.to_string())
}
