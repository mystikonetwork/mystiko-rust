extern crate core;

use crate::common::{evn_init, ENV_MUTEX};
use mystiko_roller::roller::{run_roller, Roller};
use std::env;

pub mod common;
mod config;
pub mod context;
mod data;
mod pull;
mod rollup;
pub mod test_files;

#[tokio::test]
pub async fn test_run_roller_with_error() {
    let _guard = ENV_MUTEX.write().await;
    evn_init();
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/5");
    env::set_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID", "56");
    run_roller().await;

    env::set_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID", "156111222");
    run_roller().await;
    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
    env::remove_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID");
}

#[tokio::test]
pub async fn test_run_roller() {
    let _guard = ENV_MUTEX.write().await;
    evn_init();
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "./tests/test_files/config/5");
    env::set_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID", "1");
    let mut r = Roller::new().await.unwrap();
    r.stop = true;
    r.start().await;
    assert_eq!(r.round, 1);
    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
    env::remove_var("MYSTIKO_ROLLER.CHAIN.CHAIN_ID");
}
