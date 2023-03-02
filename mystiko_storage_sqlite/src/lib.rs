#![forbid(unsafe_code)]
extern crate async_trait;
extern crate futures;
extern crate num_traits;
extern crate sqlx;

use async_trait::async_trait;
use futures::lock::Mutex;
use num_traits::{Float, PrimInt};
use sqlx::sqlite::{SqliteConnectOptions, SqliteRow};
use sqlx::{ConnectOptions, Executor, Row, SqliteConnection};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;

use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::storage::Storage;

pub struct SqliteRawData {
    row: SqliteRow,
}

pub struct SqliteStorage {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl DocumentRawData for SqliteRawData {
    fn field_integer_value<T: PrimInt + FromStr>(&self, field: &str) -> Result<Option<T>, Error> {
        match self.row.try_get::<Option<i64>, &str>(field) {
            Ok(Some(v)) => Ok(T::from(v)),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("failed to parse column {} value: {}", field, e),
            )),
        }
    }

    fn field_float_value<T: Float + FromStr>(&self, field: &str) -> Result<Option<T>, Error> {
        match self.row.try_get::<Option<f64>, &str>(field) {
            Ok(Some(v)) => Ok(T::from(v)),
            Ok(None) => Ok(None),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("failed to parse column {} value: {}", field, e),
            )),
        }
    }

    fn field_string_value(&self, field: &str) -> Result<Option<String>, Error> {
        match self.row.try_get::<Option<String>, &str>(field) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("failed to parse column {} value: {}", field, e),
            )),
        }
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn execute(&mut self, statement: String) -> Result<(), Error> {
        let mut connection = self.connection.lock().await;
        let result = connection.execute(sqlx::query(&statement)).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("sqlite execution error: {}", e),
            )),
        }
    }

    async fn query<T: DocumentData>(
        &mut self,
        statement: String,
    ) -> Result<Vec<Document<T>>, Error> {
        let mut connection = self.connection.lock().await;
        let results = connection.fetch_all(sqlx::query(&statement)).await;
        match results {
            Ok(mut rows) => {
                let mut documents: Vec<Document<T>> = Vec::new();
                while !rows.is_empty() {
                    let document = Document::<T>::deserialize(&SqliteRawData {
                        row: rows.remove(0),
                    })?;
                    documents.push(document);
                }
                Ok(documents)
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("sqlite querying error: {}", e),
            )),
        }
    }

    async fn collection_exists(&mut self, collection: &str) -> Result<bool, Error> {
        let mut connection = self.connection.lock().await;
        let results = connection
            .fetch_all(
                sqlx::query(
                    "SELECT `name` FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?",
                )
                .bind(collection),
            )
            .await;
        match results {
            Ok(rows) => Ok(!rows.is_empty()),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("sqlite checking table existence error: {}", e),
            )),
        }
    }
}

pub struct SqliteStorageBuilder {
    path: String,
}

static SQLITE_MEMORY_PATH: &'static str = ":memory:";

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

    pub async fn build(self) -> Result<SqliteStorage, Error> {
        let connection_result = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(&self.path)
            .connect()
            .await;
        match connection_result {
            Ok(connection) => Ok(SqliteStorage {
                connection: Arc::new(Mutex::new(connection)),
            }),
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("failed to create sqlite connection pool: {}", e),
            )),
        }
    }
}
