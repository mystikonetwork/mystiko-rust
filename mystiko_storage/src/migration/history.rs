use crate::{find_required_column_value, Column, DocumentData, StorageError};
use mystiko_protos::storage::v1::{ColumnType, ColumnValue};
use serde::{Deserialize, Serialize};

type Result<T> = anyhow::Result<T, StorageError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationHistory {
    pub collection_name: String,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MigrationHistoryColumn {
    CollectionName,
    Version,
}

impl AsRef<str> for MigrationHistoryColumn {
    fn as_ref(&self) -> &str {
        match self {
            MigrationHistoryColumn::CollectionName => "collection_name",
            MigrationHistoryColumn::Version => "version",
        }
    }
}

impl std::fmt::Display for MigrationHistoryColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl From<MigrationHistoryColumn> for String {
    fn from(value: MigrationHistoryColumn) -> Self {
        value.to_string()
    }
}

impl DocumentData for MigrationHistory {
    fn create(column_values: &[(String, ColumnValue)]) -> Result<Self> {
        Ok(Self {
            collection_name: find_required_column_value(&MigrationHistoryColumn::CollectionName, column_values)?
                .as_string()?,
            version: find_required_column_value(&MigrationHistoryColumn::Version, column_values)?.as_usize()?,
        })
    }

    fn collection_name() -> &'static str {
        "__migrations__"
    }

    fn columns() -> Vec<Column> {
        vec![
            Column::builder()
                .column_name(MigrationHistoryColumn::CollectionName)
                .column_type(ColumnType::String)
                .length_limit(Some(128))
                .build(),
            Column::builder()
                .column_name(MigrationHistoryColumn::Version)
                .column_type(ColumnType::USize)
                .build(),
        ]
    }

    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)> {
        Self::columns()
            .into_iter()
            .zip(vec![
                Some(self.collection_name.clone().into()),
                Some(self.version.into()),
            ])
            .collect()
    }
}
