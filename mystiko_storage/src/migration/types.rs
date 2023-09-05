use crate::{Column, IndexColumns, UniqueColumns};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCollectionMigration {
    pub columns: Vec<Column>,
    #[builder(default)]
    pub unique_columns: Vec<UniqueColumns>,
    #[builder(default)]
    pub index_columns: Vec<IndexColumns>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct AddIndexMigration {
    #[builder(default, setter(into, strip_option))]
    pub index_name: Option<String>,
    #[builder(setter(into))]
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct AddColumnMigration {
    pub column: Column,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct DropColumnMigration {
    #[builder(setter(into))]
    pub column_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct RenameColumnMigration {
    #[builder(setter(into))]
    pub old_column_name: String,
    #[builder(setter(into))]
    pub new_column_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Migration {
    CreateCollection(CreateCollectionMigration),
    AddIndex(AddIndexMigration),
    AddColumn(AddColumnMigration),
    DropColumn(DropColumnMigration),
    RenameColumn(RenameColumnMigration),
}

impl From<CreateCollectionMigration> for Migration {
    fn from(value: CreateCollectionMigration) -> Self {
        Self::CreateCollection(value)
    }
}

impl From<AddIndexMigration> for Migration {
    fn from(value: AddIndexMigration) -> Self {
        Self::AddIndex(value)
    }
}

impl From<AddColumnMigration> for Migration {
    fn from(value: AddColumnMigration) -> Self {
        Self::AddColumn(value)
    }
}

impl From<DropColumnMigration> for Migration {
    fn from(value: DropColumnMigration) -> Self {
        Self::DropColumn(value)
    }
}

impl From<RenameColumnMigration> for Migration {
    fn from(value: RenameColumnMigration) -> Self {
        Self::RenameColumn(value)
    }
}
