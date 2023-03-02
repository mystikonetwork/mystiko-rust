#![forbid(unsafe_code)]
use crate::document::{DocumentData, DocumentRawData, DocumentSchema};
use std::io::Error;

static MIGRATION_COLLECTION_NAME: &'static str = "__migrations__";
static MIGRATION_SQL: &'static [&'static str; 1] = &["CREATE TABLE __migrations__ (\
    id VARCHAR(64) NOT NULL PRIMARY KEY,\
    created_at int NOT NULL,\
    updated_at int NOT NULL,\
    collection_name VARCHAR(255) NOT NULL UNIQUE, \
    version int NOT NULL)"];
static MIGRATION_FIELDS: &'static [&'static str; 2] = &["collection_name", "version"];
pub static MIGRATION_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: MIGRATION_COLLECTION_NAME,
    migrations: MIGRATION_SQL,
    field_names: MIGRATION_FIELDS,
};

#[derive(Clone)]
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
    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Migration {
            collection_name: raw.field_string_value(MIGRATION_FIELDS[0])?.unwrap(),
            version: raw.field_integer_value(MIGRATION_FIELDS[1])?.unwrap(),
        })
    }
}
