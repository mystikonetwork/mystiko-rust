use ethers_providers::Quorum;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_core::helper::provider::ProvidersConfig;
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::ChainProvidersOptions;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_providers_config() {
    let config = Arc::new(
        MystikoConfig::from_json_file("tests/files/helper/provider/config.json")
            .await
            .unwrap(),
    );
    let providers_config = ProvidersConfig::new(config);
    let providers_options1 = providers_config.providers_options(5).unwrap();
    match providers_options1 {
        ProvidersOptions::Failover(options) => {
            assert_eq!(options.len(), 2);
            assert_eq!(options[0].url, "http://localhost:8545");
            assert_eq!(options[1].url, "http://localhost:8546");
        }
        _ => panic!("unexpected provider options type"),
    }
    let providers_options2 = providers_config.providers_options(11155111).unwrap();
    match providers_options2 {
        ProvidersOptions::Quorum(options, quorum_options) => {
            assert_eq!(options.len(), 3);
            assert_eq!(options[0].url, "http://localhost:8545");
            assert_eq!(options[0].timeout_retries, Some(3));
            assert_eq!(options[0].rate_limit_retries, Some(3));
            assert_eq!(options[0].quorum_weight, Some(2));
            assert_eq!(options[0].request_timeout, Some(Duration::from_millis(20000)));
            assert_eq!(options[1].url, "http://localhost:8546");
            match quorum_options.quorum.unwrap() {
                Quorum::Percentage(percentage) => assert_eq!(percentage, 75),
                _ => panic!("unexpected quorum type"),
            }
        }
        _ => panic!("unexpected provider options type"),
    }
    assert!(providers_config.providers_options(97).is_none());
}
