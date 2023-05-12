use crate::column::Column;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCollectionMigration {
    pub collection_name: String,
    pub columns: Vec<Column>,
    #[builder(default)]
    pub unique_columns: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct AddIndexMigration {
    pub collection_name: String,
    pub index_name: String,
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct AddColumnMigration {
    pub collection_name: String,
    pub column: Column,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct DropColumnMigration {
    pub collection_name: String,
    pub column_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct RenameCollectionMigration {
    pub old_collection_name: String,
    pub new_collection_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct RenameColumnMigration {
    pub collection_name: String,
    pub old_column_name: String,
    pub new_column_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Migration {
    CreateCollection(CreateCollectionMigration),
    AddIndex(AddIndexMigration),
    AddColumn(AddColumnMigration),
    DropColumn(DropColumnMigration),
    RenameCollection(RenameCollectionMigration),
    RenameColumn(RenameColumnMigration),
}
