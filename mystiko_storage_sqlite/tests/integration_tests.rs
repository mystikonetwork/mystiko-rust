use mystiko_protos::storage::v1::{ConditionOperator, Order, OrderBy, QueryFilter, SubFilter};
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{DocumentColumn, DocumentData};
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage_macros::CollectionBuilder;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use num_bigint::{BigInt, BigUint};
use std::path::Path;
use std::sync::Arc;
use tempfile::tempdir;

#[derive(CollectionBuilder, Debug, Clone, PartialEq)]
pub struct TestDocument {
    pub field1: bool,
    pub field2: Option<bool>,
    pub field3: char,
    pub field4: Option<char>,
    pub field5: u8,
    pub field6: Option<u8>,
    pub field7: u16,
    pub field8: Option<u16>,
    pub field9: u32,
    pub field10: Option<u32>,
    pub field11: u64,
    pub field12: Option<u64>,
    pub field13: u128,
    pub field14: Option<u128>,
    pub field15: usize,
    pub field16: Option<usize>,
    pub field17: i8,
    pub field18: Option<i8>,
    pub field19: i16,
    pub field20: Option<i16>,
    pub field21: i32,
    pub field22: Option<i32>,
    pub field23: i64,
    pub field24: Option<i64>,
    pub field25: i128,
    pub field26: Option<i128>,
    pub field27: isize,
    pub field28: Option<isize>,
    pub field29: f32,
    pub field30: Option<f32>,
    pub field31: f64,
    pub field32: Option<f64>,
    pub field33: String,
    pub field34: Option<String>,
    pub field35: BigInt,
    pub field36: Option<BigInt>,
    pub field37: BigUint,
    pub field38: Option<BigUint>,
    pub field39: Vec<u8>,
    pub field40: Option<Vec<u8>>,
}

#[tokio::test]
async fn test_collection_exists() {
    let collection = create_collection().await;
    assert!(!collection.collection_exists().await.unwrap());
    collection.migrate().await.unwrap();
    assert!(collection.collection_exists().await.unwrap());
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
    let collection = TestDocumentCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)));
    collection.migrate().await.unwrap();
    std::fs::remove_dir_all(db_dir).unwrap();
}

