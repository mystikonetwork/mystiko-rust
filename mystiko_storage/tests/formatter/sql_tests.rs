use mystiko_storage::column::{Column, ColumnType, ColumnValue, IndexColumns, UniqueColumns};
use mystiko_storage::document::{Document, DocumentData};
use mystiko_storage::filter::{Condition, ConditionOperator, Order, QueryFilter, QueryFilterBuilder, SubFilter};
use mystiko_storage::formatter::sql::SqlStatementFormatter;
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::migration::types::{
    AddColumnMigration, AddIndexMigration, DropColumnMigration, Migration, RenameColumnMigration,
};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::{BigInt, BigUint};
use std::time::SystemTime;

#[derive(CollectionBuilder, Debug, Clone, PartialEq)]
#[collection(uniques = uniques(), indexes = indexes())]
pub struct TestDocument {
    pub field1: bool,
    pub field2: Option<bool>,
    pub field3: char,
    pub field4: Option<char>,
    pub field5: u8,
    pub field6: Option<u8>,
    pub field7: u16,
    pub field8: Option<u16>,
    pub field9: u32,
    pub field10: Option<u32>,
    pub field11: u64,
    pub field12: Option<u64>,
    pub field13: u128,
    pub field14: Option<u128>,
    pub field15: usize,
    pub field16: Option<usize>,
    pub field17: i8,
    pub field18: Option<i8>,
    pub field19: i16,
    pub field20: Option<i16>,
    pub field21: i32,
    pub field22: Option<i32>,
    pub field23: i64,
    pub field24: Option<i64>,
    pub field25: i128,
    pub field26: Option<i128>,
    pub field27: isize,
    pub field28: Option<isize>,
    pub field29: f32,
    pub field30: Option<f32>,
    pub field31: f64,
    pub field32: Option<f64>,
    #[column(length_limit = 64)]
    pub field33: String,
    pub field34: Option<String>,
    pub field35: BigInt,
    #[column(length_limit = 128)]
    pub field36: Option<BigInt>,
    pub field37: BigUint,
    pub field38: Option<BigUint>,
    #[column(length_limit = 256)]
    pub field39: Vec<u8>,
    pub field40: Option<Vec<u8>>,
}

