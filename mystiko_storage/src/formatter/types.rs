use crate::document::{Document, DocumentData};
use crate::migration::types::Migration;
use mystiko_protos::storage::v1::{ColumnValue, QueryFilter};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub statement: String,
    pub column_values: Vec<ColumnValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountStatement {
    pub count_column: String,
    pub statement: Statement,
}

pub trait StatementFormatter: Send + Sync + Debug {
    fn format_insert<T: DocumentData>(&self, doc: &Document<T>) -> Statement;
    fn format_update<T: DocumentData>(&self, doc: &Document<T>) -> Statement;
    fn format_delete<T: DocumentData>(&self, doc: &Document<T>) -> Statement;
    fn format_delete_by_filter<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> Statement;
    fn format_count<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> CountStatement;
    fn format_find<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> Statement;
    fn format_migration<T: DocumentData>(&self, migration: &Migration) -> Vec<Statement>;
    fn format_insert_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> Vec<Statement> {
        docs.iter().map(|doc| self.format_insert(doc)).collect::<Vec<_>>()
    }
    fn format_update_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> Vec<Statement> {
        docs.iter().map(|doc| self.format_update(doc)).collect::<Vec<_>>()
    }
    fn format_delete_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> Vec<Statement> {
        docs.iter().map(|doc| self.format_delete(doc)).collect::<Vec<_>>()
    }
    fn format_migration_batch<T: DocumentData>(&self, migrations: &[Migration]) -> Vec<Statement> {
        migrations
            .iter()
            .flat_map(|migration| self.format_migration::<T>(migration))
            .collect::<Vec<_>>()
    }
}

impl Statement {
    pub fn new(statement: String, column_values: Vec<ColumnValue>) -> Self {
        Self {
            statement,
            column_values,
        }
    }
}

impl CountStatement {
    pub fn new(count_column: String, statement: Statement) -> Self {
        Self {
            count_column,
            statement,
        }
    }
}
