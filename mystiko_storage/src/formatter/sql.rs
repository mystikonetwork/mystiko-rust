use crate::{
    AddColumnMigration, AddIndexMigration, Column, ColumnValues, CountStatement, CreateCollectionMigration, Document,
    DocumentColumn, DocumentData, DropColumnMigration, Migration, RenameColumnMigration, Statement, StatementFormatter,
    StorageError,
};
use anyhow::Result;
use mystiko_protos::storage::v1::{
    ColumnType, ColumnValue, Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter, SubFilterOperator,
};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

const DEFAULT_VALUE_MARK: &str = "?";
const DEFAULT_COUNT_MARK: &str = "COUNT(*)";

#[derive(Debug, Clone)]
pub enum SqlType {
    Sqlite,
    MySql,
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct SqlStatementFormatter {
    #[builder(default, setter(strip_option))]
    value_mark: Option<String>,
    #[builder(default, setter(strip_option))]
    count_mark: Option<String>,
    sql_type: SqlType,
}

impl StatementFormatter for SqlStatementFormatter {
    fn format_insert<T: DocumentData>(&self, doc: &Document<T>) -> Statement {
        let mut column_names: Vec<String> = Vec::new();
        let mut value_marks: Vec<String> = Vec::new();
        let mut column_values: Vec<ColumnValue> = Vec::new();
        for (column, column_value) in doc.column_values() {
            column_names.push(format!("`{}`", column.column_name));
            if let Some(column_value) = column_value {
                value_marks.push(self.value_mark());
                column_values.push(column_value);
            } else {
                value_marks.push("NULL".into());
            }
        }
        Statement::new(
            format!(
                "INSERT INTO `{}` ({}) VALUES ({})",
                T::collection_name(),
                column_names.join(", "),
                value_marks.join(", ")
            ),
            column_values,
        )
    }

    fn format_update<T: DocumentData>(&self, doc: &Document<T>) -> Statement {
        let mut column_updates: Vec<String> = vec![format!(
            "`{}` = {}",
            DocumentColumn::UpdatedAt.to_string(),
            self.value_mark()
        )];
        let mut column_values: Vec<ColumnValue> = vec![doc.updated_at.into()];
        for (column, column_value) in doc.data.column_values() {
            if let Some(column_value) = column_value {
                column_updates.push(format!("`{}` = {}", column.column_name, self.value_mark()));
                column_values.push(column_value);
            } else {
                column_updates.push(format!("`{}` = NULL", column.column_name));
            }
        }
        column_values.push(doc.id.clone().into());
        Statement::new(
            format!(
                "UPDATE `{}` SET {} WHERE `{}` = {}",
                T::collection_name(),
                column_updates.join(", "),
                DocumentColumn::Id.to_string(),
                self.value_mark()
            ),
            column_values,
        )
    }

