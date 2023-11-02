use async_trait::async_trait;
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, NameOrAddress, TransactionRequest, TxHash, U256};
use mockall::mock;
use mystiko_core::{
    BalanceOptions, Erc20ApproveOptions, Erc20BalanceOptions, Erc20TransferOptions, PublicAssetHandler, PublicAssets,
    PublicAssetsError, TransferOptions,
};
use mystiko_ethers::{JsonRpcParams, Provider, ProviderWrapper};
use mystiko_utils::address::ethers_address_from_string;
use std::sync::Arc;

#[tokio::test]
async fn test_balance_of() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, _| method == "eth_getBalance")
        .returning(|_, _| Ok(serde_json::json!("0xdeadbeef")));
    let assets = setup(1, provider, MockTransactionSigner::new());
    let balance = assets
        .balance_of(
            BalanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(balance.as_u64(), 0xdeadbeef);
}

#[tokio::test]
async fn test_transfer() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, _| method == "eth_getBalance")
        .returning(|_, _| Ok(serde_json::json!("0xdeadbeef")));
    let mut signer = MockTransactionSigner::new();
    let expected_tx = TypedTransaction::Legacy(TransactionRequest {
        from: Some(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap()),
        to: Some(NameOrAddress::Address(
            ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap(),
        )),
        value: Some(U256::from(0x10_u64)),
        ..Default::default()
    });
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| *chain_id == 1_u64 && tx == &expected_tx)
        .returning(|_, _| {
            Ok(TxHash::decode_hex("0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f").unwrap())
        });
    let assets = setup(1_u64, provider, signer);
    let tx_hash = assets
        .transfer(
            TransferOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(
        tx_hash.encode_hex(),
        "0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f"
    );
}

#[tokio::test]
async fn test_transfer_with_insufficient_balance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, _| method == "eth_getBalance")
        .returning(|_, _| Ok(serde_json::json!("0x00")));
    let assets = setup(1, provider, MockTransactionSigner::new());
    let error = assets
        .transfer(
            TransferOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await
        .unwrap_err();
    match error {
        PublicAssetsError::InsufficientBalanceError(balance, amount) => {
            assert_eq!(balance.as_u64(), 0x00);
            assert_eq!(amount.as_u64(), 0x10);
        }
        _ => panic!("unexpected error: {:?}", error),
    }
}

#[tokio::test]
async fn test_erc20_balance_of() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let assets = setup(1, provider, MockTransactionSigner::new());
    let balance = assets
        .erc20_balance_of(
            Erc20BalanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(balance.as_u64(), 0xdeadbeef_u64);
}

#[tokio::test]
async fn test_erc20_approve() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, params| {
            if method != "eth_call" {
                return false;
            }
            match params {
                JsonRpcParams::Value(value) => {
                    let params = serde_json::to_string(value).unwrap();
                    // the balanceOf and allowance method signatures
                    params.contains("0x70a08231") || params.contains("0xdd62ed3e")
                }
                _ => false,
            }
        })
        .returning(|_, params| match params {
            JsonRpcParams::Value(params) => {
                let params = serde_json::to_string(&params).unwrap();
                if params.contains("0xdd62ed3e") {
                    Ok(serde_json::json!(U256::from(0x0_u64).encode_hex()))
                } else {
                    Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex()))
                }
            }
            _ => Err(ethers_providers::ProviderError::CustomError(
                "unexpected params".to_string(),
            )),
        });
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(|chain_id, tx| *chain_id == 1_u64 && tx.data().unwrap().clone().encode_hex().contains("095ea7b3"))
        .returning(|_, _| {
            Ok(TxHash::decode_hex("0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f").unwrap())
        });
    let assets = setup(1, provider, signer);
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(
        tx_hash.unwrap().encode_hex(),
        "0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f"
    );
}

#[tokio::test]
async fn test_erc20_approve_with_sufficient_allowance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let assets = setup(1, provider, MockTransactionSigner::new());
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await
        .unwrap();
    assert!(tx_hash.is_none());
}

