use crate::column::{Column, ColumnType, ColumnValue};
use crate::document::{find_required_column_value, DocumentData};
use crate::error::StorageError;

type Result<T> = anyhow::Result<T, StorageError>;

static MIGRATION_SCHEMA: &[&str; 1] = &["CREATE TABLE __migrations__ (\
    id VARCHAR(64) NOT NULL PRIMARY KEY,\
    created_at int NOT NULL,\
    updated_at int NOT NULL,\
    collection_name VARCHAR(255) NOT NULL UNIQUE, \
    version int NOT NULL)"];

#[derive(Debug, Clone, PartialEq)]
pub struct Migration {
    pub collection_name: String,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MigrationColumn {
    CollectionName,
    Version,
}

impl ToString for MigrationColumn {
    fn to_string(&self) -> String {
        match self {
            MigrationColumn::CollectionName => "collection_name".into(),
            MigrationColumn::Version => "version".into(),
        }
    }
}

impl DocumentData for Migration {
    fn migrations() -> &'static [&'static str] {
        MIGRATION_SCHEMA
    }

    fn create(column_values: &[(Column, ColumnValue)]) -> Result<Self> {
        Ok(Self {
            collection_name: find_required_column_value(&MigrationColumn::CollectionName.to_string(), column_values)?
                .as_string()?,
            version: find_required_column_value(&MigrationColumn::Version.to_string(), column_values)?.as_usize()?,
        })
    }

    fn collection_name() -> String {
        String::from("__migrations__")
    }

    fn columns() -> Vec<Column> {
        vec![
            Column::new(MigrationColumn::CollectionName.to_string(), ColumnType::String, false),
            Column::new(MigrationColumn::Version.to_string(), ColumnType::USize, false),
        ]
    }

    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)> {
        vec![
            (
                Column::new(MigrationColumn::CollectionName.to_string(), ColumnType::String, false),
                Some(self.collection_name.clone().into()),
            ),
            (
                Column::new(MigrationColumn::Version.to_string(), ColumnType::USize, false),
                Some(self.version.into()),
            ),
        ]
    }
}
