use crate::column::{Column, ColumnType, ColumnValue};
use crate::document::{Document, DocumentColumn, DocumentData};
use crate::filter::{Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter, SubFilterOperator};
use crate::formatter::types::{CountStatement, Statement, StatementFormatter};
use crate::migration::types::{
    AddColumnMigration, AddIndexMigration, CreateCollectionMigration, DropColumnMigration, Migration,
    RenameCollectionMigration, RenameColumnMigration,
};
use typed_builder::TypedBuilder;

const DEFAULT_VALUE_MARK: &str = "?";
const DEFAULT_COUNT_MARK: &str = "COUNT(*)";

#[derive(Clone, Debug, Default, TypedBuilder)]
pub struct SqlStatementFormatter {
    #[builder(default, setter(strip_option))]
    value_mark: Option<String>,
    #[builder(default, setter(strip_option))]
    count_mark: Option<String>,
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

    fn format_delete_by_filter<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> Statement {
        match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter(query_filter);
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
        }
    }

    fn format_count<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> CountStatement {
        let statement = match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter(query_filter);
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
        CountStatement::new(self.count_mark(), statement)
    }

    fn format_find<T: DocumentData, Q: Into<QueryFilter>>(&self, filter_option: Option<Q>) -> Statement {
        let fields = Document::<T>::columns()
            .iter()
            .map(|column| format!("`{}`", &column.column_name))
            .collect::<Vec<String>>();
        match filter_option {
            Some(filter) => {
                let query_filter: QueryFilter = filter.into();
                let no_condition = query_filter.conditions.is_empty();
                let filter_statement = self.format_query_filter(query_filter);
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
        }
    }

    fn format_migration(&self, migration: &Migration) -> Vec<Statement> {
        match migration {
            Migration::CreateCollection(migration) => format_create_collection_migration(migration),
            Migration::AddIndex(migration) => vec![format_add_index_migration(migration)],
            Migration::AddColumn(migration) => vec![format_add_column_migration(migration)],
            Migration::DropColumn(migration) => vec![format_drop_column_migration(migration)],
            Migration::RenameCollection(migration) => vec![format_rename_collection_migration(migration)],
            Migration::RenameColumn(migration) => vec![format_rename_column_migration(migration)],
        }
    }
}

impl SqlStatementFormatter {
    fn value_mark(&self) -> String {
        self.value_mark.clone().unwrap_or(DEFAULT_VALUE_MARK.into())
    }

    fn count_mark(&self) -> String {
        self.count_mark.clone().unwrap_or(DEFAULT_COUNT_MARK.into())
    }

    fn format_query_filter(&self, filter: QueryFilter) -> Statement {
        let mut statements: Vec<String> = Vec::new();
        let mut condition_statements: Vec<String> = Vec::new();
        let mut column_values: Vec<ColumnValue> = Vec::new();
        let conditions_length = filter.conditions.len();
        for condition in filter.conditions {
            let sub_filters_length = condition.sub_filters.len();
            let condition_statement = self.format_condition(condition);
            if !condition_statement.statement.is_empty() {
                if conditions_length <= 1 || sub_filters_length <= 1 {
                    condition_statements.push(condition_statement.statement);
                } else {
                    condition_statements.push(format!("({})", condition_statement.statement));
                }
                column_values.extend(condition_statement.column_values);
            }
        }
        statements.push(condition_statements.join(format_condition_operator(&filter.conditions_operator)));
        if let Some(order_by) = &filter.order_by {
            if !order_by.columns.is_empty() {
                statements.push(format_order_by(order_by));
            }
        }
        if let Some(limit) = filter.limit {
            statements.push(format!("LIMIT {}", limit));
            if let Some(offset) = filter.offset {
                statements.push(format!("OFFSET {}", offset));
            }
        }
        Statement::new(statements.join(" "), column_values)
    }

    fn format_condition(&self, condition: Condition) -> Statement {
        let sub_filter_statements = condition
            .sub_filters
            .into_iter()
            .map(|sub_filter| self.format_sub_filter(sub_filter))
            .collect::<Vec<Statement>>();
        Statement::new(
            format!(
                "({})",
                sub_filter_statements
                    .iter()
                    .map(|statement| statement.statement.clone())
                    .collect::<Vec<String>>()
                    .join(format_condition_operator(&condition.operator))
            ),
            sub_filter_statements
                .into_iter()
                .flat_map(|statement| statement.column_values)
                .collect::<Vec<ColumnValue>>(),
        )
    }

