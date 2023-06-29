extern crate lazy_static;

use crate::common::ENV_MUTEX;
use mystiko_roller::common::env::{
    load_chain_explorer_api_key, load_coin_market_api_key, load_roller_circuits_path, load_roller_config_path,
    load_roller_db_path, load_roller_home_path, load_roller_private_key, load_roller_run_mod,
};
use std::env;

#[tokio::test]
async fn test_load_roller_run_mod() {
    let _guard = ENV_MUTEX.write().await;
    env::remove_var("MYSTIKO_ROLLER_RUN_MOD");
    let run_mod = load_roller_run_mod();
    assert_eq!(run_mod, "testnet");
    env::set_var("MYSTIKO_ROLLER_RUN_MOD", "mainnet");
    let run_mod = load_roller_run_mod();
    assert_eq!(run_mod, "mainnet");
    env::set_var("MYSTIKO_ROLLER_RUN_MOD", "testnet");
}

#[tokio::test]
async fn test_load_roller_home_path() {
    let _guard = ENV_MUTEX.write().await;
    let home_path = load_roller_home_path();
    assert_eq!(home_path, "/home/mystiko-miner/roller");

    env::set_var("MYSTIKO_ROLLER_HOME_PATH", "/home");
    let home_path = load_roller_home_path();
    assert_eq!(home_path, "/home");

    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
    let config_path = load_roller_config_path();
    assert_eq!(config_path, "/home/config");

    env::remove_var("MYSTIKO_ROLLER_DATA_PATH");
    let db_path = load_roller_db_path();
    assert_eq!(db_path, "/home/data");

    env::remove_var("MYSTIKO_ROLLER_CIRCUITS_PATH");
    let circuits_path = load_roller_circuits_path();
    assert_eq!(circuits_path, "/home/circuits");

    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/2");
    let config_path = load_roller_config_path();
    assert_eq!(config_path, "./tests/test_files/config/2");

    env::set_var("MYSTIKO_ROLLER_DATA_PATH", "../mystiko_circuits");
    let db_path = load_roller_db_path();
    assert_eq!(db_path, "../mystiko_circuits");

    env::set_var("MYSTIKO_ROLLER_CIRCUITS_PATH", "./tests/db");
    let circuits_path = load_roller_circuits_path();
    assert_eq!(circuits_path, "./tests/db");

    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
    env::remove_var("MYSTIKO_ROLLER_DATA_PATH");
    env::remove_var("MYSTIKO_ROLLER_CIRCUITS_PATH");
}

#[tokio::test]
async fn test_load_roller_private_key() {
    let _guard = ENV_MUTEX.write().await;
    env::remove_var("MYSTIKO_ROLLER_PRIVATE_KEY");

    let private_key = load_roller_private_key();
    assert!(private_key.is_err());
    env::set_var(
        "MYSTIKO_ROLLER_PRIVATE_KEY",
        "0x2f0ddd32231ec7dadcef459447c73fae18b9b3e3d0e0acf00e999ca5ffb8efec",
    );
    let private_key = load_roller_private_key().unwrap();
    assert_eq!(
        private_key,
        "0x2f0ddd32231ec7dadcef459447c73fae18b9b3e3d0e0acf00e999ca5ffb8efec"
    );
}

#[tokio::test]
async fn test_load_chain_explorer_api_key() {
    let _guard = ENV_MUTEX.write().await;
    env::remove_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY");
    let x_scan_api_key = load_chain_explorer_api_key();
    assert!(x_scan_api_key.is_err());
    env::set_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY", "chain_explorer_api_key");
    let x_scan_api_key = load_chain_explorer_api_key().unwrap();
    assert_eq!(x_scan_api_key, "chain_explorer_api_key");
}

#[tokio::test]
async fn test_load_coin_market_api_key() {
    let _guard = ENV_MUTEX.write().await;
    env::remove_var("MYSTIKO_ROLLER_COIN_MARKET_CAP_API_KEY");
    let coin_market_api_key = load_coin_market_api_key();
    assert!(coin_market_api_key.is_err());
    env::set_var("MYSTIKO_ROLLER_COIN_MARKET_CAP_API_KEY", "coin_market_api_key");
    let coin_market_api_key = load_coin_market_api_key().unwrap();
    assert_eq!(coin_market_api_key, "coin_market_api_key");
}
