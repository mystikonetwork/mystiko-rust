extern crate mystiko_roller;

use crate::common::ENV_MUTEX;
use mystiko_roller::config::roller::create_roller_config;
use std::env;

#[tokio::test]
async fn test_read_config() {
    let _guard = ENV_MUTEX.write().await;
    env::remove_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID");

    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "tests/test_files/config/1");
    let cfg = create_roller_config();
    assert_eq!(cfg.log_level, "INFO");
    assert_eq!(cfg.chain.chain_id, 5);
    assert_eq!(
        cfg.core.remote_base_url,
        Some("https://new.static.mystiko.network/config".to_string())
    );
    assert_eq!(cfg.core.git_revision, Some("0187435".to_string()));
    assert_eq!(cfg.rollup.force_rollup_block_count, 20);

    env::set_var("MYSTIKO_ROLLER_RUN_MOD", "mainnet");
    let cfg = create_roller_config();
    assert_eq!(cfg.chain.chain_id, 1);
    assert_eq!(cfg.rollup.force_rollup_block_count, 100);

    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "tests/test_files/config/2");
    let cfg = create_roller_config();
    assert_eq!(cfg.chain.chain_id, 1);
    assert_eq!(cfg.core.remote_base_url, None);
    assert_eq!(cfg.core.git_revision, None);
    assert_eq!(cfg.rollup.force_rollup_block_count, 100);

    env::set_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID", "123456789");
    env::set_var("MYSTIKO_ROLLER.ROLLUP.FORCE_ROLLUP_BLOCK_COUNT", "200");
    let cfg = create_roller_config();
    assert_eq!(cfg.chain.chain_id, 123456789);
    assert_eq!(cfg.rollup.force_rollup_block_count, 200);

    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
    env::remove_var("MYSTIKO_ROLLER_RUN_MOD");
    env::remove_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID");
    env::remove_var("MYSTIKO_ROLLER.ROLLUP.FORCE_ROLLUP_BLOCK_COUNT");
}
