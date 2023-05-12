use crate::column::{Column, ColumnType, ColumnValue};
use crate::error::StorageError;
use std::fmt::Debug;

type Result<T> = anyhow::Result<T, StorageError>;

pub trait DocumentData: Send + Sync + Clone + Debug + PartialEq {
    fn migrations() -> &'static [&'static str];
    fn create(column_values: &[(Column, ColumnValue)]) -> Result<Self>;
    fn collection_name() -> String;
    fn columns() -> Vec<Column>;
    fn column_values(&self) -> Vec<(Column, Option<ColumnValue>)>;
    fn version() -> usize {
        Self::migrations().len()
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
    fn migrations() -> &'static [&'static str] {
        T::migrations()
    }

    fn create(column_values: &[(Column, ColumnValue)]) -> Result<Self> {
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
            Column::new(DocumentColumn::Id.to_string(), ColumnType::String, false),
            Column::new(DocumentColumn::CreatedAt.to_string(), ColumnType::U64, false),
            Column::new(DocumentColumn::UpdatedAt.to_string(), ColumnType::U64, false),
        ]
    }

    pub fn meta_column_values(&self) -> Vec<(Column, Option<ColumnValue>)> {
        vec![
            (
                Column::new(DocumentColumn::Id.to_string(), ColumnType::String, false),
                Some(self.id.clone().into()),
            ),
            (
                Column::new(DocumentColumn::CreatedAt.to_string(), ColumnType::U64, false),
                Some(self.created_at.into()),
            ),
            (
                Column::new(DocumentColumn::UpdatedAt.to_string(), ColumnType::U64, false),
                Some(self.updated_at.into()),
            ),
        ]
    }
}

pub fn find_column_value(column_name: &str, column_values: &[(Column, ColumnValue)]) -> Option<ColumnValue> {
    column_values.iter().find_map(|(column, value)| {
        if column.column_name == column_name {
            Some(value.clone())
        } else {
            None
        }
    })
}

pub fn find_required_column_value(column_name: &str, column_values: &[(Column, ColumnValue)]) -> Result<ColumnValue> {
    find_column_value(column_name, column_values)
        .ok_or_else(|| StorageError::MissingRequiredColumnError(column_name.into()))
}
