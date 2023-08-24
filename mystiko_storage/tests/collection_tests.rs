use mystiko_protos::storage::v1::{ColumnType, QueryFilter, SubFilter};
use mystiko_storage::collection::Collection;
use mystiko_storage::column::Column;
use mystiko_storage::document::{Document, DocumentColumn, DocumentData};
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage::migration::types::{
    AddColumnMigration, AddIndexMigration, DropColumnMigration, Migration, RenameColumnMigration,
};
use mystiko_storage_macros::CollectionBuilder;
use mystiko_storage_sqlite::{SqliteStorage, SqliteStorageBuilder};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[tokio::test]
async fn test_migration() {
    let collection = create_collection().await;
    assert!(!collection
        .collection_exists(TestDocument::collection_name())
        .await
        .unwrap());
    let migration_history = collection.migrate::<TestDocument>().await.unwrap();
    assert_eq!(migration_history.data.collection_name, TestDocument::collection_name());
    assert_eq!(migration_history.data.version, Document::<TestDocument>::version());
    assert!(collection
        .collection_exists(TestDocument::collection_name())
        .await
        .unwrap());
    let migration_history = collection.migrate::<TestDocument2>().await.unwrap();
    assert_eq!(migration_history.data.collection_name, TestDocument2::collection_name());
    assert_eq!(migration_history.data.version, Document::<TestDocument2>::version());
    let migration_history = collection.migrate::<TestDocument2>().await.unwrap();
    assert_eq!(migration_history.data.collection_name, TestDocument2::collection_name());
    assert_eq!(migration_history.data.version, Document::<TestDocument2>::version());
    let document = collection
        .insert::<TestDocument2>(&TestDocument2 {
            field1: 1,
            field3_new: true,
            field4: Some(String::from("abc")),
        })
        .await
        .unwrap();
    assert_eq!(document.data.field1, 1);
    assert!(document.data.field3_new);
    assert_eq!(document.data.field4, Some(String::from("abc")));
}

#[tokio::test]
async fn test_insert() {
    let collection = create_collection().await;
    collection.migrate::<TestDocument>().await.unwrap();
    assert!(collection.insert_batch::<TestDocument>(&[]).await.unwrap().is_empty());
    let documents = vec![
        TestDocument {
            field1: 1,
            field2: 2,
            field3: true,
        },
        TestDocument {
            field1: 3,
            field2: 4,
            field3: false,
        },
        TestDocument {
            field1: 5,
            field2: 6,
            field3: true,
        },
    ];
    let document = collection.insert(&documents[0]).await.unwrap();
    assert_eq!(document.created_at, document.updated_at);
    assert!(document.created_at > 0);
    assert_eq!(document.data, documents[0]);
    let insert_documents = collection.insert_batch(&documents[1..]).await.unwrap();
    assert_eq!(
        insert_documents.into_iter().map(|d| d.data).collect::<Vec<_>>(),
        documents[1..]
    );
}

