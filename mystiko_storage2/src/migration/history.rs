use crate::column::{Column, ColumnType, ColumnValue};
use crate::document::{find_required_column_value, DocumentData};
use crate::error::StorageError;

type Result<T> = anyhow::Result<T, StorageError>;

#[derive(Debug, Clone, PartialEq)]
pub struct MigrationHistory {
    pub collection_name: String,
    pub version: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MigrationHistoryColumn {
    CollectionName,
    Version,
}

impl ToString for MigrationHistoryColumn {
    fn to_string(&self) -> String {
        match self {
            MigrationHistoryColumn::CollectionName => "collection_name".into(),
            MigrationHistoryColumn::Version => "version".into(),
        }
    }
}

impl DocumentData for MigrationHistory {
    fn create(column_values: &[(&str, ColumnValue)]) -> Result<Self> {
        Ok(Self {
            collection_name: find_required_column_value(
                &MigrationHistoryColumn::CollectionName.to_string(),
                column_values,
            )?
            .as_string()?,
            version: find_required_column_value(&MigrationHistoryColumn::Version.to_string(), column_values)?
                .as_usize()?,
        })
    }

    fn collection_name() -> String {
        String::from("__migrations__")
    }

    fn columns() -> Vec<Column> {
        vec![
            Column::builder()
                .column_name(MigrationHistoryColumn::CollectionName.to_string())
                .column_type(ColumnType::String)
                .length_limit(128)
                .build(),
            Column::builder()
                .column_name(MigrationHistoryColumn::Version.to_string())
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