#[test]
fn test_format_insert() {
    let formatter = SqlStatementFormatter::default();
    let document1 = create_test_document("1", true);
    let statement1 = formatter.format_insert(&document1);
    assert_eq!(
        statement1.statement,
        "INSERT INTO `test_documents` \
     (`id`, `created_at`, `updated_at`, \
     `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
     `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
     `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
     `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
     `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40`) VALUES \
     (?, ?, ?, \
     ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, \
     NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, \
     NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL, ?, NULL)"
    );
    assert_eq!(statement1.column_values.len(), 23);
    let document2 = create_test_document("2", false);
    let statement2 = formatter.format_insert(&document2);
    assert_eq!(
        statement2.statement,
        "INSERT INTO `test_documents` \
     (`id`, `created_at`, `updated_at`, \
     `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
     `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
     `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
     `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
     `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40`) VALUES \
     (?, ?, ?, \
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, \
     ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );
    assert_eq!(statement2.column_values.len(), 43);
    let statements = formatter.format_insert_batch(&vec![document1, document2]);
    assert_eq!(statements, vec![statement1, statement2]);
}

#[test]
fn test_format_update() {
    let formatter = SqlStatementFormatter::default();
    let document1 = create_test_document("1", true);
    let statement1 = formatter.format_update(&document1);
    assert_eq!(
        statement1.statement,
        "UPDATE `test_documents` \
        SET `updated_at` = ?, \
        `field1` = ?, `field2` = NULL, `field3` = ?, `field4` = NULL, \
        `field5` = ?, `field6` = NULL, `field7` = ?, `field8` = NULL, \
        `field9` = ?, `field10` = NULL, `field11` = ?, `field12` = NULL, \
        `field13` = ?, `field14` = NULL, `field15` = ?, `field16` = NULL, \
        `field17` = ?, `field18` = NULL, `field19` = ?, `field20` = NULL, \
        `field21` = ?, `field22` = NULL, `field23` = ?, `field24` = NULL, \
        `field25` = ?, `field26` = NULL, `field27` = ?, `field28` = NULL, \
        `field29` = ?, `field30` = NULL, `field31` = ?, `field32` = NULL, \
        `field33` = ?, `field34` = NULL, `field35` = ?, `field36` = NULL, \
        `field37` = ?, `field38` = NULL, `field39` = ?, `field40` = NULL \
        WHERE `id` = ?"
    );
    assert_eq!(statement1.column_values.len(), 22);
    let document2 = create_test_document("2", false);
    let statement2 = formatter.format_update(&document2);
    assert_eq!(
        statement2.statement,
        "UPDATE `test_documents` \
        SET `updated_at` = ?, \
        `field1` = ?, `field2` = ?, `field3` = ?, `field4` = ?, \
        `field5` = ?, `field6` = ?, `field7` = ?, `field8` = ?, \
        `field9` = ?, `field10` = ?, `field11` = ?, `field12` = ?, \
        `field13` = ?, `field14` = ?, `field15` = ?, `field16` = ?, \
        `field17` = ?, `field18` = ?, `field19` = ?, `field20` = ?, \
        `field21` = ?, `field22` = ?, `field23` = ?, `field24` = ?, \
        `field25` = ?, `field26` = ?, `field27` = ?, `field28` = ?, \
        `field29` = ?, `field30` = ?, `field31` = ?, `field32` = ?, \
        `field33` = ?, `field34` = ?, `field35` = ?, `field36` = ?, \
        `field37` = ?, `field38` = ?, `field39` = ?, `field40` = ? \
        WHERE `id` = ?"
    );
    assert_eq!(statement2.column_values.len(), 42);
    let statements = formatter.format_update_batch(&vec![document1, document2]);
    assert_eq!(statements, vec![statement1, statement2]);
}

#[test]
fn test_format_delete() {
    let formatter = SqlStatementFormatter::default();
    let document1 = create_test_document("1", true);
    let statement1 = formatter.format_delete(&document1);
    assert_eq!(statement1.statement, "DELETE FROM `test_documents` WHERE `id` = ?");
    assert_eq!(statement1.column_values, vec![ColumnValue::String("1".to_string())]);
    let document2 = create_test_document("2", false);
    let statement2 = formatter.format_delete(&document2);
    assert_eq!(statement2.statement, "DELETE FROM `test_documents` WHERE `id` = ?");
    assert_eq!(statement2.column_values, vec![ColumnValue::String("2".to_string())]);
    let statements = formatter.format_delete_batch(&vec![document1, document2]);
    assert_eq!(statements, vec![statement1, statement2]);
}

#[test]
fn test_format_delete_by_filter() {
    let formatter = SqlStatementFormatter::default();
    let statement1 = formatter.format_delete_by_filter::<TestDocument, QueryFilter>(None);
    assert_eq!(statement1.statement, "DELETE FROM `test_documents`");
    assert!(statement1.column_values.is_empty());
    let statement2 =
        formatter.format_delete_by_filter::<TestDocument, _>(Some(SubFilter::equal(TestDocumentColumn::Field5, 1u8)));
    assert_eq!(statement2.statement, "DELETE FROM `test_documents` WHERE `field5` = ?");
    assert_eq!(statement2.column_values, vec![ColumnValue::U8(1)]);
    let statement3 = formatter.format_delete_by_filter::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement3.statement,
        "DELETE FROM `test_documents` ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert!(statement3.column_values.is_empty());
    let statement4 = formatter.format_delete_by_filter::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .filter(SubFilter::equal(TestDocumentColumn::Field5, 1u8).into())
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement4.statement,
        "DELETE FROM `test_documents` WHERE `field5` = ? ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert_eq!(statement4.column_values, vec![ColumnValue::U8(1)]);
}

#[test]
fn test_format_count() {
    let formatter = SqlStatementFormatter::default();
    let statement1 = formatter.format_count::<TestDocument, QueryFilter>(None);
    assert_eq!(statement1.statement.statement, "SELECT COUNT(*) FROM `test_documents`");
    let statement1 = formatter.format_count::<TestDocument, _>(Some(QueryFilterBuilder::new().build()));
    assert_eq!(statement1.statement.statement, "SELECT COUNT(*) FROM `test_documents`");
    assert!(statement1.statement.column_values.is_empty());
    let statement2 = formatter.format_count::<TestDocument, _>(Some(SubFilter::equal(TestDocumentColumn::Field5, 1u8)));
    assert_eq!(
        statement2.statement.statement,
        "SELECT COUNT(*) FROM `test_documents` WHERE `field5` = ?"
    );
    assert_eq!(statement2.statement.column_values, vec![ColumnValue::U8(1)]);
    let statement3 = formatter.format_count::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement3.statement.statement,
        "SELECT COUNT(*) FROM `test_documents` ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert!(statement3.statement.column_values.is_empty());
    let statement4 = formatter.format_count::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .filter(SubFilter::equal(TestDocumentColumn::Field5, 1u8).into())
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement4.statement.statement,
        "SELECT COUNT(*) FROM `test_documents` WHERE `field5` = ? ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert_eq!(statement4.statement.column_values, vec![ColumnValue::U8(1)]);
    let formatter = SqlStatementFormatter::builder()
        .count_mark(String::from("MY_COUNT(*)"))
        .build();
    let statement5 = formatter.format_count::<TestDocument, QueryFilter>(None);
    assert_eq!(statement5.count_column, "MY_COUNT(*)");
    assert_eq!(
        statement5.statement.statement,
        "SELECT MY_COUNT(*) FROM `test_documents`"
    );
}

#[test]
fn test_format_find() {
    let formatter = SqlStatementFormatter::default();
    let statement1 = formatter.format_find::<TestDocument, QueryFilter>(None);
    assert_eq!(
        statement1.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` FROM `test_documents`"
    );
    let statement1 = formatter.format_find::<TestDocument, QueryFilter>(Some(QueryFilterBuilder::new().build()));
    assert_eq!(
        statement1.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` FROM `test_documents`"
    );
    assert!(statement1.column_values.is_empty());
    let statement2 = formatter.format_find::<TestDocument, _>(Some(SubFilter::equal(TestDocumentColumn::Field5, 1u8)));
    assert_eq!(
        statement2.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` \
        FROM `test_documents` WHERE `field5` = ?"
    );
    assert_eq!(statement2.column_values, vec![ColumnValue::U8(1)]);
    let statement3 = formatter.format_find::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement3.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` \
        FROM `test_documents` ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert!(statement3.column_values.is_empty());
    let statement4 = formatter.format_find::<TestDocument, _>(Some(
        QueryFilterBuilder::new()
            .filter(SubFilter::equal(TestDocumentColumn::Field5, 1u8).into())
            .order_by(TestDocumentColumn::Field1, Order::DESC)
            .limit(1)
            .offset(2)
            .build(),
    ));
    assert_eq!(
        statement4.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` \
        FROM `test_documents` WHERE `field5` = ? ORDER BY `field1` DESC LIMIT 1 OFFSET 2"
    );
    assert_eq!(statement4.column_values, vec![ColumnValue::U8(1)]);
    let formatter = SqlStatementFormatter::builder().value_mark(String::from("$")).build();
    let statement5 = formatter.format_find::<TestDocument, _>(Some(SubFilter::equal(TestDocumentColumn::Field5, 1u8)));
    assert_eq!(
        statement5.statement,
        "SELECT `id`, `created_at`, `updated_at`, \
        `field1`, `field2`, `field3`, `field4`, `field5`, `field6`, `field7`, `field8`, \
        `field9`, `field10`, `field11`, `field12`, `field13`, `field14`, `field15`, `field16`, \
        `field17`, `field18`, `field19`, `field20`, `field21`, `field22`, `field23`, `field24`, \
        `field25`, `field26`, `field27`, `field28`, `field29`, `field30`, `field31`, `field32`, \
        `field33`, `field34`, `field35`, `field36`, `field37`, `field38`, `field39`, `field40` \
        FROM `test_documents` WHERE `field5` = $"
    );
}