    fn format_update_by_filter<T, Q, V>(
        &self,
        column_values: V,
        filter_option: Option<Q>,
    ) -> Result<Statement, StorageError>
    where
        T: DocumentData,
        Q: Into<QueryFilter>,
        V: Into<ColumnValues>,
    {
        let column_values = column_values.into().column_values;
        check_column_values::<T>(&column_values)?;
        let mut values = vec![];
        let updates = column_values
            .into_iter()
            .map(|(column, value)| {
                if let Some(value) = value {
                    values.push(value);
                    format!("`{}` = {}", column, self.value_mark())
                } else {
                    format!("`{}` = NULL", column)
                }
            })
            .collect::<Vec<_>>();
        let statement = match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter::<T>(query_filter)?;
                if filter_statement.statement.is_empty() {
                    Statement::new(
                        format!("UPDATE `{}` SET {}", T::collection_name(), updates.join(", ")),
                        values,
                    )
                } else if no_condition {
                    values.extend(filter_statement.column_values);
                    Statement::new(
                        format!(
                            "UPDATE `{}` SET {} {}",
                            T::collection_name(),
                            updates.join(", "),
                            filter_statement.statement
                        ),
                        values,
                    )
                } else {
                    values.extend(filter_statement.column_values);
                    Statement::new(
                        format!(
                            "UPDATE `{}` SET {} WHERE {}",
                            T::collection_name(),
                            updates.join(", "),
                            filter_statement.statement
                        ),
                        values,
                    )
                }
            }
            None => Statement::new(
                format!("UPDATE `{}` SET {}", T::collection_name(), updates.join(", ")),
                values,
            ),
        };
        Ok(statement)
    }

    fn format_delete<T: DocumentData>(&self, doc: &Document<T>) -> Statement {
        Statement::new(
            format!(
                "DELETE FROM `{}` WHERE `{}` = {}",
                T::collection_name(),
                DocumentColumn::Id.to_string(),
                self.value_mark()
            ),
            vec![doc.id.clone().into()],
        )
    }

    fn format_delete_by_filter<T: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter_option: Option<Q>,
    ) -> Result<Statement, StorageError> {
        let statement = match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter::<T>(query_filter)?;
                if filter_statement.statement.is_empty() {
                    Statement::new(format!("DELETE FROM `{}`", T::collection_name()), Vec::new())
                } else if no_condition {
                    Statement::new(
                        format!("DELETE FROM `{}` {}", T::collection_name(), filter_statement.statement),
                        filter_statement.column_values,
                    )
                } else {
                    Statement::new(
                        format!(
                            "DELETE FROM `{}` WHERE {}",
                            T::collection_name(),
                            filter_statement.statement
                        ),
                        filter_statement.column_values,
                    )
                }
            }
            None => Statement::new(format!("DELETE FROM `{}`", T::collection_name()), Vec::new()),
        };
        Ok(statement)
    }

    fn format_count<T: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter_option: Option<Q>,
    ) -> Result<CountStatement, StorageError> {
        let statement = match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter::<T>(query_filter)?;
                if filter_statement.statement.is_empty() {
                    Statement::new(
                        format!("SELECT {} FROM `{}`", self.count_mark(), T::collection_name()),
                        Vec::new(),
                    )
                } else if no_condition {
                    Statement::new(
                        format!(
                            "SELECT {} FROM `{}` {}",
                            self.count_mark(),
                            T::collection_name(),
                            filter_statement.statement
                        ),
                        filter_statement.column_values,
                    )
                } else {
                    Statement::new(
                        format!(
                            "SELECT {} FROM `{}` WHERE {}",
                            self.count_mark(),
                            T::collection_name(),
                            filter_statement.statement
                        ),
                        filter_statement.column_values,
                    )
                }
            }
            None => Statement::new(
                format!("SELECT {} FROM `{}`", self.count_mark(), T::collection_name()),
                Vec::new(),
            ),
        };
        Ok(CountStatement::new(self.count_mark(), statement))
    }

    fn format_find<T: DocumentData, Q: Into<QueryFilter>>(
        &self,
        filter_option: Option<Q>,
    ) -> Result<Statement, StorageError> {
        let fields = Document::<T>::columns()
            .iter()
            .map(|column| format!("`{}`", &column.column_name))
            .collect::<Vec<String>>();
        let statement = match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter::<T>(query_filter)?;
                if filter_statement.statement.is_empty() {
                    Statement::new(
                        format!("SELECT {} FROM `{}`", fields.join(", "), T::collection_name()),
                        Vec::new(),
                    )
                } else if no_condition {
                    Statement::new(
                        format!(
                            "SELECT {} FROM `{}` {}",
                            fields.join(", "),
                            T::collection_name(),
                            filter_statement.statement
                        ),
                        filter_statement.column_values,
                    )
                } else {
                    Statement::new(
                        format!(
                            "SELECT {} FROM `{}` WHERE {}",
                            fields.join(", "),
                            T::collection_name(),
                            filter_statement.statement
                        ),
                        filter_statement.column_values,
                    )
                }
            }
            None => Statement::new(
                format!("SELECT {} FROM `{}`", fields.join(", "), T::collection_name()),
                Vec::new(),
            ),
        };
        Ok(statement)
    }

    fn format_migration<T: DocumentData>(&self, migration: &Migration) -> Result<Vec<Statement>, StorageError> {
        let statements = match migration {
            Migration::CreateCollection(migration) => {
                format_create_collection_migration::<T>(migration, &self.sql_type)?
            }
            Migration::AddIndex(migration) => vec![format_add_index_migration::<T>(migration)],
            Migration::AddColumn(migration) => vec![format_add_column_migration::<T>(migration, &self.sql_type)?],
            Migration::DropColumn(migration) => vec![format_drop_column_migration::<T>(migration)],
            Migration::RenameColumn(migration) => vec![format_rename_column_migration::<T>(migration)],
        };
        Ok(statements)
    }
}

impl SqlStatementFormatter {
    pub fn sqlite() -> Self {
        Self::builder().sql_type(SqlType::Sqlite).build()
    }

