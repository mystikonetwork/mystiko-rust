use crate::column::{Column, ColumnType, ColumnValue, IndexColumns, UniqueColumns};
use crate::error::StorageError;
use crate::migration::types::{CreateCollectionMigration, Migration};
use std::fmt::Debug;

type Result<T> = anyhow::Result<T, StorageError>;

pub trait DocumentData: Send + Sync + Clone + Debug + PartialEq {
    fn create(column_values: &[(String, ColumnValue)]) -> Result<Self>;
    fn collection_name() -> &'static str;
    fn columns() -> Vec<Column>;
    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)>;
    fn unique_columns() -> Vec<UniqueColumns> {
        vec![]
    }
    fn index_columns() -> Vec<IndexColumns> {
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

impl AsRef<str> for DocumentColumn {
    fn as_ref(&self) -> &str {
        match self {
            DocumentColumn::Id => "id",
            DocumentColumn::CreatedAt => "created_at",
            DocumentColumn::UpdatedAt => "updated_at",
        }
    }
}

impl ToString for DocumentColumn {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl From<DocumentColumn> for String {
    fn from(value: DocumentColumn) -> Self {
        value.to_string()
    }
}

impl<T: DocumentData> DocumentData for Document<T> {
    fn create(column_values: &[(String, ColumnValue)]) -> Result<Self> {
        Ok(Self {
            id: find_required_column_value(&DocumentColumn::Id, column_values)?.as_string()?,
            created_at: find_required_column_value(&DocumentColumn::CreatedAt, column_values)?.as_u64()?,
            updated_at: find_required_column_value(&DocumentColumn::UpdatedAt, column_values)?.as_u64()?,
            data: T::create(column_values)?,
        })
    }

    fn collection_name() -> &'static str {
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

    fn unique_columns() -> Vec<UniqueColumns> {
        T::unique_columns()
    }

    fn migrations() -> Vec<Migration> {
        let mut migrations: Vec<Migration> = vec![Migration::CreateCollection(
            CreateCollectionMigration::builder()
                .collection_name(Self::collection_name())
                .columns(Self::columns())
                .unique_columns(Self::unique_columns())
                .index_columns(Self::index_columns())
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
                .column_name(DocumentColumn::Id)
                .column_type(ColumnType::String)
                .is_primary_key(true)
                .length_limit(Some(64))
                .build(),
            Column::builder()
                .column_name(DocumentColumn::CreatedAt)
                .column_type(ColumnType::U64)
                .build(),
            Column::builder()
                .column_name(DocumentColumn::UpdatedAt)
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

    pub fn initial_migrations() -> Vec<Migration> {
        Self::migrations()
            .into_iter()
            .filter(|migration| matches!(migration, Migration::CreateCollection(_)))
            .collect()
    }
}

pub fn find_column_value<S: AsRef<str>>(
    column_name: &S,
    column_values: &[(String, ColumnValue)],
) -> Option<ColumnValue> {
    column_values.iter().find_map(|(column, value)| {
        if column == column_name.as_ref() {
            Some(value.clone())
        } else {
            None
        }
    })
}

pub fn find_required_column_value<S: AsRef<str>>(
    column_name: &S,
    column_values: &[(String, ColumnValue)],
) -> Result<ColumnValue> {
    find_column_value(column_name, column_values)
        .ok_or_else(|| StorageError::MissingRequiredColumnError(column_name.as_ref().to_string()))
}
