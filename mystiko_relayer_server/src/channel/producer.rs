use crate::document::transaction::Transaction;
use crate::error::RelayerServerError;
use crate::handler::transaction::{TransactionHandler, UpdateTransactionOptions};
use crate::types::Result;
use log::info;
use mystiko_relayer_types::{TransactRequestData, TransactStatus};
use mystiko_storage::document::Document;
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct TransactionProducer {
    pub supported_erc20_tokens: Vec<String>,
    sender: Arc<Sender<(String, TransactRequestData)>>,
    handler: Arc<TransactionHandler<SqlStatementFormatter, SqliteStorage>>,
}

impl TransactionProducer {
    pub fn new(
        supported_erc20_tokens: Vec<String>,
        sender: Arc<Sender<(String, TransactRequestData)>>,
        handler: Arc<TransactionHandler<SqlStatementFormatter, SqliteStorage>>,
    ) -> Self {
        Self {
            supported_erc20_tokens,
            sender,
            handler,
        }
    }

    pub async fn send(&self, data: TransactRequestData) -> Result<Document<Transaction>> {
        // save data to database
        let transaction = self.handler.create_by_request(data.clone()).await?;
        info!("successfully created a transaction(id = {})", &transaction.id);

        // send transaction to queue
        let queue = self
            .sender
            .send((transaction.id.clone(), data))
            .await
            .map_err(|e| RelayerServerError::QueueSendError(e.to_string()));

        match queue {
            Ok(_) => {
                info!("successfully sent a transaction to queue(id = {})", &transaction.id);
                Ok(transaction)
            }
            Err(err) => {
                self.handler
                    .update_by_id(
                        &transaction.id,
                        &UpdateTransactionOptions::builder()
                            .status(TransactStatus::Failed)
                            .error_message(err.to_string())
                            .build(),
                    )
                    .await?;
                Err(err)
            }
        }
    }
}