#[tokio::test]
async fn test_insert() {
    let collection = create_collection().await;
    let documents: Vec<_> = test_documents();
    collection.migrate().await.unwrap();
    collection.insert(&documents[0]).await.unwrap();
    collection.insert_batch(&documents[1..]).await.unwrap();
    let mut found_documents = collection.find_all().await.unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(
        documents,
        found_documents.into_iter().map(|d| d.data).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_update() {
    let collection = create_collection().await;
    collection.migrate().await.unwrap();
    let mut documents = collection.insert_batch(&test_documents()).await.unwrap();
    documents[0].data.field1 = false;
    documents[1].data.field2 = None;
    documents[2].data.field39 = vec![];
    collection.update(&documents[0]).await.unwrap();
    collection.update_batch(&documents[1..]).await.unwrap();
    documents = collection.find_all().await.unwrap();
    documents.sort_by_key(|d| d.data.field3);
    assert!(!documents[0].data.field1);
    assert!(documents[1].data.field2.is_none());
    assert!(documents[2].data.field39.is_empty());
}

#[tokio::test]
async fn test_delete() {
    let collection = create_collection().await;
    collection.migrate().await.unwrap();
    let documents = collection.insert_batch(&test_documents()).await.unwrap();
    collection.delete(&documents[0]).await.unwrap();
    assert_eq!(collection.count_all().await.unwrap(), 2);
    collection.delete_batch(&documents[1..]).await.unwrap();
    assert_eq!(collection.count_all().await.unwrap(), 0);
    let documents = collection.insert_batch(&test_documents()).await.unwrap();
    collection
        .delete_by_filter(SubFilter::equal(DocumentColumn::Id, documents[0].id.clone()))
        .await
        .unwrap();
    assert_eq!(collection.count_all().await.unwrap(), 2);
    collection.delete_all().await.unwrap();
    assert_eq!(collection.count_all().await.unwrap(), 0);
}

#[tokio::test]
async fn test_find() {
    let collection = create_collection().await;
    collection.migrate().await.unwrap();
    let mut documents = collection.insert_batch(&test_documents()).await.unwrap();
    documents.sort_by_key(|d| d.data.field3);
    let mut found_documents = collection.find_all().await.unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(documents, found_documents);
    assert_eq!(
        documents[0],
        collection.find_by_id(&documents[0].id).await.unwrap().unwrap()
    );
    assert_eq!(
        documents[1],
        collection
            .find_one(SubFilter::equal(DocumentColumn::Id, documents[1].id.clone()))
            .await
            .unwrap()
            .unwrap()
    );
    assert_eq!(
        documents[1..],
        collection
            .find(SubFilter::greater(TestDocumentColumn::Field3, documents[0].data.field3))
            .await
            .unwrap()
    );
    let filter = QueryFilter::builder()
        .conditions_operator(ConditionOperator::And)
        .conditions(vec![SubFilter::greater(
            TestDocumentColumn::Field3,
            documents[0].data.field3,
        )
        .into()])
        .limit(1)
        .offset(1)
        .order_by(
            OrderBy::builder()
                .order(Order::Asc)
                .columns(vec![TestDocumentColumn::Field3.to_string()])
                .build(),
        )
        .build();
    assert_eq!(documents[2..], collection.find(filter).await.unwrap());
}

#[tokio::test]
async fn test_count() {
    let collection = create_collection().await;
    collection.migrate().await.unwrap();
    let documents = collection.insert_batch(&test_documents()).await.unwrap();
    assert_eq!(collection.count_all().await.unwrap(), 3);
    let filter = QueryFilter::builder()
        .conditions_operator(ConditionOperator::And)
        .conditions(vec![SubFilter::greater(
            TestDocumentColumn::Field3,
            documents[0].data.field3,
        )
        .into()])
        .build();
    assert_eq!(collection.count(filter).await.unwrap(), 2);
}

async fn create_collection() -> TestDocumentCollection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().in_memory().build().await.unwrap();
    TestDocumentCollection::new(Arc::new(Collection::new(SqlStatementFormatter::sqlite(), storage)))
}

fn test_documents() -> Vec<TestDocument> {
    vec![
        TestDocument {
            field1: true,
            field2: None,
            field3: 'a',
            field4: None,
            field5: u8::MAX,
            field6: None,
            field7: u16::MAX,
            field8: None,
            field9: u32::MAX,
            field10: None,
            field11: u64::MAX,
            field12: None,
            field13: u128::MAX,
            field14: None,
            field15: usize::MAX,
            field16: None,
            field17: i8::MIN,
            field18: None,
            field19: i16::MIN,
            field20: None,
            field21: i32::MIN,
            field22: None,
            field23: i64::MIN,
            field24: None,
            field25: i128::MIN,
            field26: None,
            field27: isize::MIN,
            field28: None,
            field29: f32::MAX,
            field30: None,
            field31: f64::MIN,
            field32: None,
            field33: "field33".to_string(),
            field34: None,
            field35: BigInt::from(-0xdeadbeefbaadbabei128),
            field36: None,
            field37: BigUint::from(0xdeadbeefbaadbabeu128),
            field38: None,
            field39: vec![1, 2, 3, 4],
            field40: None,
        },
        TestDocument {
            field1: true,
            field2: None,
            field3: 'b',
            field4: Some('c'),
            field5: u8::MAX - 1,
            field6: Some(u8::MAX - 2),
            field7: u16::MAX - 1,
            field8: Some(u16::MAX - 2),
            field9: u32::MAX - 1,
            field10: Some(u32::MAX - 2),
            field11: u64::MAX - 1,
            field12: Some(u64::MAX - 2),
            field13: u128::MAX - 1,
            field14: Some(u128::MAX - 2),
            field15: usize::MAX - 1,
            field16: Some(usize::MAX - 2),
            field17: i8::MIN + 1,
            field18: Some(i8::MIN + 2),
            field19: i16::MIN + 1,
            field20: Some(i16::MIN + 2),
            field21: i32::MIN + 1,
            field22: Some(i32::MIN + 2),
            field23: i64::MIN + 1,
            field24: Some(i64::MIN + 2),
            field25: i128::MIN + 1,
            field26: Some(i128::MIN + 2),
            field27: isize::MIN + 1,
            field28: Some(isize::MIN + 2),
            field29: f32::MAX - 1.0,
            field30: Some(f32::MAX - 2.0),
            field31: f64::MIN + 1.0,
            field32: Some(f64::MAX + 2.0),
            field33: "field33_1".to_string(),
            field34: Some("field33_2".to_string()),
            field35: BigInt::from(-0xdeadbeefbaadbabdi128),
            field36: None,
            field37: BigUint::from(0xdeadbeefbaadbabfu128),
            field38: None,
            field39: vec![10, 20, 30, 40],
            field40: Some(vec![50, 60, 70, 80]),
        },
        TestDocument {
            field1: true,
            field2: None,
            field3: 'd',
            field4: Some('e'),
            field5: u8::MAX - 2,
            field6: Some(u8::MAX - 3),
            field7: u16::MAX - 2,
            field8: Some(u16::MAX - 3),
            field9: u32::MAX - 2,
            field10: Some(u32::MAX - 3),
            field11: u64::MAX - 2,
            field12: Some(u64::MAX - 3),
            field13: u128::MAX - 2,
            field14: Some(u128::MAX - 3),
            field15: usize::MAX - 2,
            field16: Some(usize::MAX - 3),
            field17: i8::MIN + 2,
            field18: Some(i8::MIN + 3),
            field19: i16::MIN + 2,
            field20: Some(i16::MIN + 3),
            field21: i32::MIN + 2,
            field22: Some(i32::MIN + 3),
            field23: i64::MIN + 2,
            field24: Some(i64::MIN + 3),
            field25: i128::MIN + 2,
            field26: Some(i128::MIN + 3),
            field27: isize::MIN + 2,
            field28: Some(isize::MIN + 3),
            field29: f32::MAX - 2.0,
            field30: Some(f32::MAX - 3.0),
            field31: f64::MIN + 2.0,
            field32: Some(f64::MAX + 3.0),
            field33: "field33_2".to_string(),
            field34: Some("field33_3".to_string()),
            field35: BigInt::from(-0xdeadbeefbaadbabci128),
            field36: Some(BigInt::from(-0xdeadbeefbaadbabbi128)),
            field37: BigUint::from(0xdeadbeefbaadbac0u128),
            field38: Some(BigUint::from(0xdeadbeefbaadbac1u128)),
            field39: vec![11, 21, 31, 41],
            field40: Some(vec![51, 52, 53, 54]),
        },
    ]
}
