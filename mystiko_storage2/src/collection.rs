use crate::document::{Document, DocumentColumn, DocumentData};
use crate::error::StorageError;
use crate::filter::{QueryFilter, SubFilter};
use crate::formatter::types::{Statement, StatementFormatter};
use crate::migration::history::{MigrationHistory, MigrationHistoryColumn};
use crate::storage::Storage;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Collection<F: StatementFormatter, S: Storage> {
    formatter: F,
    storage: S,
}

impl<F: StatementFormatter, S: Storage> Collection<F, S> {
    pub fn new(formatter: F, storage: S) -> Self {
        Collection { formatter, storage }
    }

    pub async fn insert<D: DocumentData>(&self, data: &D) -> Result<Document<D>, StorageError> {
        let now = current_timestamp();
        let document: Document<D> = Document {
            id: self.storage.uuid().await?,
            created_at: now,
            updated_at: now,
            data: data.clone(),
        };
        self.storage.execute(self.formatter.format_insert(&document)).await?;
        Ok(document)
    }

    pub async fn insert_batch<D: DocumentData>(&self, data: &Vec<D>) -> Result<Vec<Document<D>>, StorageError> {
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
                .execute_batch(self.formatter.format_insert_batch(&documents))
                .await?;
            Ok(documents)
        }
    }

    pub async fn find<D: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter: Option<Q>,
    ) -> Result<Vec<Document<D>>, StorageError> {
        self.storage
            .query::<D>(self.formatter.format_find::<D, Q>(filter))
            .await
    }

    pub async fn find_one<D: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter: Option<Q>,
    ) -> Result<Option<Document<D>>, StorageError> {
        let mut documents = self.find(filter).await?;
        if documents.is_empty() {
            Ok(None)
        } else {
            Ok(Some(documents.remove(0)))
        }
    }

    pub async fn find_by_id<D: DocumentData>(&self, id: &str) -> Result<Option<Document<D>>, StorageError> {
        let query_filter = SubFilter::equal(&DocumentColumn::Id, String::from(id));
        self.find_one(Some(query_filter)).await
    }

    pub async fn update<D: DocumentData>(&self, document: &Document<D>) -> Result<Document<D>, StorageError> {
        let mut document_new = document.clone();
        document_new.updated_at = current_timestamp();
        self.storage
            .execute(self.formatter.format_update(&document_new))
            .await?;
        Ok(document_new)
    }

    pub async fn update_batch<D: DocumentData>(
        &self,
        documents: &Vec<Document<D>>,
    ) -> Result<Vec<Document<D>>, StorageError> {
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
                .execute_batch(self.formatter.format_update_batch(&documents_new))
                .await?;
            Ok(documents_new)
        }
    }

    pub async fn delete<D: DocumentData>(&self, document: &Document<D>) -> Result<(), StorageError> {
        self.storage.execute(self.formatter.format_delete(document)).await
    }

    pub async fn delete_batch<D: DocumentData>(&self, documents: &Vec<Document<D>>) -> Result<(), StorageError> {
        if documents.is_empty() {
            Ok(())
        } else {
            self.storage
                .execute_batch(self.formatter.format_delete_batch(documents))
                .await
        }
    }

    pub async fn delete_by_filter<D: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter: Option<Q>,
    ) -> Result<(), StorageError> {
        self.storage
            .execute(self.formatter.format_delete_by_filter::<D, Q>(filter))
            .await
    }

    pub async fn count<D: DocumentData, Q: Into<QueryFilter>>(&self, filter: Option<Q>) -> Result<u64, StorageError> {
        self.storage.count(self.formatter.format_count::<D, Q>(filter)).await
    }

    pub async fn collection_exists(&self, collection_name: &str) -> Result<bool, StorageError> {
        self.storage.collection_exists(collection_name).await
    }

    pub async fn migrate<D: DocumentData>(&self) -> Result<Document<MigrationHistory>, StorageError> {
        let collection_exists = self.collection_exists(&MigrationHistory::collection_name()).await?;
        let existing: Option<Document<MigrationHistory>> = if collection_exists {
            let query_filter = SubFilter::equal(&MigrationHistoryColumn::CollectionName, D::collection_name());
            self.find_one(Some(query_filter)).await?
        } else {
            None
        };
        match existing {
            Some(mut migration) => {
                let current_version: usize = migration.data.version;
                if current_version >= D::version() {
                    Ok(migration)
                } else {
                    let mut migration_statements: Vec<Statement> = self
                        .formatter
                        .format_migration_batch(&Document::<D>::migrations()[current_version..]);
                    migration.updated_at = current_timestamp();
                    migration.data.version = D::version();
                    migration_statements.push(self.formatter.format_update(&migration));
                    self.storage.execute_batch(migration_statements).await?;
                    Ok(migration)
                }
            }
            None => {
                let mut migration_statements = vec![];
                if !collection_exists {
                    migration_statements.extend(
                        self.formatter
                            .format_migration_batch(&Document::<MigrationHistory>::migrations()),
                    );
                }
                migration_statements.extend(self.formatter.format_migration_batch(&Document::<D>::migrations()));
                let now = current_timestamp();
                let migration: Document<MigrationHistory> = Document {
                    id: self.storage.uuid().await?,
                    created_at: now,
                    updated_at: now,
                    data: MigrationHistory {
                        collection_name: D::collection_name(),
                        version: D::version(),
                    },
                };
                migration_statements.push(self.formatter.format_insert(&migration));
                self.storage.execute_batch(migration_statements).await?;
                Ok(migration)
            }
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
