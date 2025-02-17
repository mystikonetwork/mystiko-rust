use crate::{
    BalanceOptions, Erc20AllowanceOptions, Erc20ApproveOptions, Erc20BalanceOptions, Erc20TransferOptions, FromContext,
    MystikoContext, MystikoError, PublicAssetHandler, TransactionSigner, TransferOptions,
};
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, TxHash, U256};
use ethers_providers::Middleware;
use mystiko_abi::erc20::ERC20;
use mystiko_ethers::{Provider, Providers};
use mystiko_storage::{StatementFormatter, Storage};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
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
    #[error("balance_of timed out after {0} ms")]
    BalanceOfTimeoutError(u64),
    #[error("transfer timed out after {0} ms")]
    TransferTimeoutError(u64),
    #[error("erc20_balance_of timed out after {0} ms")]
    Erc20BalanceOfTimeoutError(u64),
    #[error("erc20_allowance timed out after {0} ms")]
    Erc20AllowanceTimeoutError(u64),
    #[error("erc20_approve timed out after {0} ms")]
    Erc20ApproveTimeoutError(u64),
    #[error("erc20_transfer timed out after {0} ms")]
    Erc20TransferTimeoutError(u64),
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
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                provider.get_balance(options.owner, None),
            )
            .await
            {
                Err(_) => Err(PublicAssetsError::BalanceOfTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(provider.get_balance(options.owner, None).await?)
        }
    }

    async fn transfer<T, S>(&self, options: TransferOptions<T, S>) -> Result<TxHash, Self::Error>
    where
        S: TransactionSigner + 'static,
        T: Into<TypedTransaction> + Clone + Default + Send + Sync + 'static,
    {
        let balance_options = BalanceOptions::builder()
            .chain_id(options.chain_id)
            .owner(options.owner)
            .timeout_ms(options.timeout_ms)
            .build();
        let balance = self.balance_of(balance_options).await?;
        if balance < options.amount {
            return Err(PublicAssetsError::InsufficientBalanceError(balance, options.amount));
        }
        let mut tx = options.tx.into();
        tx.set_from(options.owner);
        tx.set_to(options.recipient);
        tx.set_value(options.amount);
        let result = if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                options.signer.send_transaction(options.chain_id, tx),
            )
            .await
            {
                Err(_) => return Err(PublicAssetsError::TransferTimeoutError(timeout_ms)),
                Ok(result) => result,
            }
        } else {
            options.signer.send_transaction(options.chain_id, tx).await
        };
        Ok(result.map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?)
    }

    async fn erc20_balance_of(&self, options: Erc20BalanceOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        let contract = ERC20::new(options.asset_address, provider);
        erc20_balance_of(&contract, options.owner, options.timeout_ms).await
    }

    async fn erc20_allowance(&self, options: Erc20AllowanceOptions) -> Result<U256, Self::Error> {
        let provider = self
            .providers
            .get_provider(options.chain_id)
            .await
            .map_err(PublicAssetsError::ProviderPoolError)?;
        let contract = ERC20::new(options.asset_address, provider);
        erc20_allowance(&contract, options.owner, options.recipient, options.timeout_ms).await
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
        let balance = erc20_balance_of(&contract, options.owner, options.timeout_ms).await?;
        if balance < options.amount {
            return Err(PublicAssetsError::InsufficientBalanceError(balance, options.amount));
        }
        let allowance = erc20_allowance(&contract, options.owner, options.recipient, options.timeout_ms).await?;
        if allowance.lt(&options.amount) {
            let mut tx = options.tx.into();
            tx.set_to(options.asset_address);
            if let Some(call_data) = contract.approve(options.recipient, options.amount).calldata() {
                tx.set_data(call_data);
            }
            let tx_hash = if let Some(timeout_ms) = options.timeout_ms {
                match tokio::time::timeout(
                    Duration::from_millis(timeout_ms),
                    options.signer.send_transaction(options.chain_id, tx),
                )
                .await
                {
                    Err(_) => return Err(PublicAssetsError::Erc20ApproveTimeoutError(timeout_ms)),
                    Ok(result) => result,
                }
            } else {
                options.signer.send_transaction(options.chain_id, tx).await
            };
            Ok(Some(
                tx_hash.map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?,
            ))
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
        let result = if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(
                Duration::from_millis(timeout_ms),
                options.signer.send_transaction(options.chain_id, tx),
            )
            .await
            {
                Err(_) => return Err(PublicAssetsError::Erc20TransferTimeoutError(timeout_ms)),
                Ok(result) => result,
            }
        } else {
            options.signer.send_transaction(options.chain_id, tx).await
        };
        Ok(result.map_err(|err| PublicAssetsError::SignerError(format!("{:?}", err)))?)
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for PublicAssets<Box<dyn Providers>>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(Self {
            providers: context.providers.clone(),
        })
    }
}

async fn erc20_balance_of(
    contract: &ERC20<Provider>,
    owner: Address,
    timeout_ms: Option<u64>,
) -> Result<U256, PublicAssetsError> {
    if let Some(timeout_ms) = timeout_ms {
        match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
            contract.balance_of(owner).await
        })
        .await
        {
            Err(_) => Err(PublicAssetsError::Erc20BalanceOfTimeoutError(timeout_ms)),
            Ok(result) => Ok(result?),
        }
    } else {
        Ok(contract.balance_of(owner).await?)
    }
}

async fn erc20_allowance(
    contract: &ERC20<Provider>,
    owner: Address,
    spender: Address,
    timeout_ms: Option<u64>,
) -> Result<U256, PublicAssetsError> {
    if let Some(timeout_ms) = timeout_ms {
        match tokio::time::timeout(Duration::from_millis(timeout_ms), async {
            contract.allowance(owner, spender).await
        })
        .await
        {
            Err(_) => Err(PublicAssetsError::Erc20AllowanceTimeoutError(timeout_ms)),
            Ok(result) => Ok(result?),
        }
    } else {
        Ok(contract.allowance(owner, spender).await?)
    }
}