    pub fn mysql() -> Self {
        Self::builder().sql_type(SqlType::MySql).build()
    }

    fn value_mark(&self) -> String {
        self.value_mark.clone().unwrap_or(DEFAULT_VALUE_MARK.into())
    }

    fn count_mark(&self) -> String {
        self.count_mark.clone().unwrap_or(DEFAULT_COUNT_MARK.into())
    }

    fn format_query_filter<T: DocumentData>(&self, filter: QueryFilter) -> Result<Statement, StorageError> {
        let mut statements: Vec<String> = Vec::new();
        let mut condition_statements: Vec<String> = Vec::new();
        let mut column_values: Vec<ColumnValue> = Vec::new();
        let conditions_length = filter.conditions.len();
        for condition in filter.conditions {
            let sub_filters_length = condition.sub_filters.len();
            let condition_statement = self.format_condition::<T>(condition)?;
            if !condition_statement.statement.is_empty() {
                if conditions_length <= 1 || sub_filters_length <= 1 {
                    condition_statements.push(condition_statement.statement);
                } else {
                    condition_statements.push(format!("({})", condition_statement.statement));
                }
                column_values.extend(condition_statement.column_values);
            }
        }
        if !condition_statements.is_empty() {
            statements.push(condition_statements.join(format_condition_operator(
                &ConditionOperator::from_i32(filter.conditions_operator).unwrap_or(ConditionOperator::Unspecified),
            )?));
        }
        if let Some(order_by) = &filter.order_by {
            if !order_by.columns.is_empty() {
                statements.push(format_order_by(order_by)?);
            }
        }
        if let Some(limit) = filter.limit {
            statements.push(format!("LIMIT {}", limit));
            if let Some(offset) = filter.offset {
                statements.push(format!("OFFSET {}", offset));
            }
        }
        Ok(Statement::new(statements.join(" "), column_values))
    }

    fn format_condition<T: DocumentData>(&self, condition: Condition) -> Result<Statement, StorageError> {
        let sub_filter_statements = condition
            .sub_filters
            .into_iter()
            .map(|sub_filter| self.format_sub_filter::<T>(sub_filter))
            .collect::<Result<Vec<Statement>, StorageError>>()?;
        Ok(Statement::new(
            sub_filter_statements
                .iter()
                .map(|statement| statement.statement.clone())
                .collect::<Vec<String>>()
                .join(format_condition_operator(
                    &ConditionOperator::from_i32(condition.operator).unwrap_or(ConditionOperator::Unspecified),
                )?),
            sub_filter_statements
                .into_iter()
                .flat_map(|statement| statement.column_values)
                .collect::<Vec<ColumnValue>>(),
        ))
    }

