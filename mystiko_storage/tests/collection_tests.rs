use async_trait::async_trait;
use mystiko_storage::collection::*;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData, DOCUMENT_ID_FIELD};
use mystiko_storage::filter::{Condition, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::{SqlFormatter, StatementFormatter};
use mystiko_storage::migration::{Migration, MIGRATION_SCHEMA};
use mystiko_storage::storage::Storage;
use mystiko_storage::testing::TestDocumentData;
use num_traits::{Float, PrimInt};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use tokio_test::block_on;

#[derive(Clone)]
struct TestDocumentRawData {
    data: HashMap<String, String>,
}

impl DocumentRawData for TestDocumentRawData {
    fn field_integer_value<T: PrimInt + FromStr>(&self, field: &str) -> Result<Option<T>, Error> {
        let value = self.data.get(field);
        if value.is_some() {
            match value.unwrap().parse() {
                Ok(parsed) => Ok(Some(parsed)),
                Err(_) => Err(Error::new(ErrorKind::Other, "parsed error")),
            }
        } else {
            Ok(None)
        }
    }

    fn field_float_value<T: Float + FromStr>(&self, field: &str) -> Result<Option<T>, Error> {
        let value = self.data.get(field);
        if value.is_some() {
            match value.unwrap().parse() {
                Ok(parsed) => Ok(Some(parsed)),
                Err(_) => Err(Error::new(ErrorKind::Other, "parsed error")),
            }
        } else {
            Ok(None)
        }
    }

    fn field_string_value(&self, field: &str) -> Result<Option<String>, Error> {
        Ok(self.data.get(field).and_then(|v| Some(v.clone())))
    }
}

struct TestStorage {
    collection_exists: bool,
    raise_error_on_collection_exists: bool,
    raise_error_on_query: bool,
    raise_error_on_execute: bool,
    expected_data: Vec<TestDocumentRawData>,
    statements: Vec<String>,
}

impl TestStorage {
    pub fn new() -> Self {
        TestStorage {
            collection_exists: false,
            raise_error_on_collection_exists: false,
            raise_error_on_query: false,
            raise_error_on_execute: false,
            expected_data: vec![],
            statements: vec![],
        }
    }
}

#[async_trait]
impl Storage<TestDocumentRawData> for TestStorage {
    async fn execute(&mut self, statement: String) -> Result<(), Error> {
        self.statements.push(statement);
        if self.raise_error_on_execute {
            Err(Error::new(ErrorKind::Other, "expected error"))
        } else {
            Ok(())
        }
    }

    async fn query(&mut self, statement: String) -> Result<Vec<TestDocumentRawData>, Error> {
        self.statements.push(statement);
        if self.raise_error_on_query {
            Err(Error::new(ErrorKind::Other, "expected error"))
        } else {
            Ok(self.expected_data.clone())
        }
    }

    async fn collection_exists(&mut self, _collection: &str) -> Result<bool, Error> {
        if self.raise_error_on_collection_exists {
            Err(Error::new(ErrorKind::Other, "expected error"))
        } else {
            Ok(self.collection_exists)
        }
    }
}

#[test]
fn test_insert() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let doc = block_on(collection.insert(&TestDocumentData {
        field1: String::from("field value"),
        field2: 0xbaad,
        field3: None,
    }))
    .unwrap();
    assert!(!doc.id.is_empty());
    assert!(doc.created_at > 0);
    assert!(doc.updated_at > 0);
    assert_eq!(doc.data.field1, "field value");
    assert_eq!(doc.data.field2, 0xbaad);
    assert_eq!(doc.data.field3, None);
    assert_eq!(
        collection.formatter().format_insert(&doc),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.insert(&TestDocumentData {
        field1: String::from("field value"),
        field2: 0xbaad,
        field3: None,
    }));
    assert!(result.is_err());
}

