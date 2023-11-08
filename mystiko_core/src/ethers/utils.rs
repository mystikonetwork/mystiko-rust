use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::transaction::eip2930::AccessList;
use ethers_core::types::{
    Eip1559TransactionRequest, Eip2930TransactionRequest, TransactionReceipt, TransactionRequest, TxHash,
};
use ethers_providers::{PendingTransaction, ProviderError};
use mystiko_ethers::Provider;
use mystiko_protos::core::v1::Transaction;
use std::sync::Arc;

pub async fn wait_transaction(
    tx_hash: TxHash,
    provider: Arc<Provider>,
    confirmations: Option<u64>,
) -> Result<Option<TransactionReceipt>, ProviderError> {
    let mut pending_tx = PendingTransaction::new(tx_hash, &provider);
    if let Some(confirmations) = confirmations {
        if confirmations > 0 {
            pending_tx = pending_tx.confirmations(confirmations as usize);
        }
    }
    let receipt = pending_tx.await?;
    Ok(receipt)
}

pub fn convert_transaction(
    transaction_type: &mystiko_types::TransactionType,
    transaction: &Option<Transaction>,
) -> anyhow::Result<TypedTransaction> {
    let default_tx = match transaction_type {
        mystiko_types::TransactionType::Legacy => TypedTransaction::Legacy(TransactionRequest::new()),
        mystiko_types::TransactionType::Eip1559 => TypedTransaction::Eip1559(Eip1559TransactionRequest::new()),
        mystiko_types::TransactionType::Eip2930 => TypedTransaction::Eip2930(Eip2930TransactionRequest::new(
            TransactionRequest::new(),
            AccessList(vec![]),
        )),
    };
    let converted: Option<TypedTransaction> = transaction.clone().map(|tx| tx.try_into()).transpose()?;
    let tx = converted.map(|tx| match transaction_type {
        mystiko_types::TransactionType::Legacy => {
            if let TypedTransaction::Legacy(tx) = tx {
                TypedTransaction::Legacy(tx)
            } else {
                default_tx.clone()
            }
        }
        mystiko_types::TransactionType::Eip1559 => {
            if let TypedTransaction::Eip1559(tx) = tx {
                TypedTransaction::Eip1559(tx)
            } else {
                default_tx.clone()
            }
        }
        mystiko_types::TransactionType::Eip2930 => {
            if let TypedTransaction::Eip2930(tx) = tx {
                TypedTransaction::Eip2930(tx)
            } else {
                default_tx.clone()
            }
        }
    });
    Ok(tx.unwrap_or(default_tx))
}
