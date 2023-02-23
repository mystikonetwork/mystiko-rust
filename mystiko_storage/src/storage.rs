use crate::document::{Document, DocumentData, DocumentSchema};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::io::Error;
use std::time::SystemTime;
use ulid::Ulid;

lazy_static! {
    static ref UUID_GENERATOR: Ulid = Ulid::new();
}

pub struct QueryFilter {}

#[async_trait(?Send)]
pub trait MystikoStorage {
    fn generate_uuid(&self) -> String {
        UUID_GENERATOR.to_string()
    }
    async fn insert<T: DocumentData>(&self, document_data: &T) -> Result<Document<T>, Error> {
        let now = current_timestamp();
        let document: Document<T> = Document {
            id: self.generate_uuid(),
            created_at: now,
            updated_at: now,
            data: document_data.clone(),
        };
        let sql = format_sql_insert(&document);
        match self.execute_sql(sql).await {
            Ok(_) => Ok(document),
            Err(e) => Err(e),
        }
    }
    async fn insert_batch<T: DocumentData>(
        &self,
        documents_data: &[T],
    ) -> Result<Vec<Document<T>>, Error> {
        let mut document_sqls: Vec<String> = Vec::new();
        let mut documents: Vec<Document<T>> = Vec::new();
        let now = current_timestamp();
        for document_data in documents_data.iter() {
            let document: Document<T> = Document {
                id: self.generate_uuid(),
                created_at: now,
                updated_at: now,
                data: document_data.clone(),
            };
            document_sqls.push(format_sql_insert(&document));
            documents.push(document);
        }
        match self.execute_sql(document_sqls.join(";")).await {
            Ok(_) => Ok(documents),
            Err(e) => Err(e),
        }
    }
    async fn find<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        filter: &QueryFilter,
    ) -> Result<Vec<Document<T>>, Error>;
    async fn find_one<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        filter: &QueryFilter,
    ) -> Result<Document<T>, Error>;
    async fn find_by_id<T: DocumentData>(
        &self,
        schema: &DocumentSchema,
        id: &str,
    ) -> Result<Document<T>, Error>;
    async fn update<T: DocumentData>(&self, document: &Document<T>) -> Result<Document<T>, Error>;
    async fn update_batch<T: DocumentData>(
        &self,
        documents: &[Document<T>],
    ) -> Result<Vec<Document<T>>, Error>;
    async fn delete<T: DocumentData>(&self, document: &Document<T>) -> Result<Document<T>, Error>;
    async fn delete_batch<T: DocumentData>(
        &self,
        documents: &[Document<T>],
    ) -> Result<Vec<Document<T>>, Error>;
    async fn execute_sql(&self, sql: String) -> Result<(), Error>;
    async fn query_sql<T: DocumentData>(
        &self,
        sql: String,
        schema: &DocumentSchema,
    ) -> Result<Document<T>, Error>;
}

fn format_sql_insert<T: DocumentData>(document: &Document<T>) -> String {
    todo!()
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
