use mystiko_protos::storage::v1::{ColumnType, ColumnValue};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Column {
    pub column_type: ColumnType,
    #[builder(setter(into))]
    pub column_name: String,
    #[builder(default = false)]
    pub nullable: bool,
    #[builder(default = false)]
    pub is_primary_key: bool,
    #[builder(default)]
    pub length_limit: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct UniqueColumns {
    #[builder(default, setter(into, strip_option))]
    pub unique_name: Option<String>,
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct IndexColumns {
    #[builder(default, setter(into, strip_option))]
    pub index_name: Option<String>,
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, TypedBuilder)]
pub struct ColumnValues {
    pub(crate) column_values: Vec<(String, Option<ColumnValue>)>,
}

impl<T: ToString, I: IntoIterator<Item = T>> From<I> for UniqueColumns {
    fn from(value: I) -> Self {
        UniqueColumns::builder()
            .column_names(value.into_iter().map(|v| v.to_string()).collect())
            .build()
    }
}

impl<T: ToString, I: IntoIterator<Item = T>> From<I> for IndexColumns {
    fn from(value: I) -> Self {
        IndexColumns::builder()
            .column_names(value.into_iter().map(|v| v.to_string()).collect())
            .build()
    }
}

impl ColumnValues {
    pub fn new() -> Self {
        ColumnValues {
            column_values: Default::default(),
        }
    }

    pub fn set_value<C, V>(mut self, column: C, value: V) -> Self
    where
        C: ToString,
        V: Into<ColumnValue>,
    {
        self.column_values.push((column.to_string(), Some(value.into())));
        self
    }

    pub fn set_null_value<C>(mut self, column: C) -> Self
    where
        C: ToString,
    {
        self.column_values.push((column.to_string(), None));
        self
    }
}

impl<C, V> From<Vec<(C, V)>> for ColumnValues
where
    C: ToString,
    V: Into<ColumnValue>,
{
    fn from(values: Vec<(C, V)>) -> Self {
        let mut column_values = Self::new();
        for (column, value) in values {
            column_values = column_values.set_value(column, value);
        }
        column_values
    }
}

impl<C, V> From<(C, V)> for ColumnValues
where
    C: ToString,
    V: Into<ColumnValue>,
{
    fn from(value: (C, V)) -> Self {
        vec![value].into()
    }
}
