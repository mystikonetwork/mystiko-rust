use crate::common::env::load_roller_db_path;
use crate::db::document::commitment::{CommitmentInfo, CommitmentInfoCollection, CommitmentInfoColumn};
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::Document;
use mystiko_storage::filter::{Condition, Order, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::migration::history::MigrationHistory;
use mystiko_storage::storage::Storage;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug)]
pub struct RollerDatabase<F: StatementFormatter, S: Storage> {
    pub commitments: CommitmentInfoCollection<F, S>,
}

impl<F: StatementFormatter, S: Storage> RollerDatabase<F, S> {
    pub async fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Collection::new(formatter, storage));
        let db = RollerDatabase {
            commitments: CommitmentInfoCollection::new(collection.clone()),
        };
        let _ = db.commitments.migrate().await.expect("load db meet error");
        db
    }

    pub async fn migrate(&self) -> Result<Vec<Document<MigrationHistory>>> {
        let migrations: Vec<Document<MigrationHistory>> = vec![self.commitments.migrate().await?];
        Ok(migrations)
    }

    pub async fn find_all_commitment(&self, chain_id: u64, contract_address: &str) -> Vec<Document<CommitmentInfo>> {
        let qf = QueryFilterBuilder::new()
            .filter(Condition::and(vec![
                SubFilter::equal(CommitmentInfoColumn::ContractAddress, contract_address.to_string()),
                SubFilter::equal(CommitmentInfoColumn::ChainId, chain_id.to_string()),
            ]))
            .order_by(CommitmentInfoColumn::LeafIndex, Order::ASC)
            .build();
        self.commitments
            .find(qf)
            .await
            .expect("find all commitments meet error")
    }

    pub async fn insert_commitment(&self, cm: &CommitmentInfo) {
        self.commitments.insert(cm).await.expect("insert commitment meet error");
    }
}

pub async fn create_roller_database() -> RollerDatabase<SqlStatementFormatter, SqliteStorage> {
    let formatter = SqlStatementFormatter::default();
    let config_path = load_roller_db_path();
    let mut db_path = PathBuf::from(config_path);
    db_path.push("roller.db");
    let storage = SqliteStorageBuilder::new()
        .path(db_path.to_str().unwrap())
        .build()
        .await;
    match storage {
        Ok(s) => RollerDatabase::new(formatter, s).await,
        Err(e) => {
            panic!("create sqlite storage meet error: {:?}", e);
        }
    }
}
