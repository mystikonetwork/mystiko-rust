#![forbid(unsafe_code)]
use num_traits::{Float, PrimInt};
use std::io::Error;
use std::marker::{Send, Sync};
use std::str::FromStr;

pub struct DocumentSchema {
    pub collection_name: &'static str,
    pub migrations: &'static [&'static str],
    pub field_names: &'static [&'static str],
}

impl DocumentSchema {
    pub fn version(&self) -> usize {
        self.migrations.len()
    }
}

pub trait DocumentRawData: Send + Sync {
    fn field_integer_value<T: PrimInt + FromStr>(&self, field: &str) -> Result<Option<T>, Error>;
    fn field_float_value<T: Float + FromStr>(&self, field: &str) -> Result<Option<T>, Error>;
    fn field_string_value(&self, field: &str) -> Result<Option<String>, Error>;
}

pub trait DocumentData: Clone + Send + Sync {
    fn schema() -> &'static DocumentSchema;
    fn field_value_string(&self, field: &str) -> Option<String>;
    fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error>;
}

#[derive(Clone)]
pub struct Document<T: DocumentData> {
    pub id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub data: T,
}

pub const DOCUMENT_ID_FIELD: &str = "id";
pub const DOCUMENT_CREATED_AT_FIELD: &str = "created_at";
pub const DOCUMENT_UPDATED_AT_FIELD: &str = "updated_at";

impl<T: DocumentData> Document<T> {
    pub fn field_value_string(&self, field: &str) -> Option<String> {
        match field {
            DOCUMENT_ID_FIELD => Some(self.id.clone()),
            DOCUMENT_CREATED_AT_FIELD => Some(self.created_at.to_string()),
            DOCUMENT_UPDATED_AT_FIELD => Some(self.updated_at.to_string()),
            _ => self.data.field_value_string(field),
        }
    }
    pub fn field_names() -> Vec<&'static str> {
        let mut names = vec![
            DOCUMENT_ID_FIELD,
            DOCUMENT_CREATED_AT_FIELD,
            DOCUMENT_UPDATED_AT_FIELD,
        ];
        names.extend(T::schema().field_names);
        names
    }
    pub fn field_index(field: &str) -> Option<usize> {
        Document::<T>::field_names()
            .iter()
            .position(|f| f.eq(&field))
    }
    pub fn deserialize<F: DocumentRawData>(raw: &F) -> Result<Self, Error> {
        Ok(Document {
            id: raw.field_string_value(DOCUMENT_ID_FIELD)?.unwrap(),
            created_at: raw.field_integer_value(DOCUMENT_CREATED_AT_FIELD)?.unwrap(),
            updated_at: raw.field_integer_value(DOCUMENT_UPDATED_AT_FIELD)?.unwrap(),
            data: T::deserialize(raw)?,
        })
    }
}
