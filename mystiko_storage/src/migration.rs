use crate::document::{DocumentData, DocumentSchema};
use std::collections::HashMap;

static MIGRATION_COLLECTION_NAME: &'static str = "__migrations__";
static MIGRATION_SQL: &'static [&'static str; 1] = &["CREATE TABLE {} (\
    id VARCHAR(64) NOT NULL PRIMARY KEY,\
    created_at int NOT NULL,
    updated_at int NOT NULL,
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

impl From<HashMap<String, String>> for Migration {
    fn from(value: HashMap<String, String>) -> Self {
        Migration {
            collection_name: value.get(MIGRATION_FIELDS[0]).unwrap().clone(),
            version: value.get(MIGRATION_FIELDS[1]).unwrap().parse().unwrap(),
        }
    }
}

impl DocumentData for Migration {
    fn schema(&self) -> &'static DocumentSchema {
        &MIGRATION_SCHEMA
    }

    fn to_map(&self) -> HashMap<String, String> {
        HashMap::from([
            (
                MIGRATION_FIELDS[0].to_string(),
                self.collection_name.clone(),
            ),
            (MIGRATION_FIELDS[1].to_string(), self.version.to_string()),
        ])
    }
}
