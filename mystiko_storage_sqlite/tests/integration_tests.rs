use mystiko_storage::collection::Collection;
use mystiko_storage::document::DocumentData;
use mystiko_storage::filter::SubFilter::IsNull;
use mystiko_storage::filter::{Condition, Order, QueryFilterBuilder};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage::storage::Storage;
use mystiko_storage::testing::TestDocumentData;
use mystiko_storage_sqlite::*;
use std::path::Path;
use tempfile::tempdir;
use tokio_test::block_on;

#[test]
fn test_collection_exists() {
    let storage = block_on(SqliteStorageBuilder::new().in_memory().build()).unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    assert!(!block_on(
        collection
            .mut_storage()
            .collection_exists(TestDocumentData::schema().collection_name)
    )
    .unwrap());
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    assert!(block_on(
        collection
            .mut_storage()
            .collection_exists(TestDocumentData::schema().collection_name)
    )
    .unwrap());
}

#[test]
fn test_insert() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let d1 = block_on(collection.insert(&TestDocumentData {
        field1: String::from("field1 value"),
        field2: 0xdeadbeef,
        field3: Some(std::f64::consts::PI),
    }))
    .unwrap();
    let d2 = block_on(collection.find_by_id::<TestDocumentData>(&d1.id))
        .unwrap()
        .unwrap();
    assert_eq!(d1.id, d2.id);
    assert_eq!(d1.created_at, d2.created_at);
    assert_eq!(d1.updated_at, d2.updated_at);
    assert_eq!(d1.data.field1, d2.data.field1);
    assert_eq!(d1.data.field2, d2.data.field2);
    assert_eq!(d1.data.field3, d2.data.field3);
}

#[test]
fn test_update() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let mut d1 = block_on(collection.insert(&TestDocumentData {
        field1: String::from("field1 value"),
        field2: 0xdeadbeef,
        field3: Some(std::f64::consts::PI),
    }))
    .unwrap();
    d1.data.field3 = None;
    block_on(collection.update(&d1)).unwrap();
    let d2 = block_on(collection.find_by_id::<TestDocumentData>(&d1.id))
        .unwrap()
        .unwrap();
    assert!(d2.data.field3.is_none());
}

#[test]
fn test_delete() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let d1 = block_on(collection.insert(&TestDocumentData {
        field1: String::from("field1 value"),
        field2: 0xdeadbeef,
        field3: Some(std::f64::consts::PI),
    }))
    .unwrap();
    block_on(collection.delete(&d1)).unwrap();
    let d2 = block_on(collection.find_by_id::<TestDocumentData>(&d1.id)).unwrap();
    assert!(d2.is_none());
}

#[test]
fn test_find() {
    let storage = block_on(SqliteStorageBuilder::new().build()).unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    let docs = block_on(collection.insert_batch(&vec![
        TestDocumentData {
            field1: String::from("doc1 field1"),
            field2: 0xdeadbeef,
            field3: Some(std::f64::consts::PI),
        },
        TestDocumentData {
            field1: String::from("doc2 field1"),
            field2: 0xbaadbabe,
            field3: None,
        },
        TestDocumentData {
            field1: String::from("doc3 field1"),
            field2: 0xdeadbeef,
            field3: Some(std::f64::consts::PI),
        },
    ]))
    .unwrap();
    let d1 = block_on(collection.find::<TestDocumentData>(None)).unwrap();
    assert_eq!(d1.len(), docs.len());
    for (i, doc) in d1.iter().enumerate() {
        assert_eq!(doc.id, docs[i].id);
    }
    let filter1 = QueryFilterBuilder::new()
        .filter(Condition::FILTER(IsNull(String::from("field3"))))
        .build();
    let d2 = block_on(collection.find::<TestDocumentData>(Some(filter1))).unwrap();
    assert_eq!(d2.len(), 1);
    assert_eq!(d2[0].id, docs[1].id);
    let filter2 = QueryFilterBuilder::new()
        .offset(1)
        .limit(2)
        .order_by(vec![String::from("field1")], Order::DESC)
        .build();
    let d3 = block_on(collection.find::<TestDocumentData>(Some(filter2))).unwrap();
    assert_eq!(d3.len(), 2);
    assert_eq!(d3[0].id, docs[1].id);
    assert_eq!(d3[1].id, docs[0].id);
}

#[test]
fn test_file_db() {
    let db_dir = tempdir().unwrap();
    let db_path = db_dir.path().join(Path::new("test.db"));
    let storage = block_on(
        SqliteStorageBuilder::new()
            .path(db_path.to_str().unwrap())
            .build(),
    )
    .unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    block_on(collection.migrate(TestDocumentData::schema())).unwrap();
    std::fs::remove_dir_all(db_dir).unwrap();
}
