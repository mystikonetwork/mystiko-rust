extern crate mockito;

use crate::common::{env_init, ENV_MUTEX};
use mystiko_roller::context::{Context, ContextTrait};

#[tokio::test]
pub async fn test_context_new() {
    let _guard = ENV_MUTEX.write().await;
    env_init();

    let c = Context::new("testnet", "./tests/test_files/config/base").await;
    assert!(c.is_ok());

    let c = Context::new("testnet", "./tests/test_files/config/no_indexer_explorer").await;
    assert!(c.is_ok());
    let c = c.unwrap();
    assert!(c.indexer().is_none());
    assert!(c.chain_explorer().is_none());

    let _ = c.token_price().await;
}