#[tokio::test]
async fn test_update() {
    let collection = create_collection().await;
    collection.migrate::<TestDocument>().await.unwrap();
    assert!(collection.update_batch::<TestDocument>(&[]).await.unwrap().is_empty());
    let mut documents = collection
        .insert_batch(&[
            TestDocument {
                field1: 1,
                field2: 2,
                field3: true,
            },
            TestDocument {
                field1: 3,
                field2: 4,
                field3: false,
            },
            TestDocument {
                field1: 5,
                field2: 6,
                field3: true,
            },
        ])
        .await
        .unwrap();
    documents[0].data.field3 = false;
    documents[1].data.field3 = true;
    documents[2].data.field3 = false;
    tokio::time::sleep(Duration::from_millis(1)).await;
    let document = collection.update(&documents[0]).await.unwrap();
    assert!(!document.data.field3);
    assert!(document.updated_at > document.created_at);
    let update_documents = collection.update_batch(&documents[1..]).await.unwrap();
    assert_eq!(
        update_documents.iter().map(|d| d.data.field3).collect::<Vec<_>>(),
        documents[1..].iter().map(|d| d.data.field3).collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_delete() {
    let collection = create_collection().await;
    collection.migrate::<TestDocument>().await.unwrap();
    let documents = collection
        .insert_batch(&[
            TestDocument {
                field1: 1,
                field2: 2,
                field3: true,
            },
            TestDocument {
                field1: 3,
                field2: 4,
                field3: false,
            },
            TestDocument {
                field1: 5,
                field2: 6,
                field3: true,
            },
        ])
        .await
        .unwrap();
    collection.delete_batch::<TestDocument>(&[]).await.unwrap();
    assert_eq!(collection.count::<TestDocument, QueryFilter>(None).await.unwrap(), 3);
    collection.delete::<TestDocument>(&documents[0]).await.unwrap();
    assert_eq!(collection.count::<TestDocument, QueryFilter>(None).await.unwrap(), 2);
    collection
        .delete_by_filter::<TestDocument, _>(Some(SubFilter::equal(
            TestDocumentColumn::Field1,
            documents[1].data.field1,
        )))
        .await
        .unwrap();
    assert_eq!(collection.count::<TestDocument, QueryFilter>(None).await.unwrap(), 1);
    collection.delete_batch(&documents[2..]).await.unwrap();
    assert_eq!(collection.count::<TestDocument, QueryFilter>(None).await.unwrap(), 0);
}

#[tokio::test]
async fn test_find() {
    let collection = create_collection().await;
    collection.migrate::<TestDocument>().await.unwrap();
    let documents = collection
        .insert_batch(&[
            TestDocument {
                field1: 1,
                field2: 2,
                field3: true,
            },
            TestDocument {
                field1: 3,
                field2: 4,
                field3: false,
            },
            TestDocument {
                field1: 5,
                field2: 6,
                field3: true,
            },
        ])
        .await
        .unwrap();
    let document = collection.find_by_id::<TestDocument>(&documents[0].id).await.unwrap();
    assert_eq!(document.unwrap(), documents[0]);
    let document = collection
        .find_one(Some(SubFilter::equal(DocumentColumn::Id, documents[1].id.clone())))
        .await
        .unwrap();
    assert_eq!(document.unwrap(), documents[1]);
    assert!(collection
        .find_by_id::<TestDocument>("non_existing_id")
        .await
        .unwrap()
        .is_none());
    let mut found_documents = collection.find::<TestDocument, QueryFilter>(None).await.unwrap();
    found_documents.sort_by_key(|d| d.data.field1);
    assert_eq!(found_documents, documents);
    found_documents = collection
        .find::<TestDocument, _>(Some(SubFilter::greater(TestDocumentColumn::Field1, 1i16)))
        .await
        .unwrap();
    found_documents.sort_by_key(|d| d.data.field1);
    assert_eq!(found_documents, documents[1..]);
}

#[tokio::test]
async fn test_serde() {
    let collection = create_collection().await;
    collection.migrate::<TestDocument>().await.unwrap();
    assert!(collection.insert_batch::<TestDocument>(&[]).await.unwrap().is_empty());
    let document = collection
        .insert(&TestDocument {
            field1: 1,
            field2: 2,
            field3: true,
        })
        .await
        .unwrap();
    assert_eq!(
        document,
        serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap()
    )
}

#[derive(CollectionBuilder, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestDocument {
    pub field1: i16,
    pub field2: u32,
    pub field3: bool,
}

#[derive(CollectionBuilder, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[collection(name = "test_documents", migrations = migrations())]
pub struct TestDocument2 {
    pub field1: i16,
    pub field3_new: bool,
    #[column(length_limit = 128)]
    pub field4: Option<String>,
}

fn migrations() -> Vec<Migration> {
    vec![
        AddIndexMigration::builder()
            .index_name("my_index_1")
            .column_names(vec![TestDocumentColumn::Field1.to_string()])
            .build()
            .into(),
        AddColumnMigration::builder()
            .column(
                Column::builder()
                    .column_type(ColumnType::String)
                    .column_name("field4")
                    .nullable(true)
                    .length_limit(Some(128))
                    .build(),
            )
            .build()
            .into(),
        DropColumnMigration::builder()
            .column_name(TestDocumentColumn::Field2)
            .build()
            .into(),
        RenameColumnMigration::builder()
            .old_column_name(TestDocumentColumn::Field3)
            .new_column_name("field3_new")
            .build()
            .into(),
    ]
}

async fn create_collection() -> Collection<SqlStatementFormatter, SqliteStorage> {
    let storage = SqliteStorageBuilder::new().in_memory().build().await.unwrap();
    Collection::new(SqlStatementFormatter::sqlite(), storage)
}
