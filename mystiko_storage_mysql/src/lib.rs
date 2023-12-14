use anyhow::Result;
use async_trait::async_trait;
use mystiko_protos::storage::v1::column_value::Value;
use mystiko_protos::storage::v1::{ColumnType, ColumnValue};
use mystiko_storage::{
    comparable_string_to_i128, comparable_string_to_u128, i128_to_comparable_string, u128_to_comparable_string,
    CountStatement, Document, DocumentData, Statement, Storage, StorageError,
};
use mystiko_utils::convert::{biguint_to_bytes, i128_to_bytes, u128_to_bytes};
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::str::FromStr;
use std::time::Duration;
use typed_builder::TypedBuilder;

pub const DEFAULT_DATABASE_NAME: &str = "mystiko_database";
pub const DEFAULT_HOST: &str = "localhost";
pub const DEFAULT_PORT: u16 = 3306;
pub const DEFAULT_USERNAME: &str = "root";
pub const DEFAULT_MAX_CONNECTIONS: u32 = 10;
pub const DEFAULT_MIN_CONNECTIONS: u32 = 1;

#[derive(Debug, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct MySqlStorage {
    pool: MySqlPool,
    database: String,
}

#[derive(Debug, Clone, TypedBuilder, Serialize, Deserialize)]
#[builder(field_defaults(setter(into)))]
pub struct MySqlStorageOptions {
    #[serde(default = "default_database_name")]
    #[builder(default = default_database_name())]
    pub database: String,
    #[serde(default = "default_host")]
    #[builder(default = default_host())]
    pub host: String,
    #[serde(default = "default_port")]
    #[builder(default = default_port())]
    pub port: u16,
    #[serde(default = "default_username")]
    #[builder(default = default_username())]
    pub username: String,
    #[serde(default)]
    #[builder(default)]
    pub password: Option<String>,
    #[serde(default = "default_max_connections")]
    #[builder(default = default_max_connections())]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    #[builder(default = default_min_connections())]
    pub min_connections: u32,
    #[serde(default)]
    #[builder(default)]
    pub idle_timeout_ms: Option<u64>,
    #[serde(default)]
    #[builder(default)]
    pub max_lifetime_ms: Option<u64>,
}

impl Default for MySqlStorageOptions {
    fn default() -> Self {
        Self {
            database: default_database_name(),
            host: default_host(),
            port: default_port(),
            username: default_username(),
            password: Default::default(),
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            idle_timeout_ms: Default::default(),
            max_lifetime_ms: Default::default(),
        }
    }
}

fn default_database_name() -> String {
    DEFAULT_DATABASE_NAME.to_string()
}

fn default_host() -> String {
    DEFAULT_HOST.to_string()
}

fn default_port() -> u16 {
    DEFAULT_PORT
}

fn default_username() -> String {
    DEFAULT_USERNAME.to_string()
}

fn default_max_connections() -> u32 {
    DEFAULT_MAX_CONNECTIONS
}

fn default_min_connections() -> u32 {
    DEFAULT_MIN_CONNECTIONS
}

impl MySqlStorage {
    pub async fn connect(options: MySqlStorageOptions) -> Result<Self> {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            options.username,
            options.password.unwrap_or_default(),
            options.host,
            options.port,
            options.database
        );
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(options.max_connections)
            .min_connections(options.min_connections)
            .idle_timeout(options.idle_timeout_ms.map(Duration::from_millis))
            .max_lifetime(options.max_lifetime_ms.map(Duration::from_millis))
            .connect(&url)
            .await?;
        Ok(Self {
            pool,
            database: options.database,
        })
    }
}

