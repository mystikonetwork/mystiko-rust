extern crate lazy_static;

use dotenv::dotenv;
use lazy_static::lazy_static;
use mystiko_roller::common::env::{
    load_coin_market_api_key, load_roller_circuits_path, load_roller_config_path, load_roller_db_path,
    load_roller_home_path, load_roller_private_key, load_x_scan_api_key,
};
use std::env;
use std::sync::Mutex;

lazy_static! {
    pub static ref ENV_CONFIG_PATH_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref ENV_MOCK_INDEXER_PORT_MUTEX: Mutex<()> = Mutex::new(());
}

pub fn set_env_mock_indexer_port(port: &str) {
    env::set_var("MYSTIKO_MOCK_INDEXER_PORT", port);
}

pub fn load_env_mock_indexer_port() -> String {
    dotenv().ok();
    match env::var("MYSTIKO_MOCK_INDEXER_PORT") {
        Ok(value) => value,
        Err(_) => panic!("MYSTIKO_MOCK_INDEXER_PORT not set"),
    }
}

// #[tokio::test]
// async fn test_load_roller_run_mod() {
//     let run_mod = load_roller_run_mod();
//     assert_eq!(run_mod, "testnet");
//     env::set_var("MYSTIKO_ROLLER_RUN_MOD", "mainnet");
//     let run_mod = load_roller_run_mod();
//     assert_eq!(run_mod, "mainnet");
// }

#[tokio::test]
async fn test_load_roller_home_path() {
    let home_path = load_roller_home_path();
    assert_eq!(home_path, "/home/mystiko-miner/roller");

    env::set_var("MYSTIKO_ROLLER_HOME_PATH", "/home");
    let home_path = load_roller_home_path();
    assert_eq!(home_path, "/home");

    let config_path = load_roller_config_path();
    assert_eq!(config_path, "/home/config");

    let db_path = load_roller_db_path();
    assert_eq!(db_path, "/home/data");

    let circuits_path = load_roller_circuits_path();
    assert_eq!(circuits_path, "/home/circuits");

    // env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/2");
    // let config_path = load_roller_config_path();
    // assert_eq!(config_path, "./tests/test_files/config/2");

    // env::set_var("MYSTIKO_ROLLER_DATA_PATH", "../mystiko_circuits");
    // let db_path = load_roller_db_path();
    // assert_eq!(db_path, "../mystiko_circuits");

    // env::set_var("MYSTIKO_ROLLER_CIRCUITS_PATH", "./tests/db");
    // let circuits_path = load_roller_circuits_path();
    // assert_eq!(circuits_path, "./tests/db");
}

#[tokio::test]
async fn test_load_roller_private_key() {
    let private_key = load_roller_private_key();
    assert!(private_key.is_err());
    env::set_var(
        "MYSTIKO_ROLLER_PRIVATE_KEY",
        "0x3c87ede2a7d0a38de10b522ec6a7a3ea73be79e469c2aae38b5e0030131a9afa",
    );
    let private_key = load_roller_private_key().unwrap();
    assert_eq!(
        private_key,
        "0x3c87ede2a7d0a38de10b522ec6a7a3ea73be79e469c2aae38b5e0030131a9afa"
    );
}

#[tokio::test]
async fn test_load_x_scan_api_key() {
    let x_scan_api_key = load_x_scan_api_key();
    assert!(x_scan_api_key.is_err());
    env::set_var("MYSTIKO_ROLLER_X_SCAN_API_KEY", "x_scan_api_key");
    let x_scan_api_key = load_x_scan_api_key().unwrap();
    assert_eq!(x_scan_api_key, "x_scan_api_key");
}

#[tokio::test]
async fn test_load_coin_market_api_key() {
    let coin_market_api_key = load_coin_market_api_key();
    assert!(coin_market_api_key.is_err());
    env::set_var("MYSTIKO_ROLLER_COIN_MARKET_CAP_API_KEY", "coin_market_api_key");
    let coin_market_api_key = load_coin_market_api_key().unwrap();
    assert_eq!(coin_market_api_key, "coin_market_api_key");
}
