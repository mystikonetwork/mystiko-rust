#![forbid(unsafe_code)]
use crate::document::{DocumentData, DocumentRawData, DocumentSchema};

use crate::error::StorageError;
use anyhow::Result;

static MIGRATION_COLLECTION_NAME: &str = "__migrations__";
static MIGRATION_SQL: &[&str; 1] = &["CREATE TABLE __migrations__ (\
    id VARCHAR(64) NOT NULL PRIMARY KEY,\
    created_at int NOT NULL,\
    updated_at int NOT NULL,\
    collection_name VARCHAR(255) NOT NULL UNIQUE, \
    version int NOT NULL)"];
static MIGRATION_FIELDS: &[&str; 2] = &["collection_name", "version"];
pub static MIGRATION_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: MIGRATION_COLLECTION_NAME,
    migrations: MIGRATION_SQL,
    field_names: MIGRATION_FIELDS,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Migration {
    pub collection_name: String,
    pub version: usize,
}

impl DocumentData for Migration {
    fn schema() -> &'static DocumentSchema {
        &MIGRATION_SCHEMA
    }
    fn field_value_string(&self, field: &str) -> Option<String> {
        if field.eq(MIGRATION_FIELDS[0]) {
            Some(self.collection_name.clone())
        } else if field.eq(MIGRATION_FIELDS[1]) {
            Some(self.version.to_string())
        } else {
            None
        }
    }
    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(Migration {
            collection_name: raw.required_field_string_value(MIGRATION_FIELDS[0])?,
            version: raw.required_field_integer_value(MIGRATION_FIELDS[1])?,
        })
    }
}