#[test]
fn test_insert_batch() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let d1 = block_on(collection.insert_batch::<TestDocumentData>(&vec![])).unwrap();
    assert!(d1.is_empty());
    let raw_data: Vec<TestDocumentData> = vec![
        TestDocumentData {
            field1: String::from("doc1 field1"),
            field2: 1000,
            field3: None,
        },
        TestDocumentData {
            field1: String::from("doc2 field1"),
            field2: 2000,
            field3: Some(0.1314),
        },
    ];
    let d2 = block_on(collection.insert_batch(&raw_data)).unwrap();
    assert_eq!(
        collection.formatter().format_insert_batch(&d2),
        collection.storage().statements[0]
    );
    assert_ne!(d2[0].id, d2[1].id);
    assert_eq!(d2[0].created_at, d2[1].created_at);
    assert_eq!(d2[0].updated_at, d2[1].created_at);
    assert_eq!(d2[0].data.field1, "doc1 field1");
    assert_eq!(d2[0].data.field2, 1000);
    assert_eq!(d2[0].data.field3, None);
    assert_eq!(d2[1].data.field1, "doc2 field1");
    assert_eq!(d2[1].data.field2, 2000);
    assert_eq!(d2[1].data.field3, Some(0.1314));
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.insert_batch(&raw_data));
    assert!(result.is_err());
}

#[test]
fn test_update() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let doc = block_on(collection.update::<TestDocumentData>(&Document {
        id: String::from("1000"),
        created_at: 0,
        updated_at: 0,
        data: TestDocumentData {
            field1: String::from("field value"),
            field2: 0xbaad,
            field3: None,
        },
    }))
    .unwrap();
    assert_eq!(doc.id, "1000");
    assert!(doc.updated_at > 0);
    assert_eq!(
        collection.formatter().format_update(&doc),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.update(&doc));
    assert!(result.is_err());
}

#[test]
fn test_update_batch() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let d1 = block_on(collection.update_batch::<TestDocumentData>(&vec![])).unwrap();
    assert!(d1.is_empty());
    let documents = vec![
        Document {
            id: String::from("1000"),
            created_at: 0,
            updated_at: 0,
            data: TestDocumentData {
                field1: String::from("field value1"),
                field2: 10,
                field3: Some(0.1),
            },
        },
        Document {
            id: String::from("2000"),
            created_at: 1,
            updated_at: 1,
            data: TestDocumentData {
                field1: String::from("field value2"),
                field2: 20,
                field3: Some(0.2),
            },
        },
    ];
    let d2 = block_on(collection.update_batch(&documents)).unwrap();
    assert_ne!(documents[0].updated_at, d2[0].updated_at);
    assert_ne!(documents[1].updated_at, d2[1].updated_at);
    assert_eq!(
        collection.formatter().format_update_batch(&d2),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.update_batch(&documents));
    assert!(result.is_err());
}

#[test]
fn test_delete() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let doc = Document {
        id: String::from("1000"),
        created_at: 0,
        updated_at: 0,
        data: TestDocumentData {
            field1: String::from("field value"),
            field2: 0xbaad,
            field3: None,
        },
    };
    block_on(collection.delete::<TestDocumentData>(&doc)).unwrap();
    assert_eq!(
        collection.formatter().format_delete(&doc),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.delete::<TestDocumentData>(&doc));
    assert!(result.is_err());
}

#[test]
fn test_delete_batch() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    block_on(collection.delete_batch::<TestDocumentData>(&vec![])).unwrap();
    let documents = vec![
        Document {
            id: String::from("1000"),
            created_at: 0,
            updated_at: 0,
            data: TestDocumentData {
                field1: String::from("field value1"),
                field2: 10,
                field3: Some(0.1),
            },
        },
        Document {
            id: String::from("2000"),
            created_at: 1,
            updated_at: 1,
            data: TestDocumentData {
                field1: String::from("field value2"),
                field2: 20,
                field3: Some(0.2),
            },
        },
    ];
    block_on(collection.delete_batch(&documents)).unwrap();
    assert_eq!(
        collection.formatter().format_delete_batch(&documents),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    let result = block_on(collection.delete_batch(&documents));
    assert!(result.is_err());
}

#[test]
fn test_delete_by_filter() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    block_on(collection.delete_by_filter::<TestDocumentData>(None)).unwrap();
    assert_eq!(
        collection
            .formatter()
            .format_delete_by_filter::<TestDocumentData>(None),
        collection.storage().statements[0],
    );
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Equal(
            String::from(DOCUMENT_ID_FIELD),
            String::from("1000"),
        )))
        .build();
    block_on(collection.delete_by_filter::<TestDocumentData>(Some(filter.clone()))).unwrap();
    assert_eq!(
        collection
            .formatter()
            .format_delete_by_filter::<TestDocumentData>(Some(filter.clone())),
        collection.storage().statements[1],
    );
    collection.mut_storage().raise_error_on_execute = true;
    assert!(block_on(collection.delete_by_filter::<TestDocumentData>(Some(filter))).is_err());
}

