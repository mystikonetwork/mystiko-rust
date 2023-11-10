use mystiko_config::{MystikoConfig, RawMystikoConfig};
use mystiko_core::SignerProviderOptions;
use mystiko_ethers::{ChainProvidersOptions, ProvidersOptions};
use std::sync::Arc;

#[tokio::test]
async fn test_chain_provider_options() {
    let mut config = mystiko_config::create_raw_from_file::<RawMystikoConfig>("tests/files/mystiko/config.json")
        .await
        .unwrap();
    let mut chain_1 = config.chains[0].as_ref().clone();
    let mut chain_2 = config.chains[1].as_ref().clone();
    let chain_id_1 = chain_1.chain_id;
    let chain_id_2 = chain_2.chain_id;
    chain_1.signer_endpoint = "https://rpc.provider.com".to_string();
    chain_2.signer_endpoint = "wss://rpc.provider.com".to_string();
    config.chains = vec![Arc::new(chain_1), Arc::new(chain_2)];
    let config = Arc::new(MystikoConfig::from_raw(config).unwrap());
    let signer_provider_options: SignerProviderOptions = config.into();
    let options = signer_provider_options
        .providers_options(chain_id_1)
        .await
        .unwrap()
        .unwrap();
    match options {
        ProvidersOptions::Http(options) => assert_eq!(options.url, "https://rpc.provider.com"),
        _ => panic!("Expected Http options"),
    }
    let options = signer_provider_options
        .providers_options(chain_id_2)
        .await
        .unwrap()
        .unwrap();
    match options {
        ProvidersOptions::Ws(options) => assert_eq!(options.url, "wss://rpc.provider.com"),
        _ => panic!("Expected Ws options"),
    }
    assert!(signer_provider_options.providers_options(0).await.unwrap().is_none());
}
