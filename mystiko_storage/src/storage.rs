use crate::document::{
    Document, DocumentData, DocumentSchema, DOCUMENT_CREATED_AT_FIELD, DOCUMENT_ID_FIELD,
    DOCUMENT_UPDATED_AT_FIELD,
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use std::io::Error;
use std::time::SystemTime;
use ulid::Ulid;

lazy_static! {
    static ref UUID_GENERATOR: Ulid = Ulid::new();
}

pub struct QueryFilter {}

#[async_trait]
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
        documents_data: &Vec<T>,
    ) -> Result<Vec<Document<T>>, Error> {
        let mut documents: Vec<Document<T>> = Vec::new();
        let now = current_timestamp();
        for document_data in documents_data.iter() {
            let document: Document<T> = Document {
                id: self.generate_uuid(),
                created_at: now,
                updated_at: now,
                data: document_data.clone(),
            };
            documents.push(document);
        }
        match self.execute_sql(format_sql_inserts(&documents)).await {
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
    async fn update<T: DocumentData>(&self, document: &Document<T>) -> Result<Document<T>, Error> {
        let mut document_new = document.clone();
        document_new.updated_at = current_timestamp();
        match self.execute_sql(format_sql_update(&document_new)).await {
            Ok(_) => Ok(document_new),
            Err(e) => Err(e),
        }
    }
    async fn update_batch<T: DocumentData>(
        &self,
        documents: &Vec<Document<T>>,
    ) -> Result<Vec<Document<T>>, Error> {
        let mut documents_new: Vec<Document<T>> = Vec::new();
        for document in documents {
            let mut document_new = document.clone();
            document_new.updated_at = current_timestamp();
            documents_new.push(document_new);
        }
        match self.execute_sql(format_sql_updates(&documents_new)).await {
            Ok(_) => Ok(documents_new),
            Err(e) => Err(e),
        }
    }
    async fn delete<T: DocumentData>(&self, document: &Document<T>) -> Result<(), Error> {
        self.execute_sql(format_sql_delete(document)).await
    }
    async fn delete_batch<T: DocumentData>(
        &self,
        documents: &Vec<Document<T>>,
    ) -> Result<(), Error> {
        self.execute_sql(format_sql_deletes(documents)).await
    }
    async fn execute_sql(&self, sql: String) -> Result<(), Error>;
    async fn query_sql<T: DocumentData>(
        &self,
        sql: String,
        schema: &DocumentSchema,
    ) -> Result<Document<T>, Error>;
}

fn format_sql_insert<T: DocumentData>(document: &Document<T>) -> String {
    let mut fields: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    fields.push(format!("`{}`", DOCUMENT_ID_FIELD));
    fields.push(format!("`{}`", DOCUMENT_CREATED_AT_FIELD));
    fields.push(format!("`{}`", DOCUMENT_UPDATED_AT_FIELD));
    for field in document.data.schema().field_names.iter() {
        fields.push(format!("`{}`", field));
    }
    values.push(format!("'{}'", document.id));
    values.push(format!("'{}'", document.created_at));
    values.push(format!("'{}'", document.updated_at));
    for (_, value) in document.data.to_map().iter() {
        values.push(format!("'{}'", value));
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
    for (field, value) in document.data.to_map().iter() {
        updates.push(format!("`{}` = '{}'", field, value));
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

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
