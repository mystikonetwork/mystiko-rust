use crate::common::env::load_roller_db_path;
use crate::db::collection::commitment::CommitmentInfoCollection;
use crate::db::document::commitment::CommitmentInfo;
use anyhow::Result;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentRawData};
use mystiko_storage::filter::{Condition, Order, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use mystiko_storage_sqlite::{SqliteRawData, SqliteStorage, SqliteStorageBuilder};
use std::path::PathBuf;
use std::sync::Arc;

pub struct RollerDatabase<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    pub commitments: CommitmentInfoCollection<F, R, S>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> RollerDatabase<F, R, S> {
    pub async fn new(formatter: F, storage: S) -> Self {
        let collection = Arc::new(Collection::new(formatter, storage));
        let db = RollerDatabase {
            commitments: CommitmentInfoCollection::new(collection.clone()),
        };
        let _ = db.commitments.migrate().await.expect("load db meet error");
        db
    }

    pub async fn migrate(&self) -> Result<Vec<Document<Migration>>> {
        let migrations: Vec<Document<Migration>> = vec![self.commitments.migrate().await?];
        Ok(migrations)
    }

    // pub async fn find_latest_commitment_block_number(&self, chain_id: u64, contract_address: &str) -> Option<u64> {
    //     let qf = QueryFilterBuilder::new()
    //         .filter(Condition::AND(
    //             SubFilter::Equal(String::from("contract_address"), contract_address.to_string()),
    //             SubFilter::Equal(String::from("chain_id"), chain_id.to_string()),
    //         ))
    //         .limit(1)
    //         .order_by(vec![String::from("leaf_index")], Order::DESC)
    //         .build();
    //     let found_commitments = self.commitments.find(qf).await.expect("read database meet error");
    //     if found_commitments.is_empty() {
    //         None
    //     } else {
    //         Some(found_commitments[0].data.block_number)
    //     }
    // }

    pub async fn find_all_commitment(&self, chain_id: u64, contract_address: &str) -> Vec<Document<CommitmentInfo>> {
        let qf = QueryFilterBuilder::new()
            .filter(Condition::AND(vec![
                SubFilter::Equal(String::from("contract_address"), contract_address.to_string()),
                SubFilter::Equal(String::from("chain_id"), chain_id.to_string()),
            ]))
            .order_by(vec![String::from("leaf_index")], Order::ASC)
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

pub async fn create_roller_database() -> RollerDatabase<SqlFormatter, SqliteRawData, SqliteStorage> {
    let formatter = SqlFormatter {};
    let config_path = load_roller_db_path();
    let mut db_path = PathBuf::from(config_path);
    db_path.push("roller.db");
    let storage = SqliteStorageBuilder::new()
        .path(db_path.to_str().unwrap())
        .build()
        .await
        .unwrap();
    RollerDatabase::new(formatter, storage).await
}
