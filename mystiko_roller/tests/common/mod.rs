use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use tokio::sync::RwLock;

pub mod env_tests;
pub mod error_tests;

lazy_static! {
    pub static ref ENV_MUTEX: RwLock<()> = RwLock::new(());
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

pub fn env_init() {
    // env::set_var("MYSTIKO_ROLLER.LOG_LEVEL", "OFF");

    env::set_var("MYSTIKO_ROLLER_CIRCUITS_PATH", "./tests/test_files/circuits");
    env::set_var("MYSTIKO_ROLLER_DATA_PATH", "./tests/test_files/db");

    env::set_var("MYSTIKO_ROLLER_COIN_MARKET_CAP_API_KEY", "coin_market_api_key");
    env::set_var("MYSTIKO_ROLLER_CHAIN_EXPLORER_API_KEY", "chain_explorer_api_key");
    env::set_var(
        "MYSTIKO_ROLLER_PRIVATE_KEY",
        "0x2f0ddd32231ec7dadcef459447c73fae18b9b3e3d0e0acf00e999ca5ffb8efec",
    );
}
