#![forbid(unsafe_code)]
use crate::document::{Document, DocumentData, DocumentSchema, DOCUMENT_ID_FIELD};
use crate::filter::{Condition, QueryFilter, QueryFilterBuilder, SubFilter};
use crate::formatter::StatementFormatter;
use crate::migration::{Migration, MIGRATION_SCHEMA};
use crate::storage::Storage;
use std::io::Error;
use std::time::SystemTime;

pub struct Collection<F: StatementFormatter, S: Storage> {
    formatter: F,
    storage: S,
}

impl<F: StatementFormatter, S: Storage> Collection<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        Collection { formatter, storage }
    }
    pub async fn insert<D: DocumentData>(&mut self, data: &D) -> Result<Document<D>, Error> {
        let now = current_timestamp();
        let document: Document<D> = Document {
            id: self.storage.uuid().await?,
            created_at: now,
            updated_at: now,
            data: data.clone(),
        };
        let sql = self.formatter.format_insert(&document);
        self.storage.execute(sql).await?;
        Ok(document)
    }
    pub async fn insert_batch<D: DocumentData>(
        &mut self,
        data: &Vec<D>,
    ) -> Result<Vec<Document<D>>, Error> {
        if data.is_empty() {
            Ok(vec![])
        } else {
            let mut documents: Vec<Document<D>> = Vec::new();
            let now = current_timestamp();
            for doc in data.iter() {
                let document: Document<D> = Document {
                    id: self.storage.uuid().await?,
                    created_at: now,
                    updated_at: now,
                    data: doc.clone(),
                };
                documents.push(document);
            }
            self.storage
                .execute(self.formatter.format_insert_batch(&documents))
                .await?;
            Ok(documents)
        }
    }
    pub async fn find<D: DocumentData>(
        &mut self,
        filter: Option<QueryFilter>,
    ) -> Result<Vec<Document<D>>, Error> {
        self.storage
            .query(self.formatter.format_find::<D>(filter))
            .await
    }
    pub async fn find_one<D: DocumentData>(
        &mut self,
        filter: Option<QueryFilter>,
    ) -> Result<Option<Document<D>>, Error> {
        let mut documents = self.find(filter).await?;
        if documents.is_empty() {
            Ok(None)
        } else {
            Ok(Some(documents.remove(0)))
        }
    }
    pub async fn find_by_id<D: DocumentData>(
        &mut self,
        id: &str,
    ) -> Result<Option<Document<D>>, Error> {
        let query_filter = QueryFilterBuilder::new()
            .filter(Condition::FILTER(SubFilter::Equal(
                String::from(DOCUMENT_ID_FIELD),
                String::from(id),
            )))
            .build();
        self.find_one(Some(query_filter)).await
    }
    pub async fn update<D: DocumentData>(
        &mut self,
        document: &Document<D>,
    ) -> Result<Document<D>, Error> {
        let mut document_new = document.clone();
        document_new.updated_at = current_timestamp();
        self.storage
            .execute(self.formatter.format_update(&document_new))
            .await?;
        Ok(document_new)
    }
    pub async fn update_batch<D: DocumentData>(
        &mut self,
        documents: &Vec<Document<D>>,
    ) -> Result<Vec<Document<D>>, Error> {
        if documents.is_empty() {
            Ok(vec![])
        } else {
            let mut documents_new: Vec<Document<D>> = Vec::new();
            let now = current_timestamp();
            for document in documents {
                let mut document_new = document.clone();
                document_new.updated_at = now;
                documents_new.push(document_new);
            }
            self.storage
                .execute(self.formatter.format_update_batch(&documents_new))
                .await?;
            Ok(documents_new)
        }
    }
    pub async fn delete<D: DocumentData>(&mut self, document: &Document<D>) -> Result<(), Error> {
        self.storage
            .execute(self.formatter.format_delete(document))
            .await
    }
    pub async fn delete_batch<D: DocumentData>(
        &mut self,
        documents: &Vec<Document<D>>,
    ) -> Result<(), Error> {
        if documents.is_empty() {
            Ok(())
        } else {
            self.storage
                .execute(self.formatter.format_delete_batch(documents))
                .await
        }
    }
    pub async fn migrate(&mut self, schema: &DocumentSchema) -> Result<Document<Migration>, Error> {
        let collection_exists = self.collection_exists(schema).await?;
        let existing: Option<Document<Migration>> = if collection_exists {
            let query_filter = QueryFilterBuilder::new()
                .filter(Condition::FILTER(SubFilter::Equal(
                    String::from(MIGRATION_SCHEMA.field_names[0]),
                    String::from(schema.collection_name),
                )))
                .build();
            self.find_one(Some(query_filter)).await?
        } else {
            None
        };
        match existing {
            Some(mut migration) => {
                let current_version: usize = migration.data.version;
                if current_version >= schema.version() {
                    Ok(migration)
                } else {
                    let migration_sql = schema.migrations[current_version..].join(";");
                    migration.updated_at = current_timestamp();
                    migration.data.version = schema.migrations.len();
                    let migration_update_sql = self.formatter.format_update(&migration);
                    self.storage
                        .execute(format!("{};{}", migration_sql, migration_update_sql))
                        .await?;
                    Ok(migration)
                }
            }
            None => {
                let migration_sql = schema.migrations.join(";");
                let now = current_timestamp();
                let migration: Document<Migration> = Document {
                    id: self.storage.uuid().await?,
                    created_at: now,
                    updated_at: now,
                    data: Migration {
                        collection_name: String::from(schema.collection_name),
                        version: schema.migrations.len(),
                    },
                };
                let migration_creation_sql = self.formatter.format_insert(&migration);
                if collection_exists {
                    self.storage
                        .execute(format!("{};{}", migration_sql, migration_creation_sql))
                        .await?;
                    Ok(migration)
                } else {
                    self.storage
                        .execute(format!(
                            "{};{};{}",
                            MIGRATION_SCHEMA.migrations[0], migration_sql, migration_creation_sql
                        ))
                        .await?;
                    Ok(migration)
                }
            }
        }
    }

    pub async fn collection_exists(&mut self, schema: &DocumentSchema) -> Result<bool, Error> {
        self.storage.collection_exists(schema.collection_name).await
    }

    pub fn formatter(&self) -> &F {
        &self.formatter
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn mut_storage(&mut self) -> &mut S {
        &mut self.storage
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