#[test]
fn test_format_create_collection_migration() {
    let formatter = SqlStatementFormatter::default();
    let statements = formatter.format_migration_batch::<TestDocument>(&Document::<TestDocument>::migrations());
    assert_eq!(statements.len(), 3);
    let statement1 = statements.get(0).unwrap();
    assert_eq!(
        statement1.statement,
        "CREATE TABLE IF NOT EXISTS `test_documents` (\
    `id` VARCHAR(64) NOT NULL PRIMARY KEY, \
    `created_at` BIGINT NOT NULL, \
    `updated_at` BIGINT NOT NULL, \
    `field1` TINYINT NOT NULL, \
    `field2` TINYINT, \
    `field3` VARCHAR(1) NOT NULL, \
    `field4` VARCHAR(1), \
    `field5` TINYINT NOT NULL, \
    `field6` TINYINT, \
    `field7` SMALLINT NOT NULL, \
    `field8` SMALLINT, \
    `field9` INT NOT NULL, \
    `field10` INT, \
    `field11` VARCHAR(20) NOT NULL, \
    `field12` VARCHAR(20), \
    `field13` VARCHAR(40) NOT NULL, \
    `field14` VARCHAR(40), \
    `field15` VARCHAR(20) NOT NULL, \
    `field16` VARCHAR(20), \
    `field17` TINYINT NOT NULL, \
    `field18` TINYINT, \
    `field19` SMALLINT NOT NULL, \
    `field20` SMALLINT, \
    `field21` INT NOT NULL, \
    `field22` INT, \
    `field23` BIGINT NOT NULL, \
    `field24` BIGINT, \
    `field25` VARCHAR(40) NOT NULL, \
    `field26` VARCHAR(40), \
    `field27` BIGINT NOT NULL, \
    `field28` BIGINT, \
    `field29` FLOAT NOT NULL, \
    `field30` FLOAT, \
    `field31` DOUBLE NOT NULL, \
    `field32` DOUBLE, \
    `field33` VARCHAR(64) NOT NULL, \
    `field34` TEXT, \
    `field35` TEXT NOT NULL, \
    `field36` VARCHAR(128), \
    `field37` TEXT NOT NULL, \
    `field38` TEXT, \
    `field39` VARCHAR(256) NOT NULL, \
    `field40` TEXT, \
    CONSTRAINT `my_unique_1` UNIQUE (`field1`), \
    CONSTRAINT `test_documents_unique_field3_field5` UNIQUE (`field3`, `field5`)\
    )"
    );
    assert!(statement1.column_values.is_empty());
    let statement2 = statements.get(1).unwrap();
    assert_eq!(
        statement2.statement,
        "CREATE INDEX `my_index_1` ON `test_documents` (`field7`)"
    );
    assert!(statement2.column_values.is_empty());
    let statement3 = statements.get(2).unwrap();
    assert_eq!(
        statement3.statement,
        "CREATE INDEX `test_documents_index_field9_field11` ON `test_documents` (`field9`, `field11`)"
    );
    assert!(statement3.column_values.is_empty());
}

