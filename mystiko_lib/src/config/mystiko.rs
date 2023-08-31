use crate::core::{instance, runtime};
use anyhow::Result;

pub fn config_version() -> Result<String> {
    runtime().block_on(version())
}

async fn version() -> Result<String> {
    let mystiko_guard = instance().read().await;
    let version: &str = mystiko_guard.get()?.config.version();
    Ok(version.to_string())
}
