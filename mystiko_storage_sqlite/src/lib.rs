#![forbid(unsafe_code)]
extern crate anyhow;
extern crate async_trait;
extern crate num_traits;
extern crate serde_json;
extern crate sqlx;

use anyhow::Result;
use async_trait::async_trait;
use mystiko_storage::column::{ColumnType, ColumnValue};
use mystiko_storage::document::{Document, DocumentData};
use sqlx::{ConnectOptions, Row, Sqlite};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

use mystiko_storage::error::StorageError;
use mystiko_storage::formatter::types::{CountStatement, Statement};
use mystiko_storage::storage::Storage;

static SQLITE_MEMORY_PATH: &str = ":memory:";
static DEFAULT_MAX_CONNECTION: u32 = 4;

type Query<'a> = sqlx::query::Query<'a, Sqlite, <Sqlite as sqlx::database::HasArguments<'a>>::Arguments>;

#[derive(Debug)]
enum SqliteConnection {
    Single(Arc<Mutex<sqlx::SqliteConnection>>),
    Pool(sqlx::SqlitePool),
}

#[derive(Debug)]
pub struct SqliteStorage {
    connection: SqliteConnection,
}

#[derive(Default)]
pub struct SqliteStorageBuilder {
    path: String,
    max_connection: Option<u32>,
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn execute(&self, statement: Statement) -> Result<(), StorageError> {
        self.execute_query(statement_to_query(&statement)?).await
    }

    async fn execute_batch(&self, statements: Vec<Statement>) -> Result<(), StorageError> {
        let statement = Statement {
            statement: statements
                .iter()
                .map(|s| s.statement.clone())
                .collect::<Vec<String>>()
                .join(";"),
            column_values: statements.into_iter().flat_map(|s| s.column_values).collect::<Vec<_>>(),
        };
        self.execute_query(statement_to_query(&statement)?).await
    }

    async fn query<T: DocumentData>(&self, statement: Statement) -> Result<Vec<Document<T>>, StorageError> {
        let query = statement_to_query(&statement)?;
        let rows = self.execute_fetch(query).await?;
        Ok(rows_to_documents(&rows)?)
    }

    async fn count(&self, statement: CountStatement) -> Result<u64, StorageError> {
        let query = statement_to_query(&statement.statement)?;
        let rows = self.execute_fetch(query).await?;
        Ok(rows_to_count(&rows, &statement.count_column)?)
    }

    async fn collection_exists(&self, collection: &str) -> Result<bool, StorageError> {
        let query =
            sqlx::query("SELECT `name` FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?").bind(collection);
        let rows = self.execute_fetch(query).await?;
        Ok(!rows.is_empty())
    }
}

impl SqliteStorage {
    async fn execute_query<'a>(&self, query: Query<'a>) -> Result<(), StorageError> {
        match &self.connection {
            SqliteConnection::Single(connection) => {
                let mut connection = connection.lock().await;
                query
                    .execute(&mut *connection)
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?;
            }
            SqliteConnection::Pool(pool) => {
                query
                    .execute(pool)
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?;
            }
        }
        Ok(())
    }

    async fn execute_fetch<'a>(&self, query: Query<'a>) -> Result<Vec<sqlx::sqlite::SqliteRow>, StorageError> {
        let rows = match &self.connection {
            SqliteConnection::Single(connection) => {
                let mut connection = connection.lock().await;
                query
                    .fetch_all(&mut *connection)
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?
            }
            SqliteConnection::Pool(pool) => query
                .fetch_all(pool)
                .await
                .map_err(|e| StorageError::DatabaseError(e.into()))?,
        };
        Ok(rows)
    }
}

impl SqliteStorageBuilder {
    pub fn new() -> SqliteStorageBuilder {
        SqliteStorageBuilder {
            path: String::from(SQLITE_MEMORY_PATH),
            max_connection: None,
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
        let connection = if self.path == SQLITE_MEMORY_PATH {
            SqliteConnection::Single(Arc::new(Mutex::new(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(SQLITE_MEMORY_PATH)
                    .create_if_missing(true)
                    .connect()
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?,
            )))
        } else {
            let options = sqlx::sqlite::SqliteConnectOptions::new()
                .filename(&self.path)
                .create_if_missing(true);
            SqliteConnection::Pool(
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(self.max_connection.unwrap_or(DEFAULT_MAX_CONNECTION))
                    .connect_with(options)
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?,
            )
        };
        Ok(SqliteStorage { connection })
    }
}

fn statement_to_query(statement: &Statement) -> Result<Query<'_>, StorageError> {
    let mut query = sqlx::query::<Sqlite>(&statement.statement);
    query = bind_query(query, statement.column_values.iter().collect::<Vec<_>>())?;
    Ok(query)
}

