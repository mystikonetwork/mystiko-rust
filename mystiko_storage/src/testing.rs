use crate::document::{DocumentData, DocumentRawData, DocumentSchema};

use crate::error::StorageError;
use anyhow::Result;

pub static TEST_DOCUMENT_SCHEMA: DocumentSchema = DocumentSchema {
    collection_name: "test_documents",
    migrations: &[
        "CREATE TABLE `test_documents` (\
            `id` VARCHAR(64) NOT NULL PRIMARY KEY,\
            `created_at` INT NOT NULL,\
            `updated_at` INT NOT NULL,\
            `field1` VARCHAR(255) NOT NULL,\
            `field2` INT NOT NULL,\
            `field3` REAL)",
        "ALTER TABLE `test_documents` RENAME COLUMN `field1` TO `f1`",
        "ALTER TABLE `test_documents` RENAME COLUMN `f1` TO `field1`",
    ],
    field_names: &["field1", "field2", "field3"],
};

#[derive(Clone, PartialEq, Debug)]
pub struct TestDocumentData {
    pub field1: String,
    pub field2: u32,
    pub field3: Option<f64>,
}

impl DocumentData for TestDocumentData {
    fn schema() -> &'static DocumentSchema {
        &TEST_DOCUMENT_SCHEMA
    }
    fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            "field1" => Some(self.field1.clone()),
            "field2" => Some(self.field2.to_string()),
            "field3" => self.field3.map(|f| f.to_string()),
            _ => None,
        }
    }
    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, StorageError> {
        Ok(TestDocumentData {
            field1: raw.required_field_string_value("field1")?,
            field2: raw.required_field_integer_value("field2")?,
            field3: raw.field_float_value("field3")?,
        })
    }
}
