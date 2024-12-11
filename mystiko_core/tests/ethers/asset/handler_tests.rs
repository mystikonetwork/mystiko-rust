use crate::common::{MockProvider, MockProviders, MockTransactionSigner};
use crate::ethers::TimeoutProvider;
use ethers_core::abi::{AbiDecode, AbiEncode};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{NameOrAddress, TransactionRequest, TxHash, U256};
use mystiko_core::{
    BalanceOptions, Erc20AllowanceOptions, Erc20ApproveOptions, Erc20BalanceOptions, Erc20TransferOptions,
    PublicAssetHandler, PublicAssets, PublicAssetsError, TransferOptions,
};
use mystiko_ethers::{JsonRpcClientWrapper, JsonRpcParams, Provider, ProviderWrapper};
use mystiko_utils::address::ethers_address_from_string;
use std::sync::Arc;

#[tokio::test]
async fn test_balance_of() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, _| method == "eth_getBalance")
        .returning(|_, _| Ok(serde_json::json!("0xdeadbeef")));
    let assets = setup(1, provider);
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
async fn test_balance_of_timeout_error() {
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1, provider);
    let balance = assets
        .balance_of(
            BalanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(balance.unwrap_err().to_string(), "balance_of timed out after 1 ms");
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
    let assets = setup(1_u64, provider);
    let tx_hash = assets
        .transfer(
            TransferOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .amount(0x10_u64)
                .signer(signer)
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
    let assets = setup(1, provider);
    let error = assets
        .transfer(
            TransferOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .amount(0x10_u64)
                .signer(MockTransactionSigner::new())
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
async fn test_transfer_timeout_error() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(move |method, _| method == "eth_getBalance")
        .returning(|_, _| Ok(serde_json::json!("0xdeadbeef")));
    let signer = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1_u64, provider);
    let result = assets
        .transfer(
            TransferOptions::<TransactionRequest, TimeoutProvider>::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .amount(0x10_u64)
                .signer(signer)
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(result.unwrap_err().to_string(), "transfer timed out after 1 ms");
}

#[tokio::test]
async fn test_erc20_balance_of() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let assets = setup(1, provider);
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
async fn test_erc20_balance_of_timeout_error() {
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1, provider);
    let result = assets
        .erc20_balance_of(
            Erc20BalanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(result.unwrap_err().to_string(), "erc20_balance_of timed out after 1 ms");
}

#[tokio::test]
async fn test_erc20_allowance() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let assets = setup(1, provider);
    let balance = assets
        .erc20_allowance(
            Erc20AllowanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .build(),
        )
        .await
        .unwrap();
    assert_eq!(balance.as_u64(), 0xdeadbeef_u64);
}

#[tokio::test]
async fn test_erc20_allowance_timeout_error() {
    let provider = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1, provider);
    let result = assets
        .erc20_allowance(
            Erc20AllowanceOptions::builder()
                .chain_id(1_u64)
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(result.unwrap_err().to_string(), "erc20_allowance timed out after 1 ms");
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(signer)
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
async fn test_erc20_approve_reset_allowance() {
    let mut provider = MockProvider::new();
    let amount = U256::from(1234567890123456789_u64);
    let amount_hex = format!("{:x}", amount);
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
        .returning(move |_, params| match params {
            JsonRpcParams::Value(params) => {
                let params = serde_json::to_string(&params).unwrap();
                if params.contains("0xdd62ed3e") {
                    Ok(serde_json::json!(U256::from(0x2_u64).encode_hex()))
                } else {
                    Ok(serde_json::json!(amount.encode_hex()))
                }
            }
            _ => Err(ethers_providers::ProviderError::CustomError(
                "unexpected params".to_string(),
            )),
        });
    let mut signer = MockTransactionSigner::new();
    signer
        .expect_send_transaction()
        .withf(move |chain_id, tx| {
            *chain_id == 1_u64
                && tx.data().unwrap().clone().encode_hex().contains("095ea7b3")
                && tx.data().unwrap().clone().encode_hex().contains(amount_hex.as_str())
        })
        .returning(|_, _| {
            Ok(TxHash::decode_hex("0xbabc0eb1e1d720da01feefb176bae8683183dc8b2a4d599e91bda5efca9ef60f").unwrap())
        });
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(amount)
                .signer(signer)
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(MockTransactionSigner::new())
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(MockTransactionSigner::new())
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
async fn test_erc20_approve_timeout_error() {
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
    let signer = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1, provider);
    let result = assets
        .erc20_approve(
            Erc20ApproveOptions::<TransactionRequest, TimeoutProvider>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(signer)
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(result.unwrap_err().to_string(), "erc20_approve timed out after 1 ms");
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(signer)
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(MockTransactionSigner::new())
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
    let assets = setup(1, provider);
    let tx_hash = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest, MockTransactionSigner>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(MockTransactionSigner::new())
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

#[tokio::test]
async fn test_erc20_transfer_timeout_error() {
    let mut provider = MockProvider::new();
    provider
        .expect_request()
        .withf(|method, _| method == "eth_call")
        .returning(|_, _| Ok(serde_json::json!(U256::from(0xdeadbeef_u64).encode_hex())));
    let signer = TimeoutProvider::builder().timeout_ms(1000_u64).build();
    let assets = setup(1, provider);
    let result = assets
        .erc20_transfer(
            Erc20TransferOptions::<TransactionRequest, TimeoutProvider>::builder()
                .chain_id(1_u64)
                .asset_address(ethers_address_from_string("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap())
                .owner(ethers_address_from_string("0x8e22c73915cbcb5bda1cd8a15a7e2a6c1d370335").unwrap())
                .recipient(ethers_address_from_string("0xF0bAfD58E23726785A1681e1DEa0da15cB038C61").unwrap())
                .amount(0x10_u64)
                .signer(signer)
                .timeout_ms(1_u64)
                .build(),
        )
        .await;
    assert_eq!(result.unwrap_err().to_string(), "erc20_transfer timed out after 1 ms");
}

fn setup<P>(chain_id: u64, provider: P) -> PublicAssets<MockProviders>
where
    P: JsonRpcClientWrapper + 'static,
{
    let provider = Arc::new(Provider::new(ProviderWrapper::new(Box::new(provider))));
    let mut providers = MockProviders::new();
    providers
        .expect_get_provider()
        .withf(move |id| *id == chain_id)
        .returning(move |_| Ok(provider.clone()));
    PublicAssets::builder().providers(providers).build()
}
