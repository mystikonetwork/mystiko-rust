use mystiko_protos::storage::v1::{ConditionOperator, Order, OrderBy, QueryFilter, SubFilter};
use mystiko_storage::{Collection, DocumentColumn, DocumentData, SqlStatementFormatter};
use mystiko_storage_macros::CollectionBuilder;
use mystiko_storage_mysql::{MySqlStorage, MySqlStorageOptions};
use num_bigint::{BigInt, BigUint};
use sqlx::MySqlPool;
use std::sync::Arc;
use std::vec;

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

#[sqlx::test]
async fn test_insert(pool: MySqlPool) {
    let collection = create_collection(pool).await;
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

#[sqlx::test]
async fn test_update(pool: MySqlPool) {
    let collection = create_collection(pool).await;
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

#[sqlx::test]
async fn test_delete(pool: MySqlPool) {
    let collection = create_collection(pool).await;
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

#[sqlx::test]
async fn test_find(pool: MySqlPool) {
    let collection = create_collection(pool).await;
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
    let filter = QueryFilter::builder()
        .conditions_operator(ConditionOperator::And)
        .conditions(vec![SubFilter::greater(
            TestDocumentColumn::Field3,
            documents[0].data.field3,
        )
        .into()])
        .order_by(
            OrderBy::builder()
                .columns(vec![TestDocumentColumn::Field3.to_string()])
                .order(Order::Asc as i32)
                .build(),
        )
        .build();
    assert_eq!(documents[1..], collection.find(filter).await.unwrap());
    let filter = QueryFilter::builder()
        .conditions_operator(ConditionOperator::And)
        .conditions(vec![SubFilter::greater(
            TestDocumentColumn::Field3,
            documents[0].data.field3,
        )
        .into()])
        .order_by(
            OrderBy::builder()
                .columns(vec![TestDocumentColumn::Field3.to_string()])
                .order(Order::Asc as i32)
                .build(),
        )
        .limit(1)
        .offset(1)
        .build();
    assert_eq!(documents[2..], collection.find(filter).await.unwrap());
}

#[sqlx::test]
async fn test_count(pool: MySqlPool) {
    let collection = create_collection(pool).await;
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

#[sqlx::test]
async fn test_collection_exists(pool: MySqlPool) {
    let collection = create_collection(pool).await;
    assert!(!collection.collection_exists().await.unwrap());
    collection.migrate().await.unwrap();
    assert!(collection.collection_exists().await.unwrap());
}

#[sqlx::test]
async fn test_builder(_pool: MySqlPool) {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let parsed_url = url::Url::parse(&database_url).unwrap();
    let options = MySqlStorageOptions::builder()
        .database(parsed_url.path().trim_start_matches('/').to_owned())
        .host(parsed_url.host_str().unwrap().to_owned())
        .port(parsed_url.port().unwrap())
        .username(parsed_url.username().to_owned())
        .password(parsed_url.password().unwrap().to_owned())
        .max_connections(1u32)
        .min_connections(1u32)
        .max_lifetime(std::time::Duration::from_secs(120))
        .idle_timeout(std::time::Duration::from_secs(60))
        .build();
    let _ = MySqlStorage::connect(options).await.unwrap();
}

#[sqlx::test]
async fn test_unsupported_primitive_integers(pool: MySqlPool) {
    let collection = create_collection(pool).await;
    collection.migrate().await.unwrap();
    let mut documents = test_documents();
    documents[0].field13 = 100u128;
    documents[0].field25 = -100i128;
    documents[1].field13 = 20u128;
    documents[1].field25 = 20i128;
    documents[2].field13 = 3u128;
    documents[2].field25 = -3i128;
    let documents = collection.insert_batch(&documents).await.unwrap();
    let mut found_documents = collection
        .find(SubFilter::greater(TestDocumentColumn::Field13, 3u128))
        .await
        .unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(documents[..2], found_documents);
    found_documents = collection
        .find(SubFilter::less(TestDocumentColumn::Field13, 100u128))
        .await
        .unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(documents[1..], found_documents);
    found_documents = collection
        .find(SubFilter::greater(TestDocumentColumn::Field25, 0i128))
        .await
        .unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(documents[1..2], found_documents);
    found_documents = collection
        .find(SubFilter::less(TestDocumentColumn::Field25, -3i128))
        .await
        .unwrap();
    found_documents.sort_by_key(|d| d.data.field3);
    assert_eq!(documents[0..1], found_documents);
}

#[test]
fn test_create_default_stroage() {
    let option1 = MySqlStorageOptions::default();
    assert_eq!(option1.database, "mystiko_database".to_string());
    let default_host = "localhost";
    let default_port = 3306u16;
    let default_username = "root";
    let default_max_connections = 10u32;
    let default_min_connections = 1u32;
    assert_eq!(option1.host, default_host.to_string());
    assert_eq!(option1.port, default_port);
    assert_eq!(option1.username, default_username.to_string());
    assert_eq!(option1.password, None);
    assert_eq!(option1.max_connections, default_max_connections);
    assert_eq!(option1.min_connections, default_min_connections);
    assert_eq!(option1.idle_timeout, None);
    assert_eq!(option1.max_lifetime, None);
    let option2 = MySqlStorageOptions::builder()
        .database("mystiko_database".to_string())
        .build();
    assert_eq!(option2.database, "mystiko_database".to_string());
    assert_eq!(option2.host, default_host.to_string());
    assert_eq!(option2.port, default_port);
    assert_eq!(option2.username, default_username.to_string());
    assert_eq!(option2.password, None);
    assert_eq!(option2.max_connections, default_max_connections);
    assert_eq!(option2.min_connections, default_min_connections);
    assert_eq!(option2.idle_timeout, None);
    assert_eq!(option2.max_lifetime, None);
}

async fn create_collection(pool: MySqlPool) -> TestDocumentCollection<SqlStatementFormatter, MySqlStorage> {
    let result: (String,) = sqlx::query_as("SELECT DATABASE()").fetch_one(&pool).await.unwrap();
    let storage = MySqlStorage::builder().pool(pool).database(result.0).build();
    TestDocumentCollection::new(Arc::new(Collection::new(SqlStatementFormatter::mysql(), storage)))
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