fn bind_query<'a>(mut query: Query<'a>, values: Vec<&ColumnValue>) -> Result<Query<'a>, StorageError> {
    for value in values {
        match value.column_type() {
            ColumnType::Bool => {
                query = query.bind(value.as_bool()?);
            }
            ColumnType::Char => {
                query = query.bind(value.as_char()?.to_string());
            }
            ColumnType::I8 => {
                query = query.bind(value.as_i8()?);
            }
            ColumnType::I16 => {
                query = query.bind(value.as_i16()?);
            }
            ColumnType::I32 => {
                query = query.bind(value.as_i32()?);
            }
            ColumnType::I64 => {
                query = query.bind(value.as_i64()?);
            }
            ColumnType::I128 => {
                query = query.bind(value.as_i128()?.to_string());
            }
            ColumnType::ISize => {
                query = query.bind(value.as_isize()? as i64);
            }
            ColumnType::U8 => {
                query = query.bind(value.as_u8()?);
            }
            ColumnType::U16 => {
                query = query.bind(value.as_u16()?);
            }
            ColumnType::U32 => {
                query = query.bind(value.as_u32()?);
            }
            ColumnType::U64 => {
                query = query.bind(value.as_u64()?.to_string());
            }
            ColumnType::U128 => {
                query = query.bind(value.as_u128()?.to_string());
            }
            ColumnType::USize => {
                query = query.bind(value.as_usize()?.to_string());
            }
            ColumnType::F32 => {
                query = query.bind(value.as_f32()?);
            }
            ColumnType::F64 => {
                query = query.bind(value.as_f64()?);
            }
            ColumnType::String => {
                query = query.bind(value.as_string()?);
            }
            ColumnType::Json => {
                query = query.bind(serde_json::to_string(&value.as_json()?)?);
            }
        }
    }
    Ok(query)
}

fn rows_to_documents<T: DocumentData>(rows: &[sqlx::sqlite::SqliteRow]) -> Result<Vec<Document<T>>, StorageError> {
    let mut documents: Vec<_> = vec![];
    for row in rows.iter() {
        documents.push(row_to_document::<T>(row)?);
    }
    Ok(documents)
}

fn row_to_document<T: DocumentData>(row: &sqlx::sqlite::SqliteRow) -> Result<Document<T>, StorageError> {
    let mut columns_with_value: Vec<(String, ColumnValue)> = vec![];
    for column in Document::<T>::columns() {
        match column.column_type {
            ColumnType::Bool => {
                if let Some(value) = get_column_value::<bool>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::Bool(value)));
                }
            }
            ColumnType::Char => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    if let Some(value) = value.chars().next() {
                        columns_with_value.push((column.column_name, ColumnValue::Char(value)));
                    }
                }
            }
            ColumnType::I8 => {
                if let Some(value) = get_column_value::<i8>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::I8(value)));
                }
            }
            ColumnType::I16 => {
                if let Some(value) = get_column_value::<i16>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::I16(value)));
                }
            }
            ColumnType::I32 => {
                if let Some(value) = get_column_value::<i32>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::I32(value)));
                }
            }
            ColumnType::I64 => {
                if let Some(value) = get_column_value::<i64>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::I64(value)));
                }
            }
            ColumnType::I128 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: i128 = value.parse()?;
                    columns_with_value.push((column.column_name, ColumnValue::I128(value)));
                }
            }
            ColumnType::ISize => {
                if let Some(value) = get_column_value::<i64>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::ISize(value as isize)));
                }
            }
            ColumnType::U8 => {
                if let Some(value) = get_column_value::<u8>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::U8(value)));
                }
            }
            ColumnType::U16 => {
                if let Some(value) = get_column_value::<u16>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::U16(value)));
                }
            }
            ColumnType::U32 => {
                if let Some(value) = get_column_value::<u32>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::U32(value)));
                }
            }
            ColumnType::U64 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: u64 = value.parse()?;
                    columns_with_value.push((column.column_name, ColumnValue::U64(value)));
                }
            }
            ColumnType::U128 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: u128 = value.parse()?;
                    columns_with_value.push((column.column_name, ColumnValue::U128(value)));
                }
            }
            ColumnType::USize => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: usize = value.parse()?;
                    columns_with_value.push((column.column_name, ColumnValue::USize(value)));
                }
            }
            ColumnType::F32 => {
                if let Some(value) = get_column_value::<f32>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::F32(value)));
                }
            }
            ColumnType::F64 => {
                if let Some(value) = get_column_value::<f64>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::F64(value)));
                }
            }
            ColumnType::String => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    columns_with_value.push((column.column_name, ColumnValue::String(value)));
                }
            }
            ColumnType::Json => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value = serde_json::from_str(&value)?;
                    columns_with_value.push((column.column_name, ColumnValue::Json(value)));
                }
            }
        };
    }
    Document::<T>::create(&columns_with_value)
}

fn rows_to_count(rows: &[sqlx::sqlite::SqliteRow], count_column: &str) -> Result<u64, StorageError> {
    if let Some(row) = rows.get(0) {
        let value = get_column_value::<u32>(row, count_column)?;
        if let Some(value) = value {
            return Ok(value as u64);
        }
    }
    Ok(0)
}

fn get_column_value<'a, T: sqlx::Decode<'a, Sqlite> + sqlx::types::Type<Sqlite>>(
    row: &'a sqlx::sqlite::SqliteRow,
    column: &str,
) -> Result<Option<T>, StorageError> {
    row.try_get::<'a, Option<T>, &str>(column)
        .map_err(|e| corrupted_data_error(column, e))
}

fn corrupted_data_error(column: &str, error: sqlx::error::Error) -> StorageError {
    StorageError::CorruptedDataError(format!("failed to parse column {} value: {}", column, error))
}