#[test]
fn test_format_add_index_migration() {
    let formatter = SqlStatementFormatter::default();
    let migration: Migration = AddIndexMigration::builder()
        .index_name("my_index_1")
        .column_names(vec![
            TestDocumentColumn::Field7.to_string(),
            TestDocumentColumn::Field9.to_string(),
        ])
        .build()
        .into();
    let statements = formatter.format_migration::<TestDocument>(&migration);
    assert_eq!(statements.len(), 1);
    let statement = statements.get(0).unwrap();
    assert_eq!(
        statement.statement,
        "CREATE INDEX `my_index_1` ON `test_documents` (`field7`, `field9`)"
    );
    assert!(statement.column_values.is_empty());
}

#[test]
fn test_format_add_column_migration() {
    let formatter = SqlStatementFormatter::default();
    let migration: Migration = AddColumnMigration::builder()
        .column(
            Column::builder()
                .column_type(ColumnType::String)
                .column_name("field41")
                .length_limit(Some(128))
                .nullable(false)
                .build(),
        )
        .build()
        .into();
    let statements = formatter.format_migration::<TestDocument>(&migration);
    assert_eq!(statements.len(), 1);
    let statement = statements.get(0).unwrap();
    assert_eq!(
        statement.statement,
        "ALTER TABLE `test_documents` ADD COLUMN `field41` VARCHAR(128) NOT NULL"
    );
    assert!(statement.column_values.is_empty());
}

#[test]
fn test_format_drop_column_migration() {
    let formatter = SqlStatementFormatter::default();
    let migration: Migration = DropColumnMigration::builder().column_name("field1").build().into();
    let statements = formatter.format_migration::<TestDocument>(&migration);
    assert_eq!(statements.len(), 1);
    let statement = statements.get(0).unwrap();
    assert_eq!(statement.statement, "ALTER TABLE `test_documents` DROP COLUMN `field1`");
    assert!(statement.column_values.is_empty());
}

