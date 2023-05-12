#![forbid(unsafe_code)]
use crate::common::error::RollerError;
use crate::db::document::commitment::CommitmentInfo;
use mystiko_storage::collection::Collection;
use mystiko_storage::document::{Document, DocumentData, DocumentRawData};
use mystiko_storage::filter::QueryFilter;
use mystiko_storage::formatter::StatementFormatter;
use mystiko_storage::migration::Migration;
use mystiko_storage::storage::Storage;
use std::sync::Arc;

pub type Result<T> = anyhow::Result<T, RollerError>;

#[derive(Debug)]
pub struct CommitmentInfoCollection<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> {
    collection: Arc<Collection<F, R, S>>,
}

impl<F: StatementFormatter, R: DocumentRawData, S: Storage<R>> CommitmentInfoCollection<F, R, S> {
    pub fn new(collection: Arc<Collection<F, R, S>>) -> Self {
        CommitmentInfoCollection { collection }
    }

    pub async fn insert(&self, commitment: &CommitmentInfo) -> Result<Document<CommitmentInfo>> {
        self.collection
            .insert(commitment)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn insert_batch(&self, commitments: &Vec<CommitmentInfo>) -> Result<Vec<Document<CommitmentInfo>>> {
        self.collection
            .insert_batch(commitments)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn find<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Vec<Document<CommitmentInfo>>> {
        self.collection
            .find::<CommitmentInfo, Q>(Some(filter))
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn find_all(&self) -> Result<Vec<Document<CommitmentInfo>>> {
        self.collection
            .find::<CommitmentInfo, QueryFilter>(None)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn find_one<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<Option<Document<CommitmentInfo>>> {
        self.collection
            .find_one(Some(filter))
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Document<CommitmentInfo>>> {
        self.collection.find_by_id(id).await.map_err(RollerError::DatabaseError)
    }

    pub async fn count<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<u64> {
        self.collection
            .count::<CommitmentInfo, Q>(Some(filter))
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn count_all(&self) -> Result<u64> {
        self.collection
            .count::<CommitmentInfo, QueryFilter>(None)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn update(&self, commitment: &Document<CommitmentInfo>) -> Result<Document<CommitmentInfo>> {
        self.collection
            .update(commitment)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn update_batch(
        &self,
        commitments: &Vec<Document<CommitmentInfo>>,
    ) -> Result<Vec<Document<CommitmentInfo>>> {
        self.collection
            .update_batch(commitments)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn delete(&self, commitment: &Document<CommitmentInfo>) -> Result<()> {
        self.collection
            .delete(commitment)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn delete_batch(&self, commitments: &Vec<Document<CommitmentInfo>>) -> Result<()> {
        self.collection
            .delete_batch(commitments)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn delete_all(&self) -> Result<()> {
        self.collection
            .delete_by_filter::<CommitmentInfo, QueryFilter>(None)
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn delete_by_filter<Q: Into<QueryFilter>>(&self, filter: Q) -> Result<()> {
        self.collection
            .delete_by_filter::<CommitmentInfo, Q>(Some(filter))
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn migrate(&self) -> Result<Document<Migration>> {
        self.collection
            .migrate(CommitmentInfo::schema())
            .await
            .map_err(RollerError::DatabaseError)
    }

    pub async fn collection_exists(&self) -> Result<bool> {
        self.collection
            .collection_exists(CommitmentInfo::schema())
            .await
            .map_err(RollerError::DatabaseError)
    }
}
