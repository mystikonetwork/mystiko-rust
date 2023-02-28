use crate::document::{
    Document, DocumentData, DocumentSchema, DOCUMENT_CREATED_AT_FIELD, DOCUMENT_ID_FIELD,
    DOCUMENT_UPDATED_AT_FIELD,
};
use crate::filter::{Condition, QueryFilter, QueryFilterBuilder, SubFilter};
use crate::migration::{Migration, MIGRATION_SCHEMA};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::io::Error;
use std::time::SystemTime;
use ulid::Ulid;

lazy_static! {
    static ref UUID_GENERATOR: Ulid = Ulid::new();
}

#[async_trait]
pub trait MystikoStorage {
    async fn insert<T: DocumentData>(&self, document_data: &T) -> Result<Document<T>, Error> {
        let now = self.current_timestamp();
        let document: Document<T> = Document {
            id: self.generate_uuid(),
            created_at: now,
            updated_at: now,
            data: document_data.clone(),
        };
        let sql = format_sql_insert(&document);
        match self.execute(sql).await {
            Ok(_) => Ok(document),
            Err(e) => Err(e),
        }
    }
    async fn insert_batch<T: DocumentData>(
        &self,
        documents_data: &Vec<T>,
    ) -> Result<Vec<Document<T>>, Error> {
        if documents_data.is_empty() {
            Ok(vec![])
        } else {
            let mut documents: Vec<Document<T>> = Vec::new();
            let now = self.current_timestamp();
            for document_data in documents_data.iter() {
                let document: Document<T> = Document {
                    id: self.generate_uuid(),
                    created_at: now,
                    updated_at: now,
                    data: document_data.clone(),
                };
                documents.push(document);
            }
            match self.execute(format_sql_inserts(&documents)).await {
                Ok(_) => Ok(documents),
                Err(e) => Err(e),
            }
        }
    }
    async fn find<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        filter: Option<QueryFilter>,
    ) -> Result<Vec<Document<T>>, Error> {
        match self.query(format_sql_find(schema, filter), schema).await {
            Ok(documents) => Ok(documents),
            Err(e) => Err(e),
        }
    }
    async fn find_one<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        filter: Option<QueryFilter>,
    ) -> Result<Option<Document<T>>, Error> {
        match self.find(schema, filter).await {
            Ok(mut documents) => {
                if documents.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(documents.remove(0)))
                }
            }
            Err(e) => Err(e),
        }
    }
    async fn find_by_id<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        id: &str,
    ) -> Result<Option<Document<T>>, Error> {
        let query_filter = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from(DOCUMENT_ID_FIELD),
                String::from(id),
            )))
            .build();
        self.find_one(schema, Some(query_filter)).await
    }
    async fn update<T: DocumentData>(&self, document: &Document<T>) -> Result<Document<T>, Error> {
        let mut document_new = document.clone();
        document_new.updated_at = self.current_timestamp();
        match self.execute(format_sql_update(&document_new)).await {
            Ok(_) => Ok(document_new),
            Err(e) => Err(e),
        }
    }
    async fn update_batch<T: DocumentData>(
        &self,
        documents: &Vec<Document<T>>,
    ) -> Result<Vec<Document<T>>, Error> {
        if documents.is_empty() {
            Ok(vec![])
        } else {
            let mut documents_new: Vec<Document<T>> = Vec::new();
            let now = self.current_timestamp();
            for document in documents {
                let mut document_new = document.clone();
                document_new.updated_at = now;
                documents_new.push(document_new);
            }
            match self.execute(format_sql_updates(&documents_new)).await {
                Ok(_) => Ok(documents_new),
                Err(e) => Err(e),
            }
        }
    }
    async fn delete<T: DocumentData>(&self, document: &Document<T>) -> Result<(), Error> {
        self.execute(format_sql_delete(document)).await
    }
    async fn delete_batch<T: DocumentData>(
        &self,
        documents: &Vec<Document<T>>,
    ) -> Result<(), Error> {
        if documents.is_empty() {
            Ok(())
        } else {
            self.execute(format_sql_deletes(documents)).await
        }
    }
    async fn migrate(&self, schema: &DocumentSchema) -> Result<(), Error> {
        let collection_exists_result = self
            .collection_exists(MIGRATION_SCHEMA.collection_name)
            .await;
        match collection_exists_result {
            Ok(collection_exists) => {
                let existing: Result<Option<Document<Migration>>, Error> = if collection_exists {
                    let query_filter = QueryFilterBuilder::new()
                        .filter(Condition::FILTER(SubFilter::Equal(
                            String::from(MIGRATION_SCHEMA.field_names[0]),
                            String::from(schema.collection_name),
                        )))
                        .build();
                    self.find_one(&MIGRATION_SCHEMA, Some(query_filter)).await
                } else {
                    Ok(None)
                };
                match existing {
                    Ok(Some(mut migration)) => {
                        let current_version: usize = migration.data.version;
                        if current_version >= schema.migrations.len() {
                            Ok(())
                        } else {
                            let migration_sql = schema.migrations[current_version..].join(";");
                            migration.updated_at = self.current_timestamp();
                            migration.data.version = schema.migrations.len();
                            let migration_update_sql = format_sql_update(&migration);
                            self.execute(format!("{};{}", migration_sql, migration_update_sql))
                                .await
                        }
                    }
                    Ok(None) => {
                        let migration_sql = schema.migrations.join(";");
                        let now = self.current_timestamp();
                        let migration: Document<Migration> = Document {
                            id: self.generate_uuid(),
                            created_at: now,
                            updated_at: now,
                            data: Migration {
                                collection_name: String::from(schema.collection_name),
                                version: schema.migrations.len(),
                            },
                        };
                        let migration_creation_sql = format_sql_insert(&migration);
                        if collection_exists {
                            self.execute(format!("{};{}", migration_sql, migration_creation_sql))
                                .await
                        } else {
                            self.execute(format!(
                                "{};{};{}",
                                MIGRATION_SCHEMA.migrations[0],
                                migration_sql,
                                migration_creation_sql
                            ))
                            .await
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
    fn generate_uuid(&self) -> String {
        UUID_GENERATOR.to_string()
    }
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    async fn execute(&self, statement: String) -> Result<(), Error>;
    async fn query<T: DocumentData>(
        &self,
        statement: String,
        schema: &DocumentSchema,
    ) -> Result<Vec<Document<T>>, Error>;
    async fn collection_exists(&self, collection: &str) -> Result<bool, Error>;
}

fn format_sql_insert<T: DocumentData>(document: &Document<T>) -> String {
    let mut fields: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    fields.push(format!("`{}`", DOCUMENT_ID_FIELD));
    fields.push(format!("`{}`", DOCUMENT_CREATED_AT_FIELD));
    fields.push(format!("`{}`", DOCUMENT_UPDATED_AT_FIELD));
    for field in document.data.schema().field_names {
        fields.push(format!("`{}`", field));
    }
    values.push(format!("'{}'", document.id));
    values.push(format!("'{}'", document.created_at));
    values.push(format!("'{}'", document.updated_at));
    let data_map = document.data.to_map();
    for field_name in document.data.schema().field_names {
        let key = field_name.to_string();
        let value = data_map.get(&key).unwrap();
        values.push(format!("'{}'", value.clone()));
    }
    format!(
        "INSERT INTO `{}` ({}) VALUES ({})",
        document.data.schema().collection_name,
        fields.join(", "),
        values.join(", ")
    )
}

fn format_sql_inserts<T: DocumentData>(documents: &Vec<Document<T>>) -> String {
    let sqls: Vec<String> = documents.iter().map(|d| format_sql_insert(d)).collect();
    sqls.join(";")
}

fn format_sql_update<T: DocumentData>(document: &Document<T>) -> String {
    let mut updates: Vec<String> = Vec::new();
    updates.push(format!(
        "`{}` = '{}'",
        DOCUMENT_UPDATED_AT_FIELD, document.updated_at,
    ));
    let data_map = document.data.to_map();
    for field_name in document.data.schema().field_names {
        let key = field_name.to_string();
        let value = data_map.get(&key).unwrap();
        updates.push(format!("`{}` = '{}'", key, value));
    }
    format!(
        "UPDATE `{}` SET {} WHERE `{}` = '{}'",
        document.data.schema().collection_name,
        updates.join(", "),
        DOCUMENT_ID_FIELD,
        document.id
    )
}

fn format_sql_updates<T: DocumentData>(documents: &Vec<Document<T>>) -> String {
    let sqls: Vec<String> = documents.iter().map(|d| format_sql_update(d)).collect();
    sqls.join(";")
}

fn format_sql_delete<T: DocumentData>(document: &Document<T>) -> String {
    format!(
        "DELETE FROM `{}` WHERE `{}` = '{}'",
        document.data.schema().collection_name,
        DOCUMENT_ID_FIELD,
        document.id
    )
}

fn format_sql_deletes<T: DocumentData>(documents: &Vec<Document<T>>) -> String {
    let sqls: Vec<String> = documents.iter().map(|d| format_sql_delete(d)).collect();
    sqls.join(";")
}

fn format_sql_find(schema: &DocumentSchema, filter_option: Option<QueryFilter>) -> String {
    match &filter_option {
        Some(filter) => {
            let filter_sql = filter.to_sql();
            if filter_sql.is_empty() {
                format!("SELECT * FROM `{}`", schema.collection_name)
            } else {
                format!(
                    "SELECT * FROM `{}` WHERE {}",
                    schema.collection_name, filter_sql
                )
            }
        }
        None => format!("SELECT * FROM `{}`", schema.collection_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::ErrorKind;
    use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
    use tokio_test::block_on;

    struct TestStorage {
        id_counter: AtomicU32,
        time_counter: AtomicU64,
        query_statement: String,
        execute_statement: String,
        collection_exists: bool,
        raise_error_on_query: bool,
        raise_error_on_execute: bool,
        raise_error_on_collection_exists: bool,
        expected_data: Vec<HashMap<String, String>>,
    }

    static BOOK_SCHEMA: DocumentSchema = DocumentSchema {
        collection_name: "books",
        migrations: &[
            "CREATE TABLE {} (\
                id VARCHAR(64) NOT NULL PRIMARY KEY,\
                created_at int NOT NULL,
                updated_at int NOT NULL,
                title VARCHAR(255) NOT NULL,
                year int NOT NULL)",
            "ALTER TABLE books RENAME COLUMN title TO title1",
            "ALTER TABLE books RENAME COLUMN year TO year1",
        ],
        field_names: &["title", "year"],
    };
    #[derive(Clone)]
    struct Book {
        title: String,
        year: u32,
    }
    impl From<HashMap<String, String>> for Book {
        fn from(value: HashMap<String, String>) -> Self {
            Book {
                title: value.get("title").unwrap().to_string(),
                year: value.get("year").unwrap().parse().unwrap(),
            }
        }
    }
    impl DocumentData for Book {
        fn schema(&self) -> &'static DocumentSchema {
            &BOOK_SCHEMA
        }

        fn to_map(&self) -> HashMap<String, String> {
            HashMap::from([
                (String::from("title"), self.title.to_string()),
                (String::from("year"), self.year.to_string()),
            ])
        }
    }

    #[async_trait]
    impl MystikoStorage for TestStorage {
        fn generate_uuid(&self) -> String {
            self.id_counter.fetch_add(1, Ordering::SeqCst).to_string()
        }

        fn current_timestamp(&self) -> u64 {
            self.time_counter.fetch_add(1, Ordering::SeqCst)
        }

        async fn execute(&self, statement: String) -> Result<(), Error> {
            if self.raise_error_on_execute {
                Err(Error::new(ErrorKind::Other, "oh no"))
            } else if !self.execute_statement.eq(&statement) {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "expected statement: {} actual statement: {}",
                        self.execute_statement, statement
                    ),
                ))
            } else {
                Ok(())
            }
        }

        async fn query<T: DocumentData>(
            &self,
            statement: String,
            _schema: &DocumentSchema,
        ) -> Result<Vec<Document<T>>, Error> {
            if self.raise_error_on_query {
                Err(Error::new(ErrorKind::Other, "oh no"))
            } else if !self.query_statement.eq(&statement) {
                Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "expected statement: {} actual statement: {}",
                        self.query_statement, statement
                    ),
                ))
            } else {
                let documents_data: Vec<T> = self
                    .expected_data
                    .iter()
                    .map(|d| T::from(d.clone()))
                    .collect();
                let documents: Vec<Document<T>> = documents_data
                    .iter()
                    .map(|d| Document {
                        id: self.generate_uuid(),
                        created_at: self.current_timestamp(),
                        updated_at: self.current_timestamp(),
                        data: d.clone(),
                    })
                    .collect();
                Ok(documents)
            }
        }

        async fn collection_exists(&self, _collection: &str) -> Result<bool, Error> {
            if self.raise_error_on_collection_exists {
                Err(Error::new(ErrorKind::Other, "oh no"))
            } else {
                Ok(self.collection_exists)
            }
        }
    }

    fn build_test_storage() -> TestStorage {
        TestStorage {
            id_counter: AtomicU32::new(0),
            time_counter: AtomicU64::new(0),
            query_statement: String::new(),
            execute_statement: String::new(),
            collection_exists: true,
            raise_error_on_query: false,
            raise_error_on_execute: false,
            raise_error_on_collection_exists: false,
            expected_data: vec![],
        }
    }

    #[test]
    fn test_insert() {
        let mut storage = build_test_storage();
        let book: Book = Book {
            title: String::from("Gone with wind"),
            year: 1800,
        };
        storage.execute_statement = String::from(
            "INSERT INTO `books` (`id`, `created_at`, `updated_at`, `title`, `year`) \
                VALUES ('0', '0', '0', 'Gone with wind', '1800')",
        );
        let b1 = block_on(storage.insert(&book)).unwrap();
        assert_eq!(b1.id, "0");
        assert_eq!(b1.created_at, 0);
        assert_eq!(b1.updated_at, 0);
        assert_eq!(b1.data.title, "Gone with wind");
        assert_eq!(b1.data.year, 1800);
        storage.execute_statement = String::from(
            "INSERT INTO `books` (`id`, `created_at`, `updated_at`, `title`, `year`) \
                VALUES ('1', '1', '1', 'Gone with wind', '1800')",
        );
        let b2 = block_on(storage.insert(&book)).unwrap();
        assert_eq!(b2.id, "1");
        assert_eq!(b2.created_at, 1);
        assert_eq!(b2.updated_at, 1);
        assert_eq!(b2.data.title, "Gone with wind");
        assert_eq!(b2.data.year, 1800);
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.insert(&book)).is_err());
    }

    #[test]
    fn test_insert_batch() {
        let mut storage = build_test_storage();
        let books: Vec<Book> = vec![
            Book {
                title: String::from("Coding with me"),
                year: 1900,
            },
            Book {
                title: String::from("Testing with me"),
                year: 1901,
            },
        ];
        storage.execute_statement = String::from(
            "INSERT INTO `books` (`id`, `created_at`, `updated_at`, `title`, `year`) \
                VALUES ('0', '0', '0', 'Coding with me', '1900');\
                INSERT INTO `books` (`id`, `created_at`, `updated_at`, `title`, `year`) \
                VALUES ('1', '0', '0', 'Testing with me', '1901')",
        );
        let documents = block_on(storage.insert_batch(&books)).unwrap();
        assert_eq!(documents.len(), 2);
        assert_eq!(documents.get(0).unwrap().id, "0");
        assert_eq!(documents.get(0).unwrap().created_at, 0);
        assert_eq!(documents.get(0).unwrap().updated_at, 0);
        assert_eq!(documents.get(0).unwrap().data.title, "Coding with me");
        assert_eq!(documents.get(0).unwrap().data.year, 1900);
        assert_eq!(documents.get(1).unwrap().id, "1");
        assert_eq!(documents.get(1).unwrap().created_at, 0);
        assert_eq!(documents.get(1).unwrap().updated_at, 0);
        assert_eq!(documents.get(1).unwrap().data.title, "Testing with me");
        assert_eq!(documents.get(1).unwrap().data.year, 1901);
        assert!(block_on(storage.insert_batch::<Book>(&vec![]))
            .unwrap()
            .is_empty());
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.insert_batch(&books)).is_err());
    }

    #[test]
    fn test_update() {
        let mut storage = build_test_storage();
        storage.time_counter.fetch_add(1, Ordering::SeqCst);
        let d1: Document<Book> = Document {
            id: String::from("0"),
            created_at: 0,
            updated_at: 0,
            data: Book {
                title: String::from("Gone with wind"),
                year: 1810,
            },
        };
        storage.execute_statement = String::from(
            "UPDATE `books` SET \
            `updated_at` = '1', `title` = 'Gone with wind', `year` = '1810' WHERE `id` = '0'",
        );
        let d2 = block_on(storage.update(&d1)).unwrap();
        assert_eq!(d2.updated_at, 1);
        assert_eq!(d2.data.title, "Gone with wind");
        assert_eq!(d2.data.year, 1810);
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.update(&d1)).is_err());
    }

    #[test]
    fn test_update_batch() {
        let mut storage = build_test_storage();
        let documents: Vec<Document<Book>> = vec![
            Document {
                id: String::from("0"),
                created_at: 0,
                updated_at: 0,
                data: Book {
                    title: String::from("Coding with me"),
                    year: 2000,
                },
            },
            Document {
                id: String::from("1"),
                created_at: 0,
                updated_at: 0,
                data: Book {
                    title: String::from("Testing with you"),
                    year: 1901,
                },
            },
        ];
        storage.time_counter.fetch_add(1, Ordering::SeqCst);
        storage.execute_statement = String::from(
            "UPDATE `books` SET \
            `updated_at` = '1', `title` = 'Coding with me', `year` = '2000' WHERE `id` = '0';\
            UPDATE `books` SET \
            `updated_at` = '1', `title` = 'Testing with you', `year` = '1901' WHERE `id` = '1'",
        );
        let documents_new = block_on(storage.update_batch(&documents)).unwrap();
        assert_eq!(documents_new.get(0).unwrap().updated_at, 1);
        assert_eq!(documents_new.get(0).unwrap().data.title, "Coding with me");
        assert_eq!(documents_new.get(0).unwrap().data.year, 2000);
        assert_eq!(documents_new.get(1).unwrap().updated_at, 1);
        assert_eq!(documents_new.get(1).unwrap().data.title, "Testing with you");
        assert_eq!(documents_new.get(1).unwrap().data.year, 1901);
        assert!(block_on(storage.update_batch::<Book>(&vec![]))
            .unwrap()
            .is_empty());
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.update_batch(&documents)).is_err());
    }

    #[test]
    fn test_delete() {
        let mut storage = build_test_storage();
        let document: Document<Book> = Document {
            id: String::from("0"),
            created_at: 0,
            updated_at: 0,
            data: Book {
                title: String::from("Gone with wind"),
                year: 1800,
            },
        };
        storage.execute_statement = String::from("DELETE FROM `books` WHERE `id` = '0'");
        block_on(storage.delete(&document)).unwrap();
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.delete(&document)).is_err());
    }

    #[test]
    fn test_delete_batch() {
        let mut storage = build_test_storage();
        let documents: Vec<Document<Book>> = vec![
            Document {
                id: String::from("0"),
                created_at: 0,
                updated_at: 0,
                data: Book {
                    title: String::from("Coding with me"),
                    year: 2000,
                },
            },
            Document {
                id: String::from("1"),
                created_at: 0,
                updated_at: 0,
                data: Book {
                    title: String::from("Testing with me"),
                    year: 1901,
                },
            },
        ];
        storage.execute_statement = String::from(
            "DELETE FROM `books` WHERE `id` = '0';\
                DELETE FROM `books` WHERE `id` = '1'",
        );
        block_on(storage.delete_batch(&documents)).unwrap();
        assert!(block_on(storage.delete_batch::<Book>(&vec![])).is_ok());
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.delete_batch(&documents)).is_err());
    }

    #[test]
    fn test_find() {
        let mut storage = build_test_storage();
        storage.query_statement =
            String::from("SELECT * FROM `books` WHERE `title` = 'Gone with wind'");
        storage.expected_data = vec![
            HashMap::from([
                (String::from("title"), String::from("Gone with wind")),
                (String::from("year"), String::from("1800")),
            ]),
            HashMap::from([
                (String::from("title"), String::from("Gone with wind")),
                (String::from("year"), String::from("2000")),
            ]),
        ];
        let filter1 = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from("title"),
                String::from("Gone with wind"),
            )))
            .build();
        let documents1 = block_on(storage.find::<Book>(&BOOK_SCHEMA, Some(filter1))).unwrap();
        assert_eq!(documents1.len(), 2);
        assert_eq!(documents1.get(0).unwrap().id, "0");
        assert_eq!(documents1.get(0).unwrap().created_at, 0);
        assert_eq!(documents1.get(0).unwrap().updated_at, 1);
        assert_eq!(documents1.get(0).unwrap().data.title, "Gone with wind");
        assert_eq!(documents1.get(0).unwrap().data.year, 1800);
        assert_eq!(documents1.get(1).unwrap().id, "1");
        assert_eq!(documents1.get(1).unwrap().created_at, 2);
        assert_eq!(documents1.get(1).unwrap().updated_at, 3);
        assert_eq!(documents1.get(1).unwrap().data.title, "Gone with wind");
        assert_eq!(documents1.get(1).unwrap().data.year, 2000);
        storage.query_statement = String::from("SELECT * FROM `books` WHERE `year` = '1700'");
        storage.expected_data = vec![];
        let filter2 = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from("year"),
                String::from("1700"),
            )))
            .build();
        let documents2 = block_on(storage.find::<Book>(&BOOK_SCHEMA, Some(filter2))).unwrap();
        assert!(documents2.is_empty());
        storage.query_statement = String::from("SELECT * FROM `books`");
        let filter3 = QueryFilterBuilder::new().build();
        let documents3 = block_on(storage.find::<Book>(&BOOK_SCHEMA, Some(filter3))).unwrap();
        assert!(documents3.is_empty());
        let documents3 = block_on(storage.find::<Book>(&BOOK_SCHEMA, None)).unwrap();
        assert!(documents3.is_empty());
        storage.raise_error_on_query = true;
        assert!(block_on(storage.find::<Book>(&BOOK_SCHEMA, None)).is_err());
    }

    #[test]
    fn test_find_one() {
        let mut storage = build_test_storage();
        storage.query_statement = String::from("SELECT * FROM `books`");
        storage.expected_data = vec![
            HashMap::from([
                (String::from("title"), String::from("Testing with me")),
                (String::from("year"), String::from("1800")),
            ]),
            HashMap::from([
                (String::from("title"), String::from("Gone with wind")),
                (String::from("year"), String::from("2000")),
            ]),
        ];
        let d1 = block_on(storage.find_one::<Book>(&BOOK_SCHEMA, None))
            .unwrap()
            .unwrap();
        assert_eq!(d1.id, "0");
        assert_eq!(d1.created_at, 0);
        assert_eq!(d1.updated_at, 1);
        assert_eq!(d1.data.title, "Testing with me");
        assert_eq!(d1.data.year, 1800);
        storage.expected_data = vec![];
        let d2 = block_on(storage.find_one::<Book>(&BOOK_SCHEMA, None)).unwrap();
        assert!(d2.is_none());
        storage.raise_error_on_query = true;
        assert!(block_on(storage.find_one::<Book>(&BOOK_SCHEMA, None)).is_err());
    }

    #[test]
    fn test_find_by_id() {
        let mut storage = build_test_storage();
        storage.query_statement = String::from("SELECT * FROM `books` WHERE `id` = '1'");
        storage.expected_data = vec![HashMap::from([
            (String::from("title"), String::from("Coding with me")),
            (String::from("year"), String::from("1800")),
        ])];
        let d1 = block_on(storage.find_by_id::<Book>(&BOOK_SCHEMA, &String::from("1")))
            .unwrap()
            .unwrap();
        assert_eq!(d1.id, "0");
        assert_eq!(d1.created_at, 0);
        assert_eq!(d1.updated_at, 1);
        assert_eq!(d1.data.title, "Coding with me");
        assert_eq!(d1.data.year, 1800);
        storage.expected_data = vec![];
        let d2 = block_on(storage.find_by_id::<Book>(&BOOK_SCHEMA, &String::from("1"))).unwrap();
        assert!(d2.is_none());
        storage.raise_error_on_query = true;
        assert!(block_on(storage.find_by_id::<Book>(&BOOK_SCHEMA, &String::from("1"))).is_err());
    }

    #[test]
    fn test_migrate_initialization() {
        let mut storage = build_test_storage();
        storage.collection_exists = false;
        storage.raise_error_on_collection_exists = true;
        assert!(block_on(storage.migrate(&BOOK_SCHEMA)).is_err());
        storage.raise_error_on_collection_exists = false;
        storage.execute_statement = format!(
            "{};{};INSERT INTO `{}` \
                (`id`, `created_at`, `updated_at`, `collection_name`, `version`) VALUES \
                ('0', '0', '0', 'books', '3')",
            MIGRATION_SCHEMA.migrations[0],
            BOOK_SCHEMA.migrations.join(";"),
            MIGRATION_SCHEMA.collection_name
        );
        block_on(storage.migrate(&BOOK_SCHEMA)).unwrap();
        storage.raise_error_on_execute = true;
        assert!(block_on(storage.migrate(&BOOK_SCHEMA)).is_err());
    }

    #[test]
    fn test_migrate_non_existing() {
        let mut storage = build_test_storage();
        storage.collection_exists = true;
        storage.query_statement = format!(
            "SELECT * FROM `{}` WHERE `collection_name` = 'books'",
            MIGRATION_SCHEMA.collection_name
        );
        storage.execute_statement = format!(
            "{};INSERT INTO `{}` \
                (`id`, `created_at`, `updated_at`, `collection_name`, `version`) VALUES \
                ('0', '0', '0', 'books', '3')",
            BOOK_SCHEMA.migrations.join(";"),
            MIGRATION_SCHEMA.collection_name
        );
        block_on(storage.migrate(&BOOK_SCHEMA)).unwrap();
    }

    #[test]
    fn test_migrate_existing() {
        let mut storage = build_test_storage();
        storage.collection_exists = true;
        storage.query_statement = format!(
            "SELECT * FROM `{}` WHERE `collection_name` = 'books'",
            MIGRATION_SCHEMA.collection_name
        );
        storage.execute_statement = format!(
            "{};UPDATE `{}` SET \
            `updated_at` = '2', `collection_name` = 'books', `version` = '3' WHERE `id` = '0'",
            BOOK_SCHEMA.migrations[1..].join(";"),
            MIGRATION_SCHEMA.collection_name
        );
        storage.expected_data = vec![HashMap::from([
            (String::from("collection_name"), String::from("books")),
            (String::from("version"), String::from("1")),
        ])];
        block_on(storage.migrate(&BOOK_SCHEMA)).unwrap();
        storage.raise_error_on_execute = true;
        storage.time_counter.store(0, Ordering::SeqCst);
        assert!(block_on(storage.migrate(&BOOK_SCHEMA)).is_err());
        storage.raise_error_on_execute = false;
        storage.raise_error_on_query = true;
        storage.time_counter.store(0, Ordering::SeqCst);
        assert!(block_on(storage.migrate(&BOOK_SCHEMA)).is_err());
    }

    #[test]
    fn test_utils() {
        struct SomeStorage {}
        #[async_trait]
        impl MystikoStorage for SomeStorage {
            async fn execute(&self, _statement: String) -> Result<(), Error> {
                Err(Error::new(ErrorKind::Other, "not implemented"))
            }

            async fn query<T: DocumentData>(
                &self,
                _statement: String,
                _schema: &DocumentSchema,
            ) -> Result<Vec<Document<T>>, Error> {
                Err(Error::new(ErrorKind::Other, "not implemented"))
            }

            async fn collection_exists(&self, _collection: &str) -> Result<bool, Error> {
                Err(Error::new(ErrorKind::Other, "not implemented"))
            }
        }
        let storage: SomeStorage = SomeStorage {};
        assert!(!storage.generate_uuid().is_empty());
        assert!(storage.current_timestamp() > 0);
    }
}