#[test]
fn test_format_rename_column_migration() {
    let formatter = SqlStatementFormatter::default();
    let migration: Migration = RenameColumnMigration::builder()
        .old_column_name("field1")
        .new_column_name("field41")
        .build()
        .into();
    let statements = formatter.format_migration::<TestDocument>(&migration);
    assert_eq!(statements.len(), 1);
    let statement = statements.get(0).unwrap();
    assert_eq!(
        statement.statement,
        "ALTER TABLE `test_documents` RENAME COLUMN `field1` TO `field41`"
    );
    assert!(statement.column_values.is_empty());
}

#[test]
fn test_format_sub_filter() {
    let formatter = SqlStatementFormatter::default();
    let mut filter: QueryFilter = SubFilter::equal(TestDocumentColumn::Field1, true).into();
    let mut statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field1` = ?");
    assert_eq!(statement.column_values, vec![ColumnValue::Bool(true)]);
    filter = SubFilter::not_equal(TestDocumentColumn::Field1, true).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field1` != ?");
    assert_eq!(statement.column_values, vec![ColumnValue::Bool(true)]);
    filter = SubFilter::less(TestDocumentColumn::Field5, 1u8).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field5` < ?");
    assert_eq!(statement.column_values, vec![ColumnValue::U8(1)]);
    filter = SubFilter::less_equal(TestDocumentColumn::Field5, 1u8).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field5` <= ?");
    assert_eq!(statement.column_values, vec![ColumnValue::U8(1)]);
    filter = SubFilter::greater(TestDocumentColumn::Field5, 1u8).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field5` > ?");
    assert_eq!(statement.column_values, vec![ColumnValue::U8(1)]);
    filter = SubFilter::greater_equal(TestDocumentColumn::Field5, 1u8).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field5` >= ?");
    assert_eq!(statement.column_values, vec![ColumnValue::U8(1)]);
    filter = SubFilter::between_and(TestDocumentColumn::Field7, 1u16, 10u16).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field7` BETWEEN ? AND ?"
    );
    assert_eq!(statement.column_values, vec![ColumnValue::U16(1), ColumnValue::U16(10)]);
    filter = SubFilter::in_list(TestDocumentColumn::Field7, vec![1u16, 2u16, 3u16]).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field7` IN (?, ?, ?)"
    );
    assert_eq!(
        statement.column_values,
        vec![ColumnValue::U16(1), ColumnValue::U16(2), ColumnValue::U16(3)]
    );
    filter = SubFilter::is_null(TestDocumentColumn::Field2).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field2` IS NULL"
    );
    assert!(statement.column_values.is_empty());
    filter = SubFilter::is_not_null(TestDocumentColumn::Field2).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field2` IS NOT NULL"
    );
    assert!(statement.column_values.is_empty());
}