    fn format_sub_filter<T: DocumentData>(&self, filter: SubFilter) -> Result<Statement, StorageError> {
        check_filter_column_value::<T>(&filter)?;
        let operator = SubFilterOperator::from_i32(filter.operator).unwrap_or(SubFilterOperator::Unspecified);
        match operator {
            SubFilterOperator::Equal => Ok(Statement::new(
                format!("`{}` = {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::NotEqual => Ok(Statement::new(
                format!("`{}` != {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::Greater => Ok(Statement::new(
                format!("`{}` > {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::GreaterEqual => Ok(Statement::new(
                format!("`{}` >= {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::Less => Ok(Statement::new(
                format!("`{}` < {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::LessEqual => Ok(Statement::new(
                format!("`{}` <= {}", filter.column, self.value_mark()),
                filter.values,
            )),
            SubFilterOperator::In => {
                let value_marks = std::iter::repeat(self.value_mark())
                    .take(filter.values.len())
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(Statement::new(
                    format!("`{}` IN ({})", filter.column, value_marks),
                    filter.values,
                ))
            }
            SubFilterOperator::BetweenAnd => Ok(Statement::new(
                format!(
                    "`{}` BETWEEN {} AND {}",
                    filter.column,
                    self.value_mark(),
                    self.value_mark()
                ),
                filter.values,
            )),
            SubFilterOperator::IsNull => Ok(Statement::new(format!("`{}` IS NULL", filter.column), vec![])),
            SubFilterOperator::IsNotNull => Ok(Statement::new(format!("`{}` IS NOT NULL", filter.column), vec![])),
            _ => Err(StorageError::UnsupportedOperator("unspecified".to_string()))?,
        }
    }
}

fn format_condition_operator(operator: &ConditionOperator) -> Result<&str, StorageError> {
    match operator {
        ConditionOperator::Unspecified => Err(StorageError::UnsupportedOperator("unspecified".to_string()))?,
        ConditionOperator::And => Ok(" AND "),
        ConditionOperator::Or => Ok(" OR "),
    }
}

fn format_order_by(order_by: &OrderBy) -> Result<String, StorageError> {
    let column_names = order_by
        .columns
        .iter()
        .map(|column| format!("`{}`", column))
        .collect::<Vec<String>>();
    let order = match Order::from_i32(order_by.order).unwrap_or(Order::Unspecified) {
        Order::Unspecified => Err(StorageError::UnsupportedOperator("unspecified".to_string()))?,
        Order::Asc => "ASC",
        Order::Desc => "DESC",
    };
    Ok(format!("ORDER BY {} {}", column_names.join(", "), order))
}

fn format_create_collection_migration<T: DocumentData>(
    migration: &CreateCollectionMigration,
    sql_type: &SqlType,
) -> Result<Vec<Statement>, StorageError> {
    let mut columns_sql: Vec<String> = Vec::new();
    for column in migration.columns.iter() {
        columns_sql.push(format_column_sql(column, sql_type)?);
    }
    let mut unique_columns_sql: Vec<String> = Vec::new();
    for unique_columns in migration.unique_columns.iter() {
        let unique_columns_names = unique_columns
            .column_names
            .iter()
            .map(|column| format!("`{}`", column))
            .collect::<Vec<String>>();
        unique_columns_sql.push(format!(
            "CONSTRAINT `{}` UNIQUE ({})",
            unique_columns
                .unique_name
                .clone()
                .unwrap_or(default_unique_name(T::collection_name(), &unique_columns.column_names)),
            unique_columns_names.join(", ")
        ));
    }
    columns_sql.extend(unique_columns_sql);
    let mut statements: Vec<Statement> = vec![Statement::new(
        format!(
            "CREATE TABLE IF NOT EXISTS `{}` ({})",
            T::collection_name(),
            columns_sql.join(", ")
        ),
        vec![],
    )];
    for index_columns in migration.index_columns.iter() {
        statements.push(format_add_index_migration::<T>(
            &AddIndexMigration::builder()
                .index_name(
                    index_columns
                        .index_name
                        .clone()
                        .unwrap_or(default_index_name(T::collection_name(), &index_columns.column_names)),
                )
                .column_names(index_columns.column_names.clone())
                .build(),
        ));
    }
    Ok(statements)
}

fn format_add_index_migration<T: DocumentData>(migration: &AddIndexMigration) -> Statement {
    let index_columns = migration
        .column_names
        .iter()
        .map(|column| format!("`{}`", column))
        .collect::<Vec<String>>();
    Statement::new(
        format!(
            "CREATE INDEX `{}` ON `{}` ({})",
            migration
                .index_name
                .clone()
                .unwrap_or(default_index_name(T::collection_name(), &migration.column_names)),
            T::collection_name(),
            index_columns.join(", ")
        ),
        vec![],
    )
}

fn format_add_column_migration<T: DocumentData>(
    migration: &AddColumnMigration,
    sql_type: &SqlType,
) -> Result<Statement, StorageError> {
    Ok(Statement::new(
        format!(
            "ALTER TABLE `{}` ADD COLUMN {}",
            T::collection_name(),
            format_column_sql(&migration.column, sql_type)?
        ),
        vec![],
    ))
}

fn format_drop_column_migration<T: DocumentData>(migration: &DropColumnMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` DROP COLUMN `{}`",
            T::collection_name(),
            migration.column_name
        ),
        vec![],
    )
}

fn format_rename_column_migration<T: DocumentData>(migration: &RenameColumnMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` RENAME COLUMN `{}` TO `{}`",
            T::collection_name(),
            migration.old_column_name,
            migration.new_column_name
        ),
        vec![],
    )
}

fn format_column_sql(column: &Column, sql_type: &SqlType) -> Result<String, StorageError> {
    let mut column_sql = String::new();
    column_sql.push_str(&format!(
        "`{}` {}",
        column.column_name,
        get_column_sql_type(column, sql_type)?
    ));
    if !column.nullable {
        column_sql.push_str(" NOT NULL");
    }
    if column.is_primary_key {
        column_sql.push_str(" PRIMARY KEY");
    }
    Ok(column_sql)
}

fn get_column_sql_type(column: &Column, sql_type: &SqlType) -> Result<String, StorageError> {
    let column_type = column.column_type;
    let sql_type = match column.column_type {
        ColumnType::Bool => "TINYINT".into(),
        ColumnType::Char => "VARCHAR(1)".into(),
        ColumnType::I8 => "TINYINT".into(),
        ColumnType::I16 => "SMALLINT".into(),
        ColumnType::I32 => "INT".into(),
        ColumnType::I64 => "BIGINT".into(),
        ColumnType::I128 => "VARCHAR(32)".into(),
        ColumnType::ISize => "BIGINT".into(),
        ColumnType::U8 => match sql_type {
            SqlType::Sqlite => "TINYINT".into(),
            SqlType::MySql => "TINYINT UNSIGNED".into(),
        },
        ColumnType::U16 => match sql_type {
            SqlType::Sqlite => "SMALLINT".into(),
            SqlType::MySql => "SMALLINT UNSIGNED".into(),
        },
        ColumnType::U32 => match sql_type {
            SqlType::Sqlite => "INT".into(),
            SqlType::MySql => "INT UNSIGNED".into(),
        },
        ColumnType::U64 => match sql_type {
            SqlType::Sqlite => "VARCHAR(16)".into(),
            SqlType::MySql => "BIGINT UNSIGNED".into(),
        },
        ColumnType::U128 => "VARCHAR(32)".into(),
        ColumnType::USize => match sql_type {
            SqlType::Sqlite => "VARCHAR(16)".into(),
            SqlType::MySql => "BIGINT UNSIGNED".into(),
        },
        ColumnType::F32 => "FLOAT".into(),
        ColumnType::F64 => "DOUBLE".into(),
        ColumnType::String => varchar_or_text_sql_type(&column.length_limit),
        ColumnType::BigInt => varchar_or_text_sql_type(&column.length_limit),
        ColumnType::BigUint => varchar_or_text_sql_type(&column.length_limit),
        ColumnType::Json => varchar_or_text_sql_type(&column.length_limit),
        _ => Err(StorageError::UnsupportedColumnTypeError(column_type.to_string()))?,
    };
    Ok(sql_type)
}

fn varchar_or_text_sql_type(length_limit: &Option<u64>) -> String {
    if let Some(length_limit) = length_limit {
        format!("VARCHAR({})", length_limit)
    } else {
        "TEXT".into()
    }
}

fn default_unique_name(collection_name: &str, column_names: &[String]) -> String {
    format!("{}_unique_{}", collection_name, column_names.join("_"))
}

fn default_index_name(collection_name: &str, column_names: &[String]) -> String {
    format!("{}_index_{}", collection_name, column_names.join("_"))
}

fn columns_map<T: DocumentData>() -> HashMap<String, Column> {
    Document::<T>::columns()
        .into_iter()
        .map(|c| (c.column_name.clone(), c))
        .collect::<HashMap<String, Column>>()
}

fn check_filter_column_value<T: DocumentData>(filter: &SubFilter) -> Result<(), StorageError> {
    let columns = columns_map::<T>();
    if let Some(column) = columns.get(&filter.column) {
        for value in filter.values.iter() {
            let column_type = value.column_type()?;
            if column_type != column.column_type {
                return Err(StorageError::WrongColumnValueTypeError(
                    filter.column.clone(),
                    format!("{:?}", column.column_type),
                    format!("{:?}", column_type),
                ));
            }
        }
    } else {
        return Err(StorageError::NoSuchColumnError(filter.column.clone()));
    }
    Ok(())
}

fn check_column_values<T: DocumentData>(column_values: &[(String, Option<ColumnValue>)]) -> Result<(), StorageError> {
    let columns = columns_map::<T>();
    for (column_name, column_value) in column_values.iter() {
        if let Some(column) = columns.get(column_name) {
            if let Some(column_value) = column_value {
                let column_type = column_value.column_type()?;
                if column_type != column.column_type {
                    return Err(StorageError::WrongColumnValueTypeError(
                        column_name.clone(),
                        format!("{:?}", column.column_type),
                        format!("{:?}", column_type),
                    ));
                }
            } else if !column.nullable {
                return Err(StorageError::SetNullToRequiredColumnError(column_name.clone()));
            }
        } else {
            return Err(StorageError::NoSuchColumnError(column_name.clone()));
        }
    }
    Ok(())
}
