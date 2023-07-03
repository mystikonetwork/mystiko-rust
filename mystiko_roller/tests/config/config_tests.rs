extern crate mystiko_roller;

use crate::common::ENV_MUTEX;
use mystiko_roller::config::roller::{create_mystiko_config, create_roller_config, CoreConfig, RollupConfig};
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

#[tokio::test]
#[should_panic(expected = "unsupported rollup size")]
async fn test_get_rollup_cost() {
    let rollup_cfg = RollupConfig {
        merkle_tree_height: 32,
        force_rollup_block_count: 100,
        rollup1_gas_cost: 1,
        rollup2_gas_cost: 2,
        rollup4_gas_cost: 3,
        rollup8_gas_cost: 4,
        rollup16_gas_cost: 5,
    };
    assert_eq!(rollup_cfg.get_rollup_cost(1), 1);
    assert_eq!(rollup_cfg.get_rollup_cost(2), 2);
    assert_eq!(rollup_cfg.get_rollup_cost(4), 3);
    assert_eq!(rollup_cfg.get_rollup_cost(8), 4);
    assert_eq!(rollup_cfg.get_rollup_cost(16), 5);
    assert_eq!(rollup_cfg.get_rollup_cost(32), 6);
}

#[tokio::test]
#[should_panic(expected = "failed load core config from file")]
async fn test_create_mystiko_config1() {
    let _guard = ENV_MUTEX.write().await;
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "tests/test_files/config/4");

    let core_cfg = CoreConfig {
        is_staging: false,
        remote_base_url: None,
        git_revision: None,
    };
    let _ = create_mystiko_config(&core_cfg).await;
}

#[tokio::test]
#[should_panic(expected = "failed load core config from remote")]
async fn test_create_mystiko_config2() {
    let _guard = ENV_MUTEX.write().await;
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "tests/test_files/config/1");

    let core_cfg = CoreConfig {
        is_staging: false,
        remote_base_url: Some("https://new.static.mystiko.network/config".to_string()),
        git_revision: Some("1234567".to_string()),
    };
    let _ = create_mystiko_config(&core_cfg).await;
}
