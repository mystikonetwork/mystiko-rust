use crate::{
    BalanceOptions, Erc20ApproveOptions, Erc20BalanceOptions, Erc20TransferOptions, PublicAssetHandler,
    TransactionSigner, TransferOptions,
};
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{TxHash, U256};
use ethers_providers::Middleware;
use mystiko_abi::erc20::ERC20;
use mystiko_ethers::{Provider, Providers};
use std::fmt::Debug;
use std::ops::Sub;
use std::sync::Arc;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct PublicAssets<P: Providers> {
    providers: Arc<P>,
}

#[derive(Debug, Error)]
pub enum PublicAssetsError {
    #[error("providers raised error: {0}")]
    ProviderPoolError(anyhow::Error),
    #[error(transparent)]
    ProviderError(#[from] ethers_providers::ProviderError),
    #[error(transparent)]
    ContractError(#[from] ethers_contract::ContractError<Provider>),
    #[error("insufficient balance: {0} < {1}")]
    InsufficientBalanceError(U256, U256),
    #[error("insufficient allowance: {0} < {1}")]
    InsufficientAllowanceError(U256, U256),
    #[error("transaction signer raised error: {0}")]
    SignerError(String),
}

#[async_trait]
impl<P> PublicAssetHandler for PublicAssets<P>
where
    P: Providers,
{
    type Error = PublicAssetsError;

    async fn balance_of(&self, options: BalanceOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        Ok(provider.get_balance(options.owner, None).await?)
    }

    async fn transfer<T, S>(&self, options: TransferOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
    {
        let balance = self
            .balance_of(
                BalanceOptions::builder()
                    .chain_id(options.chain_id)
                    .owner(options.owner)
                    .build(),
            )
            .await?;
        if balance < options.amount {
            return Err(PublicAssetsError::InsufficientBalanceError(balance, options.amount));
        }
        let mut tx = options.tx.into();
        tx.set_from(options.owner);
        tx.set_to(options.recipient);
        tx.set_value(options.amount);
        Ok(options
            .signer
            .send_transaction(options.chain_id, tx)
            .await
            .map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?)
    }

    async fn erc20_balance_of(&self, options: Erc20BalanceOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        let contract = ERC20::new(options.asset_address, provider);
        Ok(contract.balance_of(options.owner).await?)
    }

    async fn erc20_approve<T, S>(&self, options: Erc20ApproveOptions<T, S>) -> Result<Option<TxHash>, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
    {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        let contract = ERC20::new(options.asset_address, provider);
        let balance = contract.balance_of(options.owner).await?;
        if balance < options.amount {
            return Err(PublicAssetsError::InsufficientBalanceError(balance, options.amount));
        }
        let allowance = contract.allowance(options.owner, options.recipient).await?;
        if allowance.le(&options.amount) {
            let mut tx = options.tx.into();
            tx.set_to(options.asset_address);
            let allowance = options.amount.sub(allowance);
            if let Some(call_data) = contract.approve(options.recipient, allowance).calldata() {
                tx.set_data(call_data);
            }
            let tx_hash = options
                .signer
                .send_transaction(options.chain_id, tx)
                .await
                .map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?;
            Ok(Some(tx_hash))
        } else {
            Ok(None)
        }
    }

    async fn erc20_transfer<T, S>(&self, options: Erc20TransferOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
    {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        let contract = ERC20::new(options.asset_address, provider);
        let balance = contract.balance_of(options.owner).await?;
        if balance < options.amount {
            return Err(PublicAssetsError::InsufficientBalanceError(balance, options.amount));
        }
        let allowance = contract.allowance(options.owner, options.recipient).await?;
        if allowance < options.amount {
            return Err(PublicAssetsError::InsufficientAllowanceError(allowance, options.amount));
        }
        let mut tx = options.tx.into();
        tx.set_to(options.asset_address);
        if let Some(call_data) = contract.transfer(options.recipient, options.amount).calldata() {
            tx.set_data(call_data);
        }
        Ok(options
            .signer
            .send_transaction(options.chain_id, tx)
            .await
            .map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?)
    }
}
