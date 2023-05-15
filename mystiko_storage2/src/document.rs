use crate::column::{Column, ColumnType, ColumnValue};
use crate::error::StorageError;
use crate::migration::types::{CreateCollectionMigration, Migration};
use std::fmt::Debug;

type Result<T> = anyhow::Result<T, StorageError>;

pub trait DocumentData: Send + Sync + Clone + Debug + PartialEq {
    fn create(column_values: &[(&str, ColumnValue)]) -> Result<Self>;
    fn collection_name() -> String;
    fn columns() -> Vec<Column>;
    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)>;
    fn unique_columns() -> Vec<Vec<String>> {
        vec![]
    }
    fn migrations() -> Vec<Migration> {
        vec![]
    }
    fn version() -> usize {
        Self::migrations().len() + 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document<T: DocumentData> {
    pub id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub data: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentColumn {
    Id,
    CreatedAt,
    UpdatedAt,
}

impl ToString for DocumentColumn {
    fn to_string(&self) -> String {
        match self {
            DocumentColumn::Id => "id".into(),
            DocumentColumn::CreatedAt => "created_at".into(),
            DocumentColumn::UpdatedAt => "updated_at".into(),
        }
    }
}

impl<T: DocumentData> DocumentData for Document<T> {
    fn create(column_values: &[(&str, ColumnValue)]) -> Result<Self> {
        Ok(Self {
            id: find_required_column_value(&DocumentColumn::Id.to_string(), column_values)?.as_string()?,
            created_at: find_required_column_value(&DocumentColumn::CreatedAt.to_string(), column_values)?.as_u64()?,
            updated_at: find_required_column_value(&DocumentColumn::UpdatedAt.to_string(), column_values)?.as_u64()?,
            data: T::create(column_values)?,
        })
    }

    fn collection_name() -> String {
        T::collection_name()
    }

    fn columns() -> Vec<Column> {
        let mut names = Self::meta_columns();
        names.extend(T::columns());
        names
    }

    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)> {
        let mut values = self.meta_column_values();
        values.extend(self.data.column_values());
        values
    }

    fn unique_columns() -> Vec<Vec<String>> {
        T::unique_columns()
    }

    fn migrations() -> Vec<Migration> {
        let mut migrations: Vec<Migration> = vec![Migration::CreateCollection(
            CreateCollectionMigration::builder()
                .collection_name(Self::collection_name())
                .columns(Self::columns())
                .unique_columns(Self::unique_columns())
                .build(),
        )];
        migrations.extend(T::migrations());
        migrations
    }
}

impl<T: DocumentData> Document<T> {
    pub fn new(id: String, created_at: u64, updated_at: u64, data: T) -> Self {
        Self {
            id,
            created_at,
            updated_at,
            data,
        }
    }

    pub fn meta_columns() -> Vec<Column> {
        vec![
            Column::builder()
                .column_name(DocumentColumn::Id.to_string())
                .column_type(ColumnType::String)
                .is_primary_key(true)
                .length_limit(Some(64))
                .build(),
            Column::builder()
                .column_name(DocumentColumn::CreatedAt.to_string())
                .column_type(ColumnType::U64)
                .build(),
            Column::builder()
                .column_name(DocumentColumn::UpdatedAt.to_string())
                .column_type(ColumnType::U64)
                .build(),
        ]
    }

    pub fn meta_column_values(&self) -> Vec<(Column, Option<ColumnValue>)> {
        Self::columns()
            .into_iter()
            .zip(vec![
                Some(self.id.clone().into()),
                Some(self.created_at.into()),
                Some(self.updated_at.into()),
            ])
            .collect()
    }
}

pub fn find_column_value(column_name: &str, column_values: &[(&str, ColumnValue)]) -> Option<ColumnValue> {
    column_values.iter().find_map(|(column, value)| {
        if *column == column_name {
            Some(value.clone())
        } else {
            None
        }
    })
}

pub fn find_required_column_value(column_name: &str, column_values: &[(&str, ColumnValue)]) -> Result<ColumnValue> {
    find_column_value(column_name, column_values)
        .ok_or_else(|| StorageError::MissingRequiredColumnError(column_name.into()))
}
