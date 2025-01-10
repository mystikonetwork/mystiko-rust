use anyhow::Result;
use async_trait::async_trait;
use mystiko_protos::storage::v1::column_value::Value;
use mystiko_protos::storage::v1::{ColumnType, ColumnValue};
use mystiko_storage::{
    comparable_string_to_i128, comparable_string_to_u128, comparable_string_to_u64, comparable_string_to_usize,
    i128_to_comparable_string, u128_to_comparable_string, u64_to_comparable_string, usize_to_comparable_string,
    CountStatement, Document, DocumentData, Statement, Storage, StorageError,
};
use mystiko_utils::convert::{biguint_to_bytes, i128_to_bytes, u128_to_bytes};
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous};
use sqlx::{ConnectOptions, Row, Sqlite};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use typed_builder::TypedBuilder;

static SQLITE_MEMORY_PATH: &str = ":memory:";
static DEFAULT_MAX_CONNECTION: u32 = 1;
static DEFAULT_EXECUTE_BATCH_SIZE: usize = 1000;

type Query<'a> = sqlx::query::Query<'a, Sqlite, <Sqlite as sqlx::database::HasArguments<'a>>::Arguments>;

#[derive(Debug)]
enum SqliteConnection {
    Single(Arc<Mutex<sqlx::SqliteConnection>>),
    Pool(sqlx::SqlitePool),
}

