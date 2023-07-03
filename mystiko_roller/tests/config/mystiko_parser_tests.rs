use crate::common::ENV_MUTEX;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_roller::config::mystiko_parser::MystikoConfigParser;
use mystiko_roller::config::roller::create_roller_config;
use std::env;

#[tokio::test]
async fn test_providers_options() {
    let _guard = ENV_MUTEX.write().await;
    env::set_var("MYSTIKO_ROLLER_CONFIG_PATH", "tests/test_files/config/2");

    let roller_cfg = create_roller_config();
    let core_cfg_parser = MystikoConfigParser::new(&roller_cfg.core).await;
    let op = core_cfg_parser.providers_options(5).await;
    assert!(op.unwrap().is_none());
    let op = core_cfg_parser.providers_options(1).await;
    assert!(matches!(op.unwrap().unwrap(), ProvidersOptions::Failover(_)));
    let op = core_cfg_parser.providers_options(56).await;
    assert!(matches!(op.unwrap().unwrap(), ProvidersOptions::Quorum(_, _)));
    env::remove_var("MYSTIKO_ROLLER_CONFIG_PATH");
}