#[test]
fn test_find() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let d1 = block_on(collection.find::<TestDocumentData>(None)).unwrap();
    assert!(d1.is_empty());
    assert_eq!(
        collection.formatter().format_find::<TestDocumentData>(None),
        collection.storage().statements[0]
    );
    collection.mut_storage().expected_data = vec![
        TestDocumentRawData {
            data: HashMap::from([
                (String::from("id"), String::from("1")),
                (String::from("created_at"), String::from("1")),
                (String::from("updated_at"), String::from("1")),
                (String::from("field1"), String::from("doc1 field1")),
                (String::from("field2"), String::from("1000")),
            ]),
        },
        TestDocumentRawData {
            data: HashMap::from([
                (String::from("id"), String::from("2")),
                (String::from("created_at"), String::from("2")),
                (String::from("updated_at"), String::from("2")),
                (String::from("field1"), String::from("doc2 field1")),
                (String::from("field2"), String::from("2000")),
                (String::from("field3"), String::from("0.433")),
            ]),
        },
    ];
    let d2 = block_on(collection.find::<TestDocumentData>(None)).unwrap();
    assert_eq!(d2.len(), 2);
    assert_eq!(d2[0].id, "1");
    assert_eq!(d2[0].created_at, 1);
    assert_eq!(d2[0].updated_at, 1);
    assert_eq!(d2[0].data.field1, "doc1 field1");
    assert_eq!(d2[0].data.field2, 1000);
    assert_eq!(d2[0].data.field3, None);
    assert_eq!(d2[1].id, "2");
    assert_eq!(d2[1].created_at, 2);
    assert_eq!(d2[1].updated_at, 2);
    assert_eq!(d2[1].data.field1, "doc2 field1");
    assert_eq!(d2[1].data.field2, 2000);
    assert_eq!(d2[1].data.field3, Some(0.433));
    collection.mut_storage().raise_error_on_query = true;
    let result = block_on(collection.find::<TestDocumentData>(None));
    assert!(result.is_err());
}

#[test]
fn test_find_one() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let d1 = block_on(collection.find_one::<TestDocumentData>(None)).unwrap();
    assert!(d1.is_none());
    collection.mut_storage().expected_data = vec![TestDocumentRawData {
        data: HashMap::from([
            (String::from("id"), String::from("1")),
            (String::from("created_at"), String::from("1")),
            (String::from("updated_at"), String::from("1")),
            (String::from("field1"), String::from("doc1 field1")),
            (String::from("field2"), String::from("1000")),
        ]),
    }];
    let d2 = block_on(collection.find_one::<TestDocumentData>(None))
        .unwrap()
        .unwrap();
    assert_eq!(d2.created_at, 1);
    assert_eq!(d2.updated_at, 1);
    assert_eq!(d2.data.field1, "doc1 field1");
    assert_eq!(d2.data.field2, 1000);
    assert_eq!(d2.data.field3, None);
    collection.mut_storage().raise_error_on_query = true;
    let result = block_on(collection.find_one::<TestDocumentData>(None));
    assert!(result.is_err());
}

#[test]
fn test_find_by_id() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let d1 = block_on(collection.find_by_id::<TestDocumentData>("1000")).unwrap();
    assert!(d1.is_none());
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Equal(
            String::from(DOCUMENT_ID_FIELD),
            String::from("1000"),
        )))
        .build();
    assert_eq!(
        collection
            .formatter()
            .format_find::<TestDocumentData>(Some(filter)),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_query = true;
    let result = block_on(collection.find_by_id::<TestDocumentData>("1000"));
    assert!(result.is_err());
}

