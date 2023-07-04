use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use mystiko_roller::config::mystiko_parser::MystikoConfigParser;
use mystiko_roller::config::roller::create_roller_config;

#[tokio::test]
async fn test_providers_options() {
    let cfg_path = "tests/test_files/config/base";
    let roller_cfg = create_roller_config("mainnet", cfg_path);
    let core_cfg_parser = MystikoConfigParser::new(&roller_cfg.core, cfg_path).await;
    let op = core_cfg_parser.providers_options(5).await;
    assert!(op.unwrap().is_none());
    let op = core_cfg_parser.providers_options(1).await;
    assert!(matches!(op.unwrap().unwrap(), ProvidersOptions::Failover(_)));
    let op = core_cfg_parser.providers_options(56).await;
    assert!(matches!(op.unwrap().unwrap(), ProvidersOptions::Quorum(_, _)));
}
