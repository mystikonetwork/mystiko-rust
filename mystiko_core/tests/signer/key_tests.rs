use async_trait::async_trait;
use ethers_core::rand::thread_rng;
use ethers_core::types::{Address, TransactionRequest};
use ethers_signers::{LocalWallet, Signer};
use mockall::mock;
use mystiko_core::{PrivateKeySigner, PrivateKeySignerOptions, TransactionSigner};
use mystiko_ethers::{ChainProvidersOptions, ProviderOptions, ProviderPool, ProvidersOptions};
use mystiko_utils::hex::encode_hex;
use std::sync::Arc;

#[tokio::test]
async fn test_send_transaction() {
    let (private_key, address) = generate_private_key();
    let (server, mock_path) = mock_provider_server().await;
    let providers = create_provider_pool(server.url());
    let signer = PrivateKeySigner::new(
        PrivateKeySignerOptions::<ProviderPool<MockChainConfig>>::builder()
            .providers(providers)
            .private_key(private_key)
            .build(),
    )
    .unwrap();
    assert_eq!(signer.address().await.unwrap(), address);
    let tx_hash = signer
        .send_transaction(
            56_u64,
            TransactionRequest::pay("0x70f657164e5b75689b64b7fd1fa275f334f28e18", 100).into(),
        )
        .await
        .unwrap();
    assert_eq!(
        format!("0x{:x}", tx_hash),
        "0x8d874bd339d08093701771307ea134f2534610d1425bee4b3ed8e30854ac68b6"
    );
    mock_path.assert_async().await;
}

mock! {
    #[derive(Debug)]
    ChainConfig {}

    #[async_trait]
    impl ChainProvidersOptions for ChainConfig {
         async fn providers_options(&self, chain_id: u64) -> anyhow::Result<Option<ProvidersOptions>>;
    }
}

fn generate_private_key() -> (String, Address) {
    let wallet = LocalWallet::new(&mut thread_rng());
    (encode_hex(wallet.signer().to_bytes()), wallet.address())
}

fn create_provider_pool(url: String) -> Arc<ProviderPool<MockChainConfig>> {
    let mut mock_chain_config = MockChainConfig::new();
    mock_chain_config
        .expect_providers_options()
        .with(mockall::predicate::eq(56))
        .returning(move |_| {
            Ok(Some(ProvidersOptions::Http(
                ProviderOptions::builder().url(url.clone()).build(),
            )))
        });
    mock_chain_config
        .expect_providers_options()
        .with(mockall::predicate::ne(56))
        .returning(|_| Ok(None));
    let pool = ProviderPool::<MockChainConfig>::builder()
        .chain_providers_options(mock_chain_config)
        .build();
    Arc::new(pool)
}

async fn mock_provider_server() -> (mockito::ServerGuard, mockito::Mock) {
    let mut server = mockito::Server::new_async().await;
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
