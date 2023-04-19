use crate::error::MystikoError;
use crate::types::Result;
use mystiko_config::wrapper::mystiko::MystikoConfig;
use mystiko_database::database::Database;
use mystiko_database::document::contract::Contract;
use mystiko_storage::document::{Document, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::storage::Storage;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ContractHandler<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    db: Arc<Mutex<Database<F, R, S>>>,
    config: Arc<MystikoConfig>,
}

impl<F, R, S> ContractHandler<F, R, S>
where
    F: StatementFormatter,
    R: DocumentRawData,
    S: Storage<R>,
{
    pub fn new(db: Arc<Mutex<Database<F, R, S>>>, config: Arc<MystikoConfig>) -> Self {
        Self { db, config }
    }

    pub async fn find(&mut self, query_filter: QueryFilter) -> Result<Vec<Document<Contract>>> {
        self.db
            .lock()
            .await
            .contracts
            .find(query_filter)
            .await
            .map_err(MystikoError::DatabaseError)
    }
}
