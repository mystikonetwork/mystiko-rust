use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use tokio::sync::RwLock;

pub mod env_tests;

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

pub fn evn_init() {
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/2");
    env::set_var(
        "MYSTIKO_ROLLER_CIRCUITS_PATH",
        "./../mystiko_circuits/dist/zokrates/dev",
    );
    env::set_var("MYSTIKO_ROLLER_DATA_PATH", "./tests/db");

    // env::set_var(
    //     "MYSTIKO_ROLLER_PRIVATE_KEY",
    //     "0x3c87ede2a7d0a38de10b522ec6a7a3ea73be79e469c2aae38b5e0030131a9afa",
    // );

    env::set_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID", "1");
}
