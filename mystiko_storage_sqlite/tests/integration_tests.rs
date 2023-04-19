use mystiko_storage::collection::Collection;
use mystiko_storage::document::DocumentData;
use mystiko_storage::filter::SubFilter::IsNull;
use mystiko_storage::filter::{Condition, Order, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::SqlFormatter;
use mystiko_storage::storage::Storage;
use mystiko_storage::testing::TestDocumentData;
use mystiko_storage_sqlite::*;
use std::path::Path;
use tempfile::tempdir;

#[tokio::test]
async fn test_collection_exists() {
    let storage = SqliteStorageBuilder::new()
        .in_memory()
        .build()
        .await
        .unwrap();
    let mut collection = Collection::new(SqlFormatter {}, storage);
    assert!(!collection
        .mut_storage()
        .collection_exists(TestDocumentData::schema().collection_name)
        .await
        .unwrap());
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    assert!(collection
        .mut_storage()
        .collection_exists(TestDocumentData::schema().collection_name)
        .await
        .unwrap());
}

#[tokio::test]
async fn test_insert() {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let collection = Collection::new(SqlFormatter {}, storage);
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    let d1 = collection
        .insert(&TestDocumentData {
            field1: String::from("field1 value"),
            field2: 0xdeadbeef,
            field3: Some(std::f64::consts::PI),
        })
        .await
        .unwrap();
    let d2 = collection
        .find_by_id::<TestDocumentData>(&d1.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(d1.id, d2.id);
    assert_eq!(d1.created_at, d2.created_at);
    assert_eq!(d1.updated_at, d2.updated_at);
    assert_eq!(d1.data.field1, d2.data.field1);
    assert_eq!(d1.data.field2, d2.data.field2);
    assert_eq!(d1.data.field3, d2.data.field3);
}

#[tokio::test]
async fn test_update() {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let collection = Collection::new(SqlFormatter {}, storage);
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    let mut d1 = collection
        .insert(&TestDocumentData {
            field1: String::from("field1 value"),
            field2: 0xdeadbeef,
            field3: Some(std::f64::consts::PI),
        })
        .await
        .unwrap();
    d1.data.field3 = None;
    collection.update(&d1).await.unwrap();
    let d2 = collection
        .find_by_id::<TestDocumentData>(&d1.id)
        .await
        .unwrap()
        .unwrap();
    assert!(d2.data.field3.is_none());
}

#[tokio::test]
async fn test_delete() {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let collection = Collection::new(SqlFormatter {}, storage);
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    let d1 = collection
        .insert(&TestDocumentData {
            field1: String::from("field1 value1"),
            field2: 0xdeadbeef,
            field3: Some(std::f64::consts::PI),
        })
        .await
        .unwrap();
    let d2 = collection
        .insert(&TestDocumentData {
            field1: String::from("field1 value2"),
            field2: 0xbaadbabe,
            field3: None,
        })
        .await
        .unwrap();
    collection.delete(&d1).await.unwrap();
    let d3 = collection
        .find_by_id::<TestDocumentData>(&d1.id)
        .await
        .unwrap();
    assert!(d3.is_none());
    collection
        .delete_by_filter::<TestDocumentData>(Some(
            QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from("field1"),
                    String::from("field1 value2"),
                )))
                .build(),
        ))
        .await
        .unwrap();
    let d4 = collection
        .find_by_id::<TestDocumentData>(&d2.id)
        .await
        .unwrap();
    assert!(d4.is_none());
}

#[tokio::test]
async fn test_find() {
    let storage = SqliteStorageBuilder::new().build().await.unwrap();
    let collection = Collection::new(SqlFormatter {}, storage);
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    let docs = collection
        .insert_batch(&vec![
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
        ])
        .await
        .unwrap();
    assert_eq!(collection.count::<TestDocumentData>(None).await.unwrap(), 3);
    let d1 = collection.find::<TestDocumentData>(None).await.unwrap();
    assert_eq!(d1.len(), docs.len());
    for (i, doc) in d1.iter().enumerate() {
        assert_eq!(doc.id, docs[i].id);
    }
    let filter1 = QueryFilterBuilder::new()
        .filter(Condition::FILTER(IsNull(String::from("field3"))))
        .build();
    assert_eq!(
        collection
            .count::<TestDocumentData>(Some(filter1.clone()))
            .await
            .unwrap(),
        1
    );
    let d2 = collection
        .find::<TestDocumentData>(Some(filter1))
        .await
        .unwrap();
    assert_eq!(d2.len(), 1);
    assert_eq!(d2[0].id, docs[1].id);
    let filter2 = QueryFilterBuilder::new()
        .offset(1)
        .limit(2)
        .order_by(vec![String::from("field1")], Order::DESC)
        .build();
    let d3 = collection
        .find::<TestDocumentData>(Some(filter2))
        .await
        .unwrap();
    assert_eq!(d3.len(), 2);
    assert_eq!(d3[0].id, docs[1].id);
    assert_eq!(d3[1].id, docs[0].id);
}

#[tokio::test]
async fn test_file_db() {
    let db_dir = tempdir().unwrap();
    let db_path = db_dir.path().join(Path::new("test.db"));
    let storage = SqliteStorageBuilder::new()
        .path(db_path.to_str().unwrap())
        .build()
        .await
        .unwrap();
    let collection = Collection::new(SqlFormatter {}, storage);
    collection
        .migrate(TestDocumentData::schema())
        .await
        .unwrap();
    std::fs::remove_dir_all(db_dir).unwrap();
}
