use anyhow::Result;
use async_trait::async_trait;
use ethers_core::rand::thread_rng;
use ethers_core::types::{Address, TransactionRequest};
use ethers_signers::{LocalWallet, Signer as EthersSigner};
use mockall::{mock, predicate};
use mockito::{Mock, Server};
use mystiko_ethers::provider::factory::ProvidersOptions;
use mystiko_ethers::provider::pool::{ChainProvidersOptions, ProviderPool};
use mystiko_ethers::provider::types::ProviderOptions;
use mystiko_ethers::signer::common::Signer;
use mystiko_ethers::signer::private_key::PrivateKeySigner;
use std::sync::Arc;

mock! {
    #[derive(Debug)]
    ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> Result<Option<ProvidersOptions>>;
    }
}

#[tokio::test]
async fn test_private_key_signer() {
    let (private_key, address) = generate_private_key();
    let (_, mock_path) = mock_provider_server().await;
    let mut pk_signer = PrivateKeySigner::new(&private_key, create_provider_pool()).unwrap();
    assert_eq!(pk_signer.chain_id().await.unwrap(), 1);
    assert_eq!(pk_signer.accounts().await.unwrap(), vec![address]);
    pk_signer.switch_chain(56).await.unwrap();
    assert_eq!(pk_signer.chain_id().await.unwrap(), 56);
    let tx = TransactionRequest::pay("0x70f657164e5b75689b64b7fd1fa275f334f28e18", 100);
    let tx_hash = pk_signer.send_transaction(tx.clone()).await.unwrap();
    mock_path.assert_async().await;
    assert_eq!(
        format!("{0:?}", tx_hash),
        "0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6"
    );
    pk_signer.switch_chain(1).await.unwrap();
    assert!(pk_signer.send_transaction(tx).await.is_err());
}

fn generate_private_key() -> (String, Address) {
    let wallet = LocalWallet::new(&mut thread_rng());
    (hex::encode(wallet.signer().to_bytes()), wallet.address())
}

fn create_provider_pool() -> Arc<ProviderPool<MockChainConfig>> {
    let mut mock_chain_config = MockChainConfig::new();
    mock_chain_config
        .expect_providers_options()
        .with(predicate::eq(56))
        .returning(move |_| {
            Ok(Some(ProvidersOptions::Http(
                ProviderOptions::builder()
                    .url(String::from("http://localhost:13121"))
                    .build(),
            )))
        });
    mock_chain_config
        .expect_providers_options()
        .with(predicate::ne(56))
        .returning(|_| Ok(None));
    let pool = ProviderPool::<MockChainConfig>::builder()
        .chain_providers_options(mock_chain_config)
        .build();
    Arc::new(pool)
}

async fn mock_provider_server() -> (Server, Mock) {
    let mut server = Server::new_with_port_async(13121).await;
    let path = server
        .mock("POST", "/")
        .expect_at_least(1)
        .with_body(
            "{\
        \"jsonrpc\":\"2.0\",\
        \"id\":0,\
        \"result\":\"0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6\"\
        }",
        )
        .create_async()
        .await;
    (server, path)
}