#[test]
fn test_format_condition() {
    let formatter = SqlStatementFormatter::default();
    let mut condition = Condition {
        operator: ConditionOperator::AND,
        sub_filters: vec![],
    };
    let mut statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(condition));
    assert_eq!(statement.statement, "DELETE FROM `test_documents`");
    assert!(statement.column_values.is_empty());
    condition = SubFilter::equal(TestDocumentColumn::Field1, true).into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(condition));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field1` = ?");
    assert_eq!(statement.column_values, vec![ColumnValue::Bool(true)]);
    condition = vec![
        SubFilter::equal(TestDocumentColumn::Field1, true),
        SubFilter::equal(TestDocumentColumn::Field3, 'a'),
    ]
    .into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(condition));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field1` = ? AND `field3` = ?"
    );
    assert_eq!(
        statement.column_values,
        vec![ColumnValue::Bool(true), ColumnValue::Char('a')]
    );
    condition = (
        vec![
            SubFilter::equal(TestDocumentColumn::Field1, true),
            SubFilter::equal(TestDocumentColumn::Field3, 'a'),
        ],
        ConditionOperator::OR,
    )
        .into();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(condition));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field1` = ? OR `field3` = ?"
    );
    assert_eq!(
        statement.column_values,
        vec![ColumnValue::Bool(true), ColumnValue::Char('a')]
    );
}

#[test]
fn test_format_query_filter() {
    let formatter = SqlStatementFormatter::default();
    let mut filter = QueryFilterBuilder::new().build();
    let mut statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents`");
    assert!(statement.column_values.is_empty());
    filter = QueryFilterBuilder::new().offset(10).build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents`");
    assert!(statement.column_values.is_empty());
    filter = QueryFilterBuilder::new().limit(10).offset(20).build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` LIMIT 10 OFFSET 20");
    assert!(statement.column_values.is_empty());
    filter = QueryFilterBuilder::new()
        .order_by(TestDocumentColumn::Field1, Order::DESC)
        .build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` ORDER BY `field1` DESC"
    );
    assert!(statement.column_values.is_empty());
    filter = QueryFilterBuilder::new()
        .order_by_multiple(vec![TestDocumentColumn::Field1, TestDocumentColumn::Field3], Order::ASC)
        .limit(10)
        .offset(20)
        .build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` ORDER BY `field1`, `field3` ASC LIMIT 10 OFFSET 20"
    );
    filter = QueryFilterBuilder::new()
        .filter(SubFilter::equal(TestDocumentColumn::Field1, true).into())
        .build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(statement.statement, "DELETE FROM `test_documents` WHERE `field1` = ?");
    assert_eq!(statement.column_values, vec![ColumnValue::Bool(true)]);
    filter = QueryFilterBuilder::new()
        .filter(
            vec![
                SubFilter::equal(TestDocumentColumn::Field1, true),
                SubFilter::equal(TestDocumentColumn::Field3, 'a'),
            ]
            .into(),
        )
        .build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field1` = ? AND `field3` = ?"
    );
    assert_eq!(
        statement.column_values,
        vec![ColumnValue::Bool(true), ColumnValue::Char('a')]
    );
    filter = QueryFilterBuilder::new()
        .filters(vec![
            SubFilter::equal(TestDocumentColumn::Field1, true).into(),
            vec![
                SubFilter::equal(TestDocumentColumn::Field3, 'a'),
                SubFilter::is_not_null(TestDocumentColumn::Field4),
            ]
            .into(),
        ])
        .filter_operator(ConditionOperator::OR)
        .build();
    statement = formatter.format_delete_by_filter::<TestDocument, _>(Some(filter));
    assert_eq!(
        statement.statement,
        "DELETE FROM `test_documents` WHERE `field1` = ? OR (`field3` = ? AND `field4` IS NOT NULL)"
    );
    assert_eq!(
        statement.column_values,
        vec![ColumnValue::Bool(true), ColumnValue::Char('a')]
    );
}

fn create_test_document(id: &str, has_null: bool) -> Document<TestDocument> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    Document {
        id: id.to_string(),
        created_at: now,
        updated_at: now,
        data: TestDocument {
            field1: false,
            field2: if has_null { None } else { Some(true) },
            field3: 'a',
            field4: if has_null { None } else { Some('b') },
            field5: 1,
            field6: if has_null { None } else { Some(2) },
            field7: 3,
            field8: if has_null { None } else { Some(4) },
            field9: 5,
            field10: if has_null { None } else { Some(6) },
            field11: 7,
            field12: if has_null { None } else { Some(8) },
            field13: 9,
            field14: if has_null { None } else { Some(10) },
            field15: 11,
            field16: if has_null { None } else { Some(12) },
            field17: 13,
            field18: if has_null { None } else { Some(14) },
            field19: 15,
            field20: if has_null { None } else { Some(16) },
            field21: 17,
            field22: if has_null { None } else { Some(18) },
            field23: 19,
            field24: if has_null { None } else { Some(20) },
            field25: 21,
            field26: if has_null { None } else { Some(22) },
            field27: 23,
            field28: if has_null { None } else { Some(24) },
            field29: 25.0,
            field30: if has_null { None } else { Some(26.0) },
            field31: 27.0,
            field32: if has_null { None } else { Some(28.0) },
            field33: String::from("29"),
            field34: if has_null { None } else { Some(String::from("30")) },
            field35: BigInt::from(31),
            field36: if has_null { None } else { Some(BigInt::from(32)) },
            field37: BigUint::from(33u32),
            field38: if has_null { None } else { Some(BigUint::from(34u32)) },
            field39: vec![35, 36],
            field40: if has_null { None } else { Some(vec![37, 38]) },
        },
    }
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        UniqueColumns::builder()
            .unique_name("my_unique_1")
            .column_names(vec![TestDocumentColumn::Field1.to_string()])
            .build(),
        vec![TestDocumentColumn::Field3, TestDocumentColumn::Field5].into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        IndexColumns::builder()
            .index_name("my_index_1")
            .column_names(vec![TestDocumentColumn::Field7.to_string()])
            .build(),
        vec![TestDocumentColumn::Field9, TestDocumentColumn::Field11].into(),
    ]
}
