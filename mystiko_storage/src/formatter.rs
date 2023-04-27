#![forbid(unsafe_code)]
use crate::document::{
    Document, DocumentData, DOCUMENT_CREATED_AT_FIELD, DOCUMENT_ID_FIELD, DOCUMENT_UPDATED_AT_FIELD,
};
use crate::filter::QueryFilter;

pub trait StatementFormatter: Send + Sync {
    fn format_insert<T: DocumentData>(&self, doc: &Document<T>) -> String;
    fn format_insert_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String;
    fn format_update<T: DocumentData>(&self, doc: &Document<T>) -> String;
    fn format_update_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String;
    fn format_delete<T: DocumentData>(&self, doc: &Document<T>) -> String;
    fn format_delete_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String;
    fn format_delete_by_filter<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String;
    fn format_count<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String;
    fn format_find<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String;
}

pub struct SqlFormatter {}

impl StatementFormatter for SqlFormatter {
    fn format_insert<T: DocumentData>(&self, doc: &Document<T>) -> String {
        let mut fields: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        for field in Document::<T>::field_names() {
            fields.push(format!("`{}`", field));
        }
        for field_name in Document::<T>::field_names() {
            let value = doc.field_value_string(field_name);
            if value.is_some() {
                values.push(format!("'{}'", value.unwrap()));
            } else {
                values.push("NULL".to_string());
            }
        }
        format!(
            "INSERT INTO `{}` ({}) VALUES ({})",
            T::schema().collection_name,
            fields.join(", "),
            values.join(", ")
        )
    }

    fn format_insert_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String {
        let statements: Vec<String> = docs.iter().map(|d| self.format_insert(d)).collect();
        statements.join(";")
    }

    fn format_update<T: DocumentData>(&self, doc: &Document<T>) -> String {
        let mut updates: Vec<String> = Vec::new();
        updates.push(format!("`{}` = '{}'", DOCUMENT_UPDATED_AT_FIELD, doc.updated_at,));
        for field_name in T::schema().field_names {
            let value = doc.field_value_string(field_name);
            if value.is_some() {
                updates.push(format!("`{}` = '{}'", field_name, value.unwrap()));
            } else {
                updates.push(format!("`{}` = {}", field_name, "NULL"));
            }
        }
        format!(
            "UPDATE `{}` SET {} WHERE `{}` = '{}'",
            T::schema().collection_name,
            updates.join(", "),
            DOCUMENT_ID_FIELD,
            doc.id
        )
    }

    fn format_update_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String {
        let statements: Vec<String> = docs.iter().map(|d| self.format_update(d)).collect();
        statements.join(";")
    }

    fn format_delete<T: DocumentData>(&self, doc: &Document<T>) -> String {
        format!(
            "DELETE FROM `{}` WHERE `{}` = '{}'",
            T::schema().collection_name,
            DOCUMENT_ID_FIELD,
            doc.id
        )
    }

    fn format_delete_batch<T: DocumentData>(&self, docs: &[Document<T>]) -> String {
        let statements: Vec<String> = docs.iter().map(|d| self.format_delete(d)).collect();
        statements.join(";")
    }

    fn format_delete_by_filter<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String {
        match filter_option {
            Some(filter) => {
                let filter_sql = filter.to_sql();
                if filter_sql.is_empty() {
                    format!("DELETE FROM `{}`", T::schema().collection_name)
                } else if filter.conditions.is_empty() {
                    format!("DELETE FROM `{}` {}", T::schema().collection_name, filter_sql)
                } else {
                    format!("DELETE FROM `{}` WHERE {}", T::schema().collection_name, filter_sql)
                }
            }
            None => format!("DELETE FROM `{}`", T::schema().collection_name),
        }
    }

    fn format_count<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String {
        match filter_option {
            Some(filter) => {
                let filter_sql = filter.to_sql();
                if filter_sql.is_empty() {
                    format!("SELECT COUNT(*) FROM `{}`", T::schema().collection_name)
                } else if filter.conditions.is_empty() {
                    format!("SELECT COUNT(*) FROM `{}` {}", T::schema().collection_name, filter_sql)
                } else {
                    format!(
                        "SELECT COUNT(*) FROM `{}` WHERE {}",
                        T::schema().collection_name,
                        filter_sql
                    )
                }
            }
            None => format!("SELECT COUNT(*) FROM `{}`", T::schema().collection_name),
        }
    }

    fn format_find<T: DocumentData>(&self, filter_option: Option<QueryFilter>) -> String {
        let mut basic_fields: Vec<String> = vec![
            format!("`{}`", DOCUMENT_ID_FIELD),
            format!("`{}`", DOCUMENT_CREATED_AT_FIELD),
            format!("`{}`", DOCUMENT_UPDATED_AT_FIELD),
        ];
        let fields: Vec<String> = T::schema().field_names.iter().map(|f| format!("`{}`", f)).collect();
        basic_fields.extend(fields);
        match &filter_option {
            Some(filter) => {
                let filter_sql = filter.to_sql();
                if filter_sql.is_empty() {
                    format!(
                        "SELECT {} FROM `{}`",
                        basic_fields.join(", "),
                        T::schema().collection_name
                    )
                } else if filter.conditions.is_empty() {
                    format!(
                        "SELECT {} FROM `{}` {}",
                        basic_fields.join(", "),
                        T::schema().collection_name,
                        filter_sql
                    )
                } else {
                    format!(
                        "SELECT {} FROM `{}` WHERE {}",
                        basic_fields.join(", "),
                        T::schema().collection_name,
                        filter_sql
                    )
                }
            }
            None => format!(
                "SELECT {} FROM `{}`",
                basic_fields.join(", "),
                T::schema().collection_name
            ),
        }
    }
}
