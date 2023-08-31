use crate::runtime;
use anyhow::Result;

pub fn version() -> Result<String> {
    runtime().block_on(internal::version())
}

mod internal {
    use super::*;
    use crate::instance;

    pub(crate) async fn version() -> Result<String> {
        let mystiko_guard = instance().read().await;
        let version: &str = mystiko_guard.get()?.config.version();
        Ok(version.to_string())
    }
}
