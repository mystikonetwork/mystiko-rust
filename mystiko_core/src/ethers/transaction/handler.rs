use crate::{FromContext, MystikoContext, MystikoError, TransactionHandler, WaitOptions};
use async_trait::async_trait;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::{
    Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionReceipt, TransactionRequest,
};
use ethers_providers::PendingTransaction;
use mystiko_ethers::Providers;
use mystiko_protos::core::v1::Transaction;
use mystiko_storage::{StatementFormatter, Storage};
use mystiko_types::TransactionType;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct Transactions<P: Providers> {
    providers: Arc<P>,
}

#[derive(Debug, Error)]
pub enum TransactionsError {
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    ProviderError(#[from] ethers_providers::ProviderError),
    #[error("wait transaction for confirmations timed out after {0} ms")]
    WaitTimeoutError(u64),
}

#[async_trait]
impl<P> TransactionHandler<Transaction> for Transactions<P>
where
    P: Providers,
{
    type Error = TransactionsError;

    fn create(&self, tx: Option<Transaction>, tx_type: &TransactionType) -> Result<TypedTransaction, Self::Error> {
        let default_tx = match tx_type {
            TransactionType::Legacy => TypedTransaction::Legacy(TransactionRequest::new()),
            TransactionType::Eip1559 => TypedTransaction::Eip1559(Eip1559TransactionRequest::new()),
            TransactionType::Eip2930 => TypedTransaction::Eip2930(Eip2930TransactionRequest::new(
                TransactionRequest::new(),
                AccessList(vec![]),
            )),
        };
        let converted: Option<TypedTransaction> = tx.map(|tx| tx.try_into()).transpose()?;
        let tx = converted.map(|tx| match tx_type {
            TransactionType::Legacy => {
                if let TypedTransaction::Legacy(tx) = tx {
                    TypedTransaction::Legacy(tx)
                } else {
                    default_tx.clone()
                }
            }
            TransactionType::Eip1559 => {
                if let TypedTransaction::Eip1559(tx) = tx {
                    TypedTransaction::Eip1559(tx)
                } else {
                    default_tx.clone()
                }
            }
            TransactionType::Eip2930 => {
                if let TypedTransaction::Eip2930(tx) = tx {
                    TypedTransaction::Eip2930(tx)
                } else {
                    default_tx.clone()
                }
            }
        });
        Ok(tx.unwrap_or(default_tx))
    }

    async fn wait(&self, options: WaitOptions) -> Result<Option<TransactionReceipt>, Self::Error> {
        let provider = self.providers.get_provider(options.chain_id).await?;
        let mut pending_tx = PendingTransaction::new(options.tx_hash, &provider);
        if let Some(confirmations) = options.confirmations {
            if confirmations > 0 {
                pending_tx = pending_tx.confirmations(confirmations as usize);
            }
        }
        if let Some(interval_ms) = options.interval_ms {
            pending_tx = pending_tx.interval(Duration::from_millis(interval_ms));
        }
        if let Some(timeout_ms) = options.timeout_ms {
            match tokio::time::timeout(Duration::from_millis(timeout_ms), pending_tx).await {
                Err(_) => Err(TransactionsError::WaitTimeoutError(timeout_ms)),
                Ok(result) => Ok(result?),
            }
        } else {
            Ok(pending_tx.await?)
        }
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for Transactions<Box<dyn Providers>>
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(Self::builder().providers(context.providers.clone()).build())
    }
}