    fn format_sub_filter(&self, filter: SubFilter) -> Statement {
        match filter.operator {
            SubFilterOperator::Equal => {
                Statement::new(format!("`{}` = {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::NotEqual => {
                Statement::new(format!("`{}` != {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::Greater => {
                Statement::new(format!("`{}` > {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::GreaterEqual => {
                Statement::new(format!("`{}` >= {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::Less => {
                Statement::new(format!("`{}` < {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::LessEqual => {
                Statement::new(format!("`{}` <= {}", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::In => {
                Statement::new(format!("`{}` IN ({})", filter.column, self.value_mark()), filter.values)
            }
            SubFilterOperator::BetweenAnd => Statement::new(
                format!(
                    "`{}` BETWEEN {} AND {}",
                    filter.column,
                    self.value_mark(),
                    self.value_mark()
                ),
                filter.values,
            ),
            SubFilterOperator::IsNull => Statement::new(format!("`{}` IS NULL", filter.column), vec![]),
            SubFilterOperator::IsNotNull => Statement::new(format!("`{}` IS NOT NULL", filter.column), vec![]),
        }
    }
}

fn format_condition_operator(operator: &ConditionOperator) -> &str {
    match operator {
        ConditionOperator::AND => " AND ",
        ConditionOperator::OR => " OR ",
    }
}

fn format_order_by(order_by: &OrderBy) -> String {
    let column_names = order_by
        .columns
        .iter()
        .map(|column| format!("`{}`", column))
        .collect::<Vec<String>>();
    let order = match order_by.order {
        Order::ASC => "ASC",
        Order::DESC => "DESC",
    };
    format!("ORDER BY {} {}", column_names.join(", "), order)
}

fn format_create_collection_migration(migration: &CreateCollectionMigration) -> Vec<Statement> {
    let mut columns_sql: Vec<String> = Vec::new();
    for column in migration.columns.iter() {
        columns_sql.push(format_column_sql(column));
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
            unique_columns.unique_name,
            unique_columns_names.join(", ")
        ));
    }
    columns_sql.extend(unique_columns_sql);
    let mut statements: Vec<Statement> = vec![Statement::new(
        format!(
            "CREATE TABLE `{}` ({})",
            migration.collection_name,
            columns_sql.join(", ")
        ),
        vec![],
    )];
    for index_columns in migration.index_columns.iter() {
        statements.push(format_add_index_migration(
            &AddIndexMigration::builder()
                .collection_name(migration.collection_name.clone())
                .index_name(index_columns.index_name.clone())
                .column_names(index_columns.column_names.clone())
                .build(),
        ));
    }
    statements
}

fn format_add_index_migration(migration: &AddIndexMigration) -> Statement {
    let index_columns = migration
        .column_names
        .iter()
        .map(|column| format!("`{}`", column))
        .collect::<Vec<String>>();
    Statement::new(
        format!(
            "CREATE INDEX `{}` ON `{}` ({})",
            migration.index_name,
            migration.collection_name,
            index_columns.join(", ")
        ),
        vec![],
    )
}

fn format_add_column_migration(migration: &AddColumnMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` ADD COLUMN {}",
            migration.collection_name,
            format_column_sql(&migration.column)
        ),
        vec![],
    )
}

fn format_drop_column_migration(migration: &DropColumnMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` DROP COLUMN `{}`",
            migration.collection_name, migration.column_name
        ),
        vec![],
    )
}

fn format_rename_collection_migration(migration: &RenameCollectionMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` RENAME TO `{}`",
            migration.old_collection_name, migration.new_collection_name
        ),
        vec![],
    )
}

fn format_rename_column_migration(migration: &RenameColumnMigration) -> Statement {
    Statement::new(
        format!(
            "ALTER TABLE `{}` RENAME COLUMN `{}` TO `{}`",
            migration.collection_name, migration.old_column_name, migration.new_column_name
        ),
        vec![],
    )
}

fn format_column_sql(column: &Column) -> String {
    let mut column_sql = String::new();
    column_sql.push_str(&format!("`{}` {}", column.column_name, get_column_sql_type(column)));
    if !column.nullable {
        column_sql.push_str(" NOT NULL");
    }
    if column.is_primary_key {
        column_sql.push_str(" PRIMARY KEY");
    }
    column_sql
}

fn get_column_sql_type(column: &Column) -> String {
    match column.column_type {
        ColumnType::Bool => "TINYINT".into(),
        ColumnType::Char => "VARCHAR(1)".into(),
        ColumnType::I8 => "TINYINT".into(),
        ColumnType::I16 => "SMALLINT".into(),
        ColumnType::I32 => "INT".into(),
        ColumnType::I64 => "BIGINT".into(),
        ColumnType::I128 => "VARCHAR(40)".into(),
        ColumnType::ISize => "BIGINT".into(),
        ColumnType::U8 => "TINYINT".into(),
        ColumnType::U16 => "SMALLINT".into(),
        ColumnType::U32 => "INT".into(),
        ColumnType::U64 => "VARCHAR(20)".into(),
        ColumnType::U128 => "VARCHAR(40)".into(),
        ColumnType::USize => "VARCHAR(20)".into(),
        ColumnType::F32 => "FLOAT".into(),
        ColumnType::F64 => "DOUBLE".into(),
        ColumnType::String => varchar_or_text_sql_type(&column.length_limit),
        ColumnType::Json => varchar_or_text_sql_type(&column.length_limit),
    }
}

fn varchar_or_text_sql_type(length_limit: &Option<u64>) -> String {
    if let Some(length_limit) = length_limit {
        format!("VARCHAR({})", length_limit)
    } else {
        "TEXT".into()
    }
}