#[derive(Debug)]
pub struct SqliteStorage {
    connection: SqliteConnection,
    execute_batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct SqliteStorageOptions {
    pub path: Option<String>,
    pub max_connections: Option<u32>,
    pub execute_batch_size: Option<usize>,
    pub journal_mode: Option<String>,
    pub locking_mode: Option<String>,
    pub synchronous: Option<String>,
    pub read_only: bool,
    pub busy_timeout_ms: Option<u64>,
    pub page_size: Option<u32>,
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn execute(&self, statement: Statement) -> Result<(), StorageError> {
        self.execute_query(statement_to_query(&statement)?).await
    }

    async fn execute_batch(&self, statements: Vec<Statement>) -> Result<(), StorageError> {
        for statements_chunk in statements.chunks(self.execute_batch_size) {
            let statement = Statement {
                statement: statements_chunk
                    .iter()
                    .map(|s| s.statement.clone())
                    .collect::<Vec<String>>()
                    .join(";"),
                column_values: statements_chunk
                    .iter()
                    .flat_map(|s| s.column_values.clone())
                    .collect::<Vec<_>>(),
            };
            self.execute_query(statement_to_query(&statement)?).await?;
        }
        Ok(())
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
    async fn execute_query(&self, query: Query<'_>) -> Result<(), StorageError> {
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

    async fn execute_fetch(&self, query: Query<'_>) -> Result<Vec<sqlx::sqlite::SqliteRow>, StorageError> {
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

impl SqliteStorage {
    pub async fn new<O>(options: O) -> Result<Self, StorageError>
    where
        O: Into<SqliteStorageOptions>,
    {
        let options = options.into();
        let in_memory = options.path.is_none();
        let default_journal_mode = if in_memory {
            SqliteJournalMode::Memory
        } else {
            SqliteJournalMode::Wal
        };
        let journal_mode = options
            .journal_mode
            .map(|mode| SqliteJournalMode::from_str(&mode))
            .transpose()
            .map_err(|e| StorageError::DatabaseError(e.into()))?
            .unwrap_or(default_journal_mode);
        let synchronous = options
            .synchronous
            .map(|mode| SqliteSynchronous::from_str(&mode))
            .transpose()
            .map_err(|e| StorageError::DatabaseError(e.into()))?
            .unwrap_or(SqliteSynchronous::Normal);
        let locking_mode = options
            .locking_mode
            .map(|mode| SqliteLockingMode::from_str(&mode))
            .transpose()
            .map_err(|e| StorageError::DatabaseError(e.into()))?
            .unwrap_or(SqliteLockingMode::Normal);
        let mut connect_options = SqliteConnectOptions::new()
            .filename(options.path.unwrap_or(SQLITE_MEMORY_PATH.to_string()))
            .journal_mode(journal_mode)
            .synchronous(synchronous)
            .locking_mode(locking_mode)
            .read_only(options.read_only)
            .create_if_missing(true);
        if let Some(busy_timeout_ms) = options.busy_timeout_ms {
            connect_options = connect_options.busy_timeout(Duration::from_millis(busy_timeout_ms));
        }
        if let Some(page_size) = options.page_size {
            connect_options = connect_options.page_size(page_size);
        }
        let connection = if in_memory {
            SqliteConnection::Single(Arc::new(Mutex::new(
                connect_options
                    .connect()
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?,
            )))
        } else {
            SqliteConnection::Pool(
                sqlx::sqlite::SqlitePoolOptions::new()
                    .max_connections(options.max_connections.unwrap_or(DEFAULT_MAX_CONNECTION))
                    .connect_with(connect_options)
                    .await
                    .map_err(|e| StorageError::DatabaseError(e.into()))?,
            )
        };
        Ok(SqliteStorage {
            connection,
            execute_batch_size: options.execute_batch_size.unwrap_or(DEFAULT_EXECUTE_BATCH_SIZE),
        })
    }

    pub async fn from_memory() -> Result<Self, StorageError> {
        SqliteStorage::new(SqliteStorageOptions::default()).await
    }

    pub async fn from_path<S>(path: S) -> Result<Self, StorageError>
    where
        S: AsRef<str>,
    {
        SqliteStorage::new(SqliteStorageOptions::builder().path(path.as_ref().to_string()).build()).await
    }
}

impl Default for SqliteStorageOptions {
    fn default() -> Self {
        Self::builder().build()
    }
}

fn statement_to_query(statement: &Statement) -> Result<Query<'_>, StorageError> {
    let mut query = sqlx::query::<Sqlite>(&statement.statement);
    query = bind_query(query, statement.column_values.iter().collect::<Vec<_>>())?;
    Ok(query)
}

fn bind_query<'a>(mut query: Query<'a>, values: Vec<&ColumnValue>) -> Result<Query<'a>, StorageError> {
    for value in values {
        let column_type = value.column_type()?;
        match column_type {
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
                query = query.bind(i128_to_comparable_string(value.as_i128()?));
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
                query = query.bind(u64_to_comparable_string(value.as_u64()?));
            }
            ColumnType::U128 => {
                query = query.bind(u128_to_comparable_string(value.as_u128()?));
            }
            ColumnType::USize => {
                query = query.bind(usize_to_comparable_string(value.as_usize()?));
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
            ColumnType::BigInt => {
                query = query.bind(value.as_bigint()?.to_string());
            }
            ColumnType::BigUint => {
                query = query.bind(value.as_biguint()?.to_string());
            }
            ColumnType::Json => {
                query = query.bind(serde_json::to_string(&value.as_json()?)?);
            }
            _ => Err(StorageError::UnsupportedColumnTypeError(column_type.to_string()))?,
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
        let column_type = column.column_type;
        match column_type {
            ColumnType::Bool => {
                if let Some(value) = get_column_value::<bool>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::BoolValue(value)).build(),
                    ));
                }
            }
            ColumnType::Char => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    if let Some(value) = value.chars().next() {
                        columns_with_value.push((
                            column.column_name,
                            ColumnValue::builder()
                                .value(Value::CharValue(value.to_string()))
                                .build(),
                        ));
                    }
                }
            }
            ColumnType::I8 => {
                if let Some(value) = get_column_value::<i8>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::I8Value(value as i32)).build(),
                    ));
                }
            }
            ColumnType::I16 => {
                if let Some(value) = get_column_value::<i16>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::I16Value(value as i32)).build(),
                    ));
                }
            }
            ColumnType::I32 => {
                if let Some(value) = get_column_value::<i32>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::I32Value(value)).build(),
                    ));
                }
            }
            ColumnType::I64 => {
                if let Some(value) = get_column_value::<i64>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::I64Value(value)).build(),
                    ));
                }
            }
            ColumnType::I128 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: i128 = comparable_string_to_i128(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder()
                            .value(Value::I128Value(i128_to_bytes(value)))
                            .build(),
                    ));
                }
            }
            ColumnType::ISize => {
                if let Some(value) = get_column_value::<i64>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::IsizeValue(value)).build(),
                    ));
                }
            }
            ColumnType::U8 => {
                if let Some(value) = get_column_value::<u8>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::U8Value(value as u32)).build(),
                    ));
                }
            }
            ColumnType::U16 => {
                if let Some(value) = get_column_value::<u16>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::U16Value(value as u32)).build(),
                    ));
                }
            }
            ColumnType::U32 => {
                if let Some(value) = get_column_value::<u32>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::U32Value(value)).build(),
                    ));
                }
            }
            ColumnType::U64 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: u64 = comparable_string_to_u64(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::U64Value(value)).build(),
                    ));
                }
            }
            ColumnType::U128 => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: u128 = comparable_string_to_u128(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder()
                            .value(Value::U128Value(u128_to_bytes(value)))
                            .build(),
                    ));
                }
            }
            ColumnType::USize => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value: usize = comparable_string_to_usize(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::UsizeValue(value as u64)).build(),
                    ));
                }
            }
            ColumnType::F32 => {
                if let Some(value) = get_column_value::<f32>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::F32Value(value)).build(),
                    ));
                }
            }
            ColumnType::F64 => {
                if let Some(value) = get_column_value::<f64>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::F64Value(value)).build(),
                    ));
                }
            }
            ColumnType::String => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::StringValue(value)).build(),
                    ));
                }
            }
            ColumnType::BigInt => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value = BigInt::from_str(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::BigIntValue(value.into())).build(),
                    ));
                }
            }
            ColumnType::BigUint => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    let value = BigUint::from_str(&value)?;
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder()
                            .value(Value::BigUintValue(biguint_to_bytes(&value)))
                            .build(),
                    ))
                }
            }
            ColumnType::Json => {
                if let Some(value) = get_column_value::<String>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::JsonValue(value)).build(),
                    ));
                }
            }
            _ => Err(StorageError::UnsupportedColumnTypeError(column_type.to_string()))?,
        };
    }
    Document::<T>::create(&columns_with_value)
}

fn rows_to_count(rows: &[sqlx::sqlite::SqliteRow], count_column: &str) -> Result<u64, StorageError> {
    if let Some(row) = rows.first() {
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
