extern crate ethers_providers;
extern crate ethers_signers;
extern crate rand;
extern crate serde_json;
extern crate tokio;

use ethers_core::types::{U256, U64};
use ethers_core::utils::Anvil;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use mystiko_server_utils::tx_manager::config::TxManagerConfig;
use mystiko_server_utils::tx_manager::transaction::TxBuilder;

#[tokio::test]
async fn test_send_1559_tx() {
    // Spawn local node
    let anvil = Anvil::new().spawn();
    let endpoint = anvil.endpoint();

    let provider = Provider::<Http>::try_from(endpoint).unwrap();
    let chain_id = provider.get_chainid().await.unwrap();
    let wallet: LocalWallet = anvil.keys().first().unwrap().clone().into();
    let wallet = wallet.with_chain_id(chain_id.as_u64());
    let cfg = TxManagerConfig::new("testnet", None).unwrap();
    let to = anvil.addresses()[1];
    let value = ethers_core::utils::parse_ether("1").unwrap();

    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id.as_u64().into())
        .to(to)
        .wallet(wallet)
        .build();
    let mut tx = builder.build_tx(&provider).await;
    assert!(tx.is_1559_tx());

    let gas_price = tx.gas_price(&provider).await.unwrap();
    assert!(gas_price > U256::zero());

    let gas = tx.estimate_gas(vec![].as_slice(), value, &provider).await.unwrap();
    assert!(gas > U256::zero());

    let max_gas_price = Some(U256::from(100_000_000_000u64));
    let before = provider.get_balance(to, None).await.unwrap();
    let tx_hash = tx
        .send(vec![].as_slice(), value, max_gas_price, &provider)
        .await
        .unwrap();
    let receipt = tx.confirm(&provider).await.unwrap();
    assert_ne!(receipt.block_number.unwrap(), U64::from(0));
    assert_ne!(receipt.status.unwrap(), U64::from(0));
    assert_eq!(receipt.transaction_hash, tx_hash);
    let after = provider.get_balance(to, None).await.unwrap();
    assert_eq!(before + value, after);

    drop(anvil);
}

#[tokio::test]
async fn test_send_legacy_tx() {
    // Spawn local node
    let anvil = Anvil::new().spawn();
    let endpoint = anvil.endpoint();

    let provider = Provider::<Http>::try_from(endpoint).unwrap();
    let chain_id: U64 = provider.get_chainid().await.unwrap().as_u64().into();
    let wallet: LocalWallet = anvil.keys().first().unwrap().clone().into();
    let wallet = wallet.with_chain_id(chain_id.as_u64());

    let force_chain = vec![chain_id];
    let mut cfg = TxManagerConfig::new("testnet", None).unwrap();
    cfg.force_gas_price_chains = force_chain;

    let to = anvil.addresses()[1];
    let value = ethers_core::utils::parse_ether("1").unwrap();

    let builder = TxBuilder::builder()
        .config(cfg)
        .chain_id(chain_id)
        .to(to)
        .wallet(wallet)
        .build();
    let mut tx = builder.build_tx(&provider).await;
    assert!(!tx.is_1559_tx());

    let gas_price = tx.gas_price(&provider).await.unwrap();
    assert!(gas_price > U256::zero());

    let gas = tx.estimate_gas(vec![].as_slice(), value, &provider).await.unwrap();
    assert!(gas > U256::zero());

    let max_gas_price = Some(U256::from(100_000_000_000u64));
    let before = provider.get_balance(to, None).await.unwrap();
    let tx_hash = tx
        .send(vec![].as_slice(), value, max_gas_price, &provider)
        .await
        .unwrap();
    let receipt = tx.confirm(&provider).await.unwrap();
    assert_ne!(receipt.block_number.unwrap(), U64::from(0));
    assert_ne!(receipt.status.unwrap(), U64::from(0));
    assert_eq!(receipt.transaction_hash, tx_hash);
    let after = provider.get_balance(to, None).await.unwrap();
    assert_eq!(before + value, after);

    drop(anvil);
}