#[test]
fn test_migrate_initialization() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    collection.mut_storage().raise_error_on_collection_exists = true;
    assert!(block_on(collection.migrate(TestDocumentData::schema())).is_err());
    collection.mut_storage().raise_error_on_collection_exists = false;
    let migration = block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    assert_eq!(
        format!(
            "{};{};{}",
            MIGRATION_SCHEMA.migrations[0],
            TestDocumentData::schema().migrations.join(";"),
            collection.formatter().format_insert(&migration)
        ),
        collection.storage().statements[0]
    );
    collection.mut_storage().raise_error_on_execute = true;
    assert!(block_on(collection.migrate(TestDocumentData::schema())).is_err());
}

#[test]
fn test_count() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    let mut count = block_on(collection.count::<TestDocumentData>(None)).unwrap();
    assert_eq!(count, 0);
    collection.mut_storage().expected_data = vec![
        TestDocumentRawData {
            data: HashMap::from([
                (String::from("COUNT(*)"), 123.to_string()),
            ]),
        }
    ];
    count = block_on(collection.count::<TestDocumentData>(None)).unwrap();
    assert_eq!(count, 123);
    collection.mut_storage().expected_data = vec![
        TestDocumentRawData { data: HashMap::from([]) }
    ];
    count = block_on(collection.count::<TestDocumentData>(None)).unwrap();
    assert_eq!(count, 0);
    collection.mut_storage().raise_error_on_query = true;
    assert!(block_on(collection.count::<TestDocumentData>(None)).is_err());
}

#[test]
fn test_migrate_non_existing() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    collection.mut_storage().collection_exists = true;
    let migration = block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Equal(
            String::from(MIGRATION_SCHEMA.field_names[0]),
            String::from(TestDocumentData::schema().collection_name),
        )))
        .build();
    assert_eq!(
        collection
            .formatter()
            .format_find::<Migration>(Some(filter)),
        collection.storage().statements[0]
    );
    assert_eq!(
        format!(
            "{};{}",
            TestDocumentData::schema().migrations.join(";"),
            collection.formatter().format_insert(&migration)
        ),
        collection.storage().statements[1]
    );
}

#[test]
fn test_migrate_existing() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    collection.mut_storage().collection_exists = true;
    collection.mut_storage().expected_data = vec![TestDocumentRawData {
        data: HashMap::from([
            (String::from("id"), String::from("1")),
            (String::from("created_at"), String::from("1")),
            (String::from("updated_at"), String::from("1")),
            (
                String::from("collection_name"),
                String::from(TestDocumentData::schema().collection_name),
            ),
            (String::from("version"), String::from("1")),
        ]),
    }];
    let mut migration = block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Equal(
            String::from(MIGRATION_SCHEMA.field_names[0]),
            String::from(TestDocumentData::schema().collection_name),
        )))
        .build();
    migration.data.version = TestDocumentData::schema().version();
    assert_eq!(
        collection
            .formatter()
            .format_find::<Migration>(Some(filter)),
        collection.storage().statements[0]
    );
    assert_eq!(
        format!(
            "{};{}",
            TestDocumentData::schema().migrations[1..].join(";"),
            collection.formatter().format_update(&migration)
        ),
        collection.storage().statements[1]
    );
}

#[test]
fn test_migrate_skipping() {
    let mut collection: Collection<SqlFormatter, TestDocumentRawData, TestStorage> =
        Collection::new(SqlFormatter {}, TestStorage::new());
    collection.mut_storage().collection_exists = true;
    collection.mut_storage().expected_data = vec![TestDocumentRawData {
        data: HashMap::from([
            (String::from("id"), String::from("1")),
            (String::from("created_at"), String::from("1")),
            (String::from("updated_at"), String::from("1")),
            (
                String::from("collection_name"),
                String::from(TestDocumentData::schema().collection_name),
            ),
            (
                String::from("version"),
                String::from(TestDocumentData::schema().version().to_string()),
            ),
        ]),
    }];
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let filter = QueryFilterBuilder::new()
        .filter(Condition::FILTER(SubFilter::Equal(
            String::from(MIGRATION_SCHEMA.field_names[0]),
            String::from(TestDocumentData::schema().collection_name),
        )))
        .build();
    assert_eq!(collection.storage().statements.len(), 1);
    assert_eq!(
        collection
            .formatter()
            .format_find::<Migration>(Some(filter)),
        collection.storage().statements[0]
    );
}
