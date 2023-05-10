#![forbid(unsafe_code)]
extern crate anyhow;
extern crate async_trait;
extern crate num_traits;
extern crate sqlx;

use anyhow::Result;
use async_trait::async_trait;
use num_traits::{Float, PrimInt};
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{ConnectOptions, Executor, Row, SqliteConnection};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use mystiko_storage::document::DocumentRawData;
use mystiko_storage::error::StorageError;
use mystiko_storage::storage::Storage;

pub struct SqliteRawData {
    row: SqliteRow,
}

#[derive(Debug)]
pub struct SqliteStorage {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl Debug for SqliteRawData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.row.columns().fmt(f)
    }
}

impl DocumentRawData for SqliteRawData {
    fn field_integer_value<T: PrimInt + FromStr>(&self, field: &str) -> Result<Option<T>, StorageError> {
        match self.row.try_get::<Option<i64>, &str>(field) {
            Ok(Some(v)) => Ok(T::from(v)),
            Ok(None) => Ok(None),
            Err(e) => Err(StorageError::CorruptedDataError(format!(
                "failed to parse column {} value: {}",
                field, e
            ))),
        }
    }

    fn field_float_value<T: Float + FromStr>(&self, field: &str) -> Result<Option<T>, StorageError> {
        match self.row.try_get::<Option<f64>, &str>(field) {
            Ok(Some(v)) => Ok(T::from(v)),
            Ok(None) => Ok(None),
            Err(e) => Err(StorageError::CorruptedDataError(format!(
                "failed to parse column {} value: {}",
                field, e
            ))),
        }
    }

    fn field_string_value(&self, field: &str) -> Result<Option<String>, StorageError> {
        match self.row.try_get::<Option<String>, &str>(field) {
            Ok(v) => Ok(v),
            Err(e) => Err(StorageError::CorruptedDataError(format!(
                "failed to parse column {} value: {}",
                field, e
            ))),
        }
    }
}

#[async_trait]
impl Storage<SqliteRawData> for SqliteStorage {
    async fn execute(&self, statement: String) -> Result<(), StorageError> {
        let mut connection = self.connection.lock().await;
        let result = connection.execute(sqlx::query(&statement)).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(StorageError::DatabaseError(e.into())),
        }
    }

    async fn query(&self, statement: String) -> Result<Vec<SqliteRawData>, StorageError> {
        let mut connection = self.connection.lock().await;
        let results = connection.fetch_all(sqlx::query(&statement)).await;
        match results {
            Ok(mut rows) => {
                let mut documents: Vec<SqliteRawData> = Vec::new();
                while !rows.is_empty() {
                    documents.push(SqliteRawData { row: rows.remove(0) });
                }
                Ok(documents)
            }
            Err(e) => Err(StorageError::DatabaseError(e.into())),
        }
    }

    async fn collection_exists(&self, collection: &str) -> Result<bool, StorageError> {
        let mut connection = self.connection.lock().await;
        let results = connection
            .fetch_all(
                sqlx::query("SELECT `name` FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?")
                    .bind(collection),
            )
            .await;
        match results {
            Ok(rows) => Ok(!rows.is_empty()),
            Err(e) => Err(StorageError::DatabaseError(e.into())),
        }
    }
}

#[derive(Default)]
pub struct SqliteStorageBuilder {
    path: String,
}

static SQLITE_MEMORY_PATH: &str = ":memory:";

impl SqliteStorageBuilder {
    pub fn new() -> SqliteStorageBuilder {
        SqliteStorageBuilder {
            path: String::from(SQLITE_MEMORY_PATH),
        }
    }

    pub fn in_memory(mut self) -> Self {
        self.path = String::from(SQLITE_MEMORY_PATH);
        self
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub async fn build(self) -> Result<SqliteStorage, StorageError> {
        let connection_result = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(&self.path)
            .connect()
            .await;
        match connection_result {
            Ok(connection) => Ok(SqliteStorage {
                connection: Arc::new(Mutex::new(connection)),
            }),
            Err(e) => Err(StorageError::DatabaseError(e.into())),
        }
    }
}