#[tokio::test]
async fn test_erc20_approve_with_insufficient_balance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, params| match params {
            JsonRpcParams::Value(params) => {
                let params = serde_json::to_string(&params).unwrap();
                if params.contains("0xdd62ed3e") {
                    Ok(serde_json::json!(U256::from(0x0_u64).encode_hex()))
                } else {
                    Ok(serde_json::json!(U256::from(0x9_u64).encode_hex()))
                }
            }
            _ => Err(ethers_providers::ProviderError::CustomError(
                "unexpected params".to_string(),
            )),
        });
    let assets = setup(1, provider, MockTransactionSigner::new());
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await;
    match tx_hash.unwrap_err() {
        PublicAssetsError::InsufficientBalanceError(balance, amount) => {
            assert_eq!(balance.as_u64(), 0x9);
            assert_eq!(amount.as_u64(), 0x10);
        }
        _ => panic!("unexpected error"),
    }
}

#[tokio::test]
async fn test_erc20_transfer() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(|chain_id, tx| *chain_id == 1_u64 && tx.data().unwrap().clone().encode_hex().contains("a9059cbb"))
        .returning(|_, _| {
            Ok(TxHash::decode_hex("0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f").unwrap())
        });
    let assets = setup(1, provider, signer);
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(
        tx_hash.encode_hex(),
        "0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f"
    );
}

#[tokio::test]
async fn test_erc20_transfer_with_insufficient_balance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0x9_u64).encode_hex())));
    let assets = setup(1, provider, MockTransactionSigner::new());
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await;
    match tx_hash.unwrap_err() {
        PublicAssetsError::InsufficientBalanceError(balance, amount) => {
            assert_eq!(balance.as_u64(), 0x9);
            assert_eq!(amount.as_u64(), 0x10);
        }
        _ => panic!("unexpected error"),
    }
}

#[tokio::test]
async fn test_erc20_transfer_with_insufficient_allowance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, params| match params {
            JsonRpcParams::Value(params) => {
                let params = serde_json::to_string(&params).unwrap();
                if params.contains("0xdd62ed3e") {
                    Ok(serde_json::json!(U256::from(0x0_u64).encode_hex()))
                } else {
                    Ok(serde_json::json!(U256::from(0x10_u64).encode_hex()))
                }
            }
            _ => Err(ethers_providers::ProviderError::CustomError(
                "unexpected params".to_string(),
            )),
        });
    let assets = setup(1, provider, MockTransactionSigner::new());
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .build(),
        )
        .await;
    match tx_hash.unwrap_err() {
        PublicAssetsError::InsufficientAllowanceError(allowance, amount) => {
            assert_eq!(allowance.as_u64(), 0x0);
            assert_eq!(amount.as_u64(), 0x10);
        }
        _ => panic!("unexpected error"),
    }
}

fn setup(
    chain_id: u64,
    provider: MockProvider,
    signer: MockTransactionSigner,
) -> PublicAssets<MockProviders, MockTransactionSigner> {
    let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
    let mut providers = MockProviders::new();
    providers
        .expect_get_provider()
        .withf(move |id| *id == chain_id)
        .returning(move |_| Ok(provider.clone()));
    PublicAssets::builder().providers(providers).signer(signer).build()
}

mock! {
    #[derive(Debug)]
    Provider {}

    #[async_trait]
    impl mystiko_ethers::JsonRpcClientWrapper for Provider {
         async fn request(
            &self,
            method: &str,
            params: mystiko_ethers::JsonRpcParams,
        ) -> Result<serde_json::Value, ethers_providers::ProviderError>;
    }
}

mock! {
    #[derive(Debug)]
    Providers {}

    #[async_trait]
    impl mystiko_ethers::Providers for Providers {
        async fn get_provider(&self, chain_id: u64) -> anyhow::Result<Arc<Provider>>;
        async fn has_provider(&self, chain_id: u64) -> bool;
        async fn set_provider(&self, chain_id: u64, provider: Arc<Provider>) -> Option<Arc<Provider>>;
        async fn delete_provider(&self, chain_id: u64) -> Option<Arc<Provider>>;
    }
}

mock! {
    TransactionSigner {}

    #[async_trait]
    impl mystiko_core::TransactionSigner for TransactionSigner {
        async fn address(&self) -> anyhow::Result<Address>;
        async fn send_transaction(&self, chain_id: u64, tx: TypedTransaction) -> anyhow::Result<TxHash>;
    }
}
