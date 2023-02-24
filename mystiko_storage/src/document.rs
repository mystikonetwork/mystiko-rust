use std::collections::HashMap;
use std::convert::From;
use std::marker::{Send, Sync};

pub struct DocumentSchema {
    pub collection_name: &'static str,
    pub migrations: &'static [&'static str],
    pub field_names: &'static [&'static str],
}

impl DocumentSchema {
    fn version(&self) -> usize {
        self.migrations.len()
    }
}

pub trait DocumentData: From<HashMap<String, String>> + Clone + Send + Sync {
    fn schema(&self) -> &'static DocumentSchema;
    fn to_map(&self) -> HashMap<String, String>;
}

#[derive(Clone)]
pub struct Document<T: DocumentData> {
    pub id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub data: T,
}

pub const DOCUMENT_ID_FIELD: &str = "id";
pub const DOCUMENT_CREATED_AT_FIELD: &str = "created_at";
pub const DOCUMENT_UPDATED_AT_FIELD: &str = "updated_at";

impl<T: DocumentData> Document<T> {
    fn schema(&self) -> &'static DocumentSchema {
        self.data.schema()
    }
    fn to_map(&self) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(DOCUMENT_ID_FIELD.to_string(), self.id.clone());
        map.insert(
            DOCUMENT_CREATED_AT_FIELD.to_string(),
            self.created_at.to_string(),
        );
        map.insert(
            DOCUMENT_UPDATED_AT_FIELD.to_string(),
            self.updated_at.to_string(),
        );
        map.extend(self.data.to_map());
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_definition() {
        static COLLECTION_NAME: &str = "books";
        static MIGRATIONS: [&'static str; 1] = ["CREATE TABLE books"];
        static FIELD_NAMES: [&'static str; 1] = ["title"];
        static SCHEMA: DocumentSchema = DocumentSchema {
            collection_name: COLLECTION_NAME,
            migrations: &MIGRATIONS,
            field_names: &FIELD_NAMES,
        };
        #[derive(Clone)]
        struct Book {
            title: String,
        }
        impl From<HashMap<String, String>> for Book {
            fn from(value: HashMap<String, String>) -> Self {
                Book {
                    title: value.get("title").unwrap().clone(),
                }
            }
        }
        impl DocumentData for Book {
            fn schema(&self) -> &'static DocumentSchema {
                &SCHEMA
            }
            fn to_map(&self) -> HashMap<String, String> {
                HashMap::from([("title".to_string(), self.title.clone())])
            }
        }
        let book: Document<Book> = Document {
            id: "id01".to_string(),
            created_at: 0xdead,
            updated_at: 0xbeef,
            data: Book {
                title: "test book".to_string(),
            },
        };
        assert_eq!(book.id, "id01");
        assert_eq!(book.created_at, 0xdead);
        assert_eq!(book.updated_at, 0xbeef);
        assert_eq!(book.data.title, "test book");
        assert_eq!(book.schema().collection_name, COLLECTION_NAME);
        assert_eq!(book.schema().migrations, MIGRATIONS);
        assert_eq!(book.schema().field_names, FIELD_NAMES);
        assert_eq!(book.schema().version(), 1);
        assert_eq!(
            book.to_map(),
            HashMap::from([
                (DOCUMENT_ID_FIELD.to_string(), "id01".to_string()),
                (DOCUMENT_CREATED_AT_FIELD.to_string(), 0xdead.to_string()),
                (DOCUMENT_UPDATED_AT_FIELD.to_string(), 0xbeef.to_string()),
                ("title".to_string(), "test book".to_string())
            ])
        );
    }
}