#[async_trait]
impl Storage for MySqlStorage {
    async fn execute(&self, statement: Statement) -> Result<(), StorageError> {
        statement_to_query(&statement)?
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn execute_batch(&self, statements: Vec<Statement>) -> Result<(), StorageError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| StorageError::DatabaseError(e.into()))?;
        for statement in statements.iter() {
            statement_to_query(statement)?
                .execute(&mut *tx)
                .await
                .map_err(|e| StorageError::DatabaseError(e.into()))?;
        }
        tx.commit().await.map_err(|e| StorageError::DatabaseError(e.into()))?;
        Ok(())
    }

    async fn query<T: DocumentData>(&self, statement: Statement) -> Result<Vec<Document<T>>, StorageError> {
        let rows = statement_to_query(&statement)?
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::DatabaseError(e.into()))?;
        Ok(rows_to_documents(&rows)?)
    }

    async fn count(&self, statement: CountStatement) -> Result<u64, StorageError> {
        let rows = statement_to_query(&statement.statement)?
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::DatabaseError(e.into()))?;
        Ok(rows_to_count(&rows, &statement.count_column)?)
    }

    async fn collection_exists(&self, collection: &str) -> Result<bool, StorageError> {
        let query = sqlx::query(
            "SELECT * FROM \
            `information_schema`.`tables` \
            WHERE `table_schema` = ? AND `table_name` = ? \
            LIMIT 1",
        )
        .bind(&self.database)
        .bind(collection);
        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::DatabaseError(e.into()))?;
        Ok(!rows.is_empty())
    }
}

type Query<'a> = sqlx::query::Query<'a, sqlx::MySql, <sqlx::MySql as sqlx::database::HasArguments<'a>>::Arguments>;

fn statement_to_query(statement: &Statement) -> Result<Query<'_>, StorageError> {
    let mut query = sqlx::query::<sqlx::MySql>(&statement.statement);
    query = bind_mysql_query(query, statement.column_values.iter().collect::<Vec<_>>())?;
    Ok(query)
}

fn bind_mysql_query<'a>(mut query: Query<'a>, values: Vec<&ColumnValue>) -> Result<Query<'a>, StorageError> {
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
                query = query.bind(value.as_u64()?);
            }
            ColumnType::U128 => {
                query = query.bind(u128_to_comparable_string(value.as_u128()?));
            }
            ColumnType::USize => {
                query = query.bind(value.as_usize()? as u64);
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

fn rows_to_documents<T: DocumentData>(rows: &[sqlx::mysql::MySqlRow]) -> Result<Vec<Document<T>>, StorageError> {
    let mut documents: Vec<_> = vec![];
    for row in rows.iter() {
        documents.push(row_to_document::<T>(row)?);
    }
    Ok(documents)
}

fn row_to_document<T: DocumentData>(row: &sqlx::mysql::MySqlRow) -> Result<Document<T>, StorageError> {
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
                if let Some(value) = get_column_value::<u64>(row, &column.column_name)? {
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
                if let Some(value) = get_column_value::<u64>(row, &column.column_name)? {
                    columns_with_value.push((
                        column.column_name,
                        ColumnValue::builder().value(Value::UsizeValue(value)).build(),
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
                    ))
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
fn rows_to_count(rows: &[sqlx::mysql::MySqlRow], count_column: &str) -> Result<u64, StorageError> {
    if let Some(row) = rows.get(0) {
        let value = get_column_value::<i64>(row, count_column)?;
        if let Some(value) = value {
            return Ok(value as u64);
        }
    }
    Ok(0)
}

fn get_column_value<'a, T: sqlx::Decode<'a, sqlx::MySql> + sqlx::types::Type<sqlx::MySql>>(
    row: &'a sqlx::mysql::MySqlRow,
    column: &str,
) -> Result<Option<T>, StorageError> {
    row.try_get::<'a, Option<T>, &str>(column)
        .map_err(|e| corrupted_data_error(column, e))
}

fn corrupted_data_error(column: &str, error: sqlx::error::Error) -> StorageError {
    StorageError::CorruptedDataError(format!("failed to parse column {} value: {}", column, error))
}
