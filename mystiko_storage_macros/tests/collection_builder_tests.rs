use mystiko_storage::column::{Column, ColumnValue, IndexColumns, UniqueColumns};
use mystiko_storage::document::DocumentData;
use mystiko_storage::migration::types::{Migration, RenameColumnMigration};
use mystiko_storage_macros::CollectionBuilder;
use num_bigint::{BigInt, BigUint};

#[derive(CollectionBuilder, Clone, Debug, PartialEq)]
#[collection(name = "test_collection", uniques = uniques(), indexes = indexes())]
#[collection(migrations = migrations())]
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
    #[column(length_limit = 128)]
    pub field33: String,
    pub field34: Option<String>,
    pub field35: BigInt,
    #[column(length_limit = length_limit())]
    pub field36: Option<BigInt>,
    pub field37: BigUint,
    pub field38: Option<BigUint>,
    pub field39: Vec<u8>,
    pub field40: Option<Vec<u8>>,
    pub field41: Vec<Vec<String>>,
    pub field42_with_underscore: Option<Vec<Vec<String>>>,
}

fn length_limit() -> u64 {
    256
}

fn uniques() -> Vec<UniqueColumns> {
    vec![
        vec![TestDocumentColumn::Field1, TestDocumentColumn::Field3].into(),
        vec![TestDocumentColumn::Field5, TestDocumentColumn::Field7].into(),
    ]
}

fn indexes() -> Vec<IndexColumns> {
    vec![
        vec![TestDocumentColumn::Field1, TestDocumentColumn::Field3].into(),
        vec![TestDocumentColumn::Field5, TestDocumentColumn::Field7].into(),
    ]
}

fn migrations() -> Vec<Migration> {
    vec![RenameColumnMigration::builder()
        .old_column_name(TestDocumentColumn::Field3)
        .new_column_name("field3_new")
        .build()
        .into()]
}

#[test]
fn test_struct_attributes() {
    assert_eq!(TestDocument::unique_columns(), uniques());
    assert_eq!(TestDocument::index_columns(), indexes());
    assert_eq!(TestDocument::migrations(), migrations());
}

#[test]
fn test_column_enum_impl() {
    assert_eq!(TestDocumentColumn::Field1.to_string(), "field1");
    assert_eq!(TestDocumentColumn::Field2.to_string(), "field2");
    assert_eq!(TestDocumentColumn::Field3.to_string(), "field3");
    assert_eq!(TestDocumentColumn::Field4.to_string(), "field4");
    assert_eq!(TestDocumentColumn::Field5.to_string(), "field5");
    assert_eq!(TestDocumentColumn::Field6.to_string(), "field6");
    assert_eq!(TestDocumentColumn::Field7.to_string(), "field7");
    assert_eq!(TestDocumentColumn::Field8.to_string(), "field8");
    assert_eq!(TestDocumentColumn::Field9.to_string(), "field9");
    assert_eq!(TestDocumentColumn::Field10.to_string(), "field10");
    assert_eq!(TestDocumentColumn::Field11.to_string(), "field11");
    assert_eq!(TestDocumentColumn::Field12.to_string(), "field12");
    assert_eq!(TestDocumentColumn::Field13.to_string(), "field13");
    assert_eq!(TestDocumentColumn::Field14.to_string(), "field14");
    assert_eq!(TestDocumentColumn::Field15.to_string(), "field15");
    assert_eq!(TestDocumentColumn::Field16.to_string(), "field16");
    assert_eq!(TestDocumentColumn::Field17.to_string(), "field17");
    assert_eq!(TestDocumentColumn::Field18.to_string(), "field18");
    assert_eq!(TestDocumentColumn::Field19.to_string(), "field19");
    assert_eq!(TestDocumentColumn::Field20.to_string(), "field20");
    assert_eq!(TestDocumentColumn::Field21.to_string(), "field21");
    assert_eq!(TestDocumentColumn::Field22.to_string(), "field22");
    assert_eq!(TestDocumentColumn::Field23.to_string(), "field23");
    assert_eq!(TestDocumentColumn::Field24.to_string(), "field24");
    assert_eq!(TestDocumentColumn::Field25.to_string(), "field25");
    assert_eq!(TestDocumentColumn::Field26.to_string(), "field26");
    assert_eq!(TestDocumentColumn::Field27.to_string(), "field27");
    assert_eq!(TestDocumentColumn::Field28.to_string(), "field28");
    assert_eq!(TestDocumentColumn::Field29.to_string(), "field29");
    assert_eq!(TestDocumentColumn::Field30.to_string(), "field30");
    assert_eq!(TestDocumentColumn::Field31.to_string(), "field31");
    assert_eq!(TestDocumentColumn::Field32.to_string(), "field32");
    assert_eq!(TestDocumentColumn::Field33.to_string(), "field33");
    assert_eq!(TestDocumentColumn::Field34.to_string(), "field34");
    assert_eq!(TestDocumentColumn::Field35.to_string(), "field35");
    assert_eq!(TestDocumentColumn::Field36.to_string(), "field36");
    assert_eq!(TestDocumentColumn::Field37.to_string(), "field37");
    assert_eq!(TestDocumentColumn::Field38.to_string(), "field38");
    assert_eq!(TestDocumentColumn::Field39.to_string(), "field39");
    assert_eq!(TestDocumentColumn::Field40.to_string(), "field40");
    assert_eq!(TestDocumentColumn::Field41.to_string(), "field41");
    assert_eq!(
        TestDocumentColumn::Field42WithUnderscore.to_string(),
        "field42_with_underscore"
    );
}

#[test]
fn test_document_data_impl_create() {
    let document = TestDocument::create(&vec![
        ("field1".to_string(), ColumnValue::Bool(true)),
        ("field2".to_string(), ColumnValue::Bool(false)),
        ("field3".to_string(), ColumnValue::Char('f')),
        ("field4".to_string(), ColumnValue::Char('d')),
        ("field5".to_string(), ColumnValue::U8(1)),
        ("field6".to_string(), ColumnValue::U8(2)),
        ("field7".to_string(), ColumnValue::U16(3)),
        ("field8".to_string(), ColumnValue::U16(4)),
        ("field9".to_string(), ColumnValue::U32(5)),
        ("field10".to_string(), ColumnValue::U32(6)),
        ("field11".to_string(), ColumnValue::U64(7)),
        ("field12".to_string(), ColumnValue::U64(8)),
        ("field13".to_string(), ColumnValue::U128(9)),
        ("field14".to_string(), ColumnValue::U128(10)),
        ("field15".to_string(), ColumnValue::USize(11)),
        ("field16".to_string(), ColumnValue::USize(12)),
        ("field17".to_string(), ColumnValue::I8(13)),
        ("field18".to_string(), ColumnValue::I8(14)),
        ("field19".to_string(), ColumnValue::I16(15)),
        ("field20".to_string(), ColumnValue::I16(16)),
        ("field21".to_string(), ColumnValue::I32(17)),
        ("field22".to_string(), ColumnValue::I32(18)),
        ("field23".to_string(), ColumnValue::I64(19)),
        ("field24".to_string(), ColumnValue::I64(20)),
        ("field25".to_string(), ColumnValue::I128(21)),
        ("field26".to_string(), ColumnValue::I128(22)),
        ("field27".to_string(), ColumnValue::ISize(23)),
        ("field28".to_string(), ColumnValue::ISize(24)),
        ("field29".to_string(), ColumnValue::F32(25.0)),
        ("field30".to_string(), ColumnValue::F32(26.0)),
        ("field31".to_string(), ColumnValue::F64(27.0)),
        ("field32".to_string(), ColumnValue::F64(28.0)),
        ("field33".to_string(), ColumnValue::String("29".to_string())),
        ("field34".to_string(), ColumnValue::String("30".to_string())),
        ("field35".to_string(), ColumnValue::BigInt(BigInt::from(31))),
        ("field36".to_string(), ColumnValue::BigInt(BigInt::from(32))),
        ("field37".to_string(), ColumnValue::BigUint(BigUint::from(33u32))),
        ("field38".to_string(), ColumnValue::BigUint(BigUint::from(34u32))),
        (
            "field39".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![35]).unwrap()),
        ),
        (
            "field40".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![36]).unwrap()),
        ),
        (
            "field41".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![vec!["37".to_string(), "38".to_string()]]).unwrap()),
        ),
        (
            "field42_with_underscore".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![vec!["39".to_string(), "40".to_string()]]).unwrap()),
        ),
    ])
    .unwrap();
    assert!(document.field1);
    assert_eq!(document.field2, Some(false));
    assert_eq!(document.field3, 'f');
    assert_eq!(document.field4, Some('d'));
    assert_eq!(document.field5, 1);
    assert_eq!(document.field6, Some(2));
    assert_eq!(document.field7, 3);
    assert_eq!(document.field8, Some(4));
    assert_eq!(document.field9, 5);
    assert_eq!(document.field10, Some(6));
    assert_eq!(document.field11, 7);
    assert_eq!(document.field12, Some(8));
    assert_eq!(document.field13, 9);
    assert_eq!(document.field14, Some(10));
    assert_eq!(document.field15, 11);
    assert_eq!(document.field16, Some(12));
    assert_eq!(document.field17, 13);
    assert_eq!(document.field18, Some(14));
    assert_eq!(document.field19, 15);
    assert_eq!(document.field20, Some(16));
    assert_eq!(document.field21, 17);
    assert_eq!(document.field22, Some(18));
    assert_eq!(document.field23, 19);
    assert_eq!(document.field24, Some(20));
    assert_eq!(document.field25, 21);
    assert_eq!(document.field26, Some(22));
    assert_eq!(document.field27, 23);
    assert_eq!(document.field28, Some(24));
    assert_eq!(document.field29, 25.0);
    assert_eq!(document.field30, Some(26.0));
    assert_eq!(document.field31, 27.0);
    assert_eq!(document.field32, Some(28.0));
    assert_eq!(document.field33, "29".to_string());
    assert_eq!(document.field34, Some("30".to_string()));
    assert_eq!(document.field35, BigInt::from(31));
    assert_eq!(document.field36, Some(BigInt::from(32)));
    assert_eq!(document.field37, BigUint::from(33u32));
    assert_eq!(document.field38, Some(BigUint::from(34u32)));
    assert_eq!(document.field39, vec![35]);
    assert_eq!(document.field40, Some(vec![36]));
    assert_eq!(document.field41, vec![vec!["37".to_string(), "38".to_string()]]);
    assert_eq!(
        document.field42_with_underscore,
        Some(vec![vec!["39".to_string(), "40".to_string()]])
    );
}

#[test]
fn test_document_data_impl_create_with_none() {
    let document = TestDocument::create(&vec![
        ("field1".to_string(), ColumnValue::Bool(true)),
        ("field3".to_string(), ColumnValue::Char('f')),
        ("field5".to_string(), ColumnValue::U8(1)),
        ("field7".to_string(), ColumnValue::U16(2)),
        ("field9".to_string(), ColumnValue::U32(3)),
        ("field11".to_string(), ColumnValue::U64(4)),
        ("field13".to_string(), ColumnValue::U128(5)),
        ("field15".to_string(), ColumnValue::USize(6)),
        ("field17".to_string(), ColumnValue::I8(7)),
        ("field19".to_string(), ColumnValue::I16(8)),
        ("field21".to_string(), ColumnValue::I32(9)),
        ("field23".to_string(), ColumnValue::I64(10)),
        ("field25".to_string(), ColumnValue::I128(11)),
        ("field27".to_string(), ColumnValue::ISize(12)),
        ("field29".to_string(), ColumnValue::F32(13.0)),
        ("field31".to_string(), ColumnValue::F64(14.0)),
        ("field33".to_string(), ColumnValue::String("15".to_string())),
        ("field35".to_string(), ColumnValue::BigInt(BigInt::from(16))),
        ("field37".to_string(), ColumnValue::BigUint(BigUint::from(17u32))),
        (
            "field39".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![18]).unwrap()),
        ),
        (
            "field41".to_string(),
            ColumnValue::Json(serde_json::to_value(vec![vec!["19".to_string(), "20".to_string()]]).unwrap()),
        ),
    ])
    .unwrap();
    assert!(document.field1);
    assert!(document.field2.is_none());
    assert_eq!(document.field3, 'f');
    assert!(document.field4.is_none());
    assert_eq!(document.field5, 1);
    assert!(document.field6.is_none());
    assert_eq!(document.field7, 2);
    assert!(document.field8.is_none());
    assert_eq!(document.field9, 3);
    assert!(document.field10.is_none());
    assert_eq!(document.field11, 4);
    assert!(document.field12.is_none());
    assert_eq!(document.field13, 5);
    assert!(document.field14.is_none());
    assert_eq!(document.field15, 6);
    assert!(document.field16.is_none());
    assert_eq!(document.field17, 7);
    assert!(document.field18.is_none());
    assert_eq!(document.field19, 8);
    assert!(document.field20.is_none());
    assert_eq!(document.field21, 9);
    assert!(document.field22.is_none());
    assert_eq!(document.field23, 10);
    assert!(document.field24.is_none());
    assert_eq!(document.field25, 11);
    assert!(document.field26.is_none());
    assert_eq!(document.field27, 12);
    assert!(document.field28.is_none());
    assert_eq!(document.field29, 13.0);
    assert!(document.field30.is_none());
    assert_eq!(document.field31, 14.0);
    assert!(document.field32.is_none());
    assert_eq!(document.field33, "15".to_string());
    assert!(document.field34.is_none());
    assert_eq!(document.field35, BigInt::from(16));
    assert!(document.field36.is_none());
    assert_eq!(document.field37, BigUint::from(17u32));
    assert!(document.field38.is_none());
    assert_eq!(document.field39, vec![18]);
    assert!(document.field40.is_none());
    assert_eq!(document.field41, vec![vec!["19".to_string(), "20".to_string(),]]);
    assert!(document.field42_with_underscore.is_none());
}

#[test]
fn test_document_data_impl_collection_name() {
    assert_eq!(TestDocument::collection_name(), "test_collection");
}

#[test]
fn test_document_data_impl_columns() {
    assert_eq!(
        TestDocument::columns(),
        vec![
            Column::builder()
                .column_name(TestDocumentColumn::Field1.to_string())
                .column_type(mystiko_storage::column::ColumnType::Bool)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field2.to_string())
                .column_type(mystiko_storage::column::ColumnType::Bool)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field3.to_string())
                .column_type(mystiko_storage::column::ColumnType::Char)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field4.to_string())
                .column_type(mystiko_storage::column::ColumnType::Char)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field5.to_string())
                .column_type(mystiko_storage::column::ColumnType::U8)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field6.to_string())
                .column_type(mystiko_storage::column::ColumnType::U8)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field7.to_string())
                .column_type(mystiko_storage::column::ColumnType::U16)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field8.to_string())
                .column_type(mystiko_storage::column::ColumnType::U16)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field9.to_string())
                .column_type(mystiko_storage::column::ColumnType::U32)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field10.to_string())
                .column_type(mystiko_storage::column::ColumnType::U32)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field11.to_string())
                .column_type(mystiko_storage::column::ColumnType::U64)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field12.to_string())
                .column_type(mystiko_storage::column::ColumnType::U64)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field13.to_string())
                .column_type(mystiko_storage::column::ColumnType::U128)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field14.to_string())
                .column_type(mystiko_storage::column::ColumnType::U128)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field15.to_string())
                .column_type(mystiko_storage::column::ColumnType::USize)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field16.to_string())
                .column_type(mystiko_storage::column::ColumnType::USize)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field17.to_string())
                .column_type(mystiko_storage::column::ColumnType::I8)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field18.to_string())
                .column_type(mystiko_storage::column::ColumnType::I8)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field19.to_string())
                .column_type(mystiko_storage::column::ColumnType::I16)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field20.to_string())
                .column_type(mystiko_storage::column::ColumnType::I16)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field21.to_string())
                .column_type(mystiko_storage::column::ColumnType::I32)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field22.to_string())
                .column_type(mystiko_storage::column::ColumnType::I32)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field23.to_string())
                .column_type(mystiko_storage::column::ColumnType::I64)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field24.to_string())
                .column_type(mystiko_storage::column::ColumnType::I64)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field25.to_string())
                .column_type(mystiko_storage::column::ColumnType::I128)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field26.to_string())
                .column_type(mystiko_storage::column::ColumnType::I128)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field27.to_string())
                .column_type(mystiko_storage::column::ColumnType::ISize)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field28.to_string())
                .column_type(mystiko_storage::column::ColumnType::ISize)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field29.to_string())
                .column_type(mystiko_storage::column::ColumnType::F32)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field30.to_string())
                .column_type(mystiko_storage::column::ColumnType::F32)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field31.to_string())
                .column_type(mystiko_storage::column::ColumnType::F64)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field32.to_string())
                .column_type(mystiko_storage::column::ColumnType::F64)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field33.to_string())
                .column_type(mystiko_storage::column::ColumnType::String)
                .length_limit(Some(128))
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field34.to_string())
                .column_type(mystiko_storage::column::ColumnType::String)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field35.to_string())
                .column_type(mystiko_storage::column::ColumnType::BigInt)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field36.to_string())
                .column_type(mystiko_storage::column::ColumnType::BigInt)
                .length_limit(Some(256))
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field37.to_string())
                .column_type(mystiko_storage::column::ColumnType::BigUint)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field38.to_string())
                .column_type(mystiko_storage::column::ColumnType::BigUint)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field39.to_string())
                .column_type(mystiko_storage::column::ColumnType::Json)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field40.to_string())
                .column_type(mystiko_storage::column::ColumnType::Json)
                .nullable(true)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field41.to_string())
                .column_type(mystiko_storage::column::ColumnType::Json)
                .build(),
            Column::builder()
                .column_name(TestDocumentColumn::Field42WithUnderscore.to_string())
                .column_type(mystiko_storage::column::ColumnType::Json)
                .nullable(true)
                .build(),
        ]
    );
}

#[test]
fn test_document_data_impl_column_values() {
    let document = TestDocument {
        field1: true,
        field2: Some(false),
        field3: 'f',
        field4: Some('d'),
        field5: 1,
        field6: Some(2),
        field7: 3,
        field8: Some(4),
        field9: 5,
        field10: Some(6),
        field11: 7,
        field12: Some(8),
        field13: 9,
        field14: Some(10),
        field15: 11,
        field16: Some(12),
        field17: 13,
        field18: Some(14),
        field19: 15,
        field20: Some(16),
        field21: 17,
        field22: Some(18),
        field23: 19,
        field24: Some(20),
        field25: 21,
        field26: Some(22),
        field27: 23,
        field28: Some(24),
        field29: 25.0,
        field30: Some(26.0),
        field31: 27.0,
        field32: Some(28.0),
        field33: "29".to_string(),
        field34: Some("30".to_string()),
        field35: BigInt::from(31),
        field36: Some(BigInt::from(32)),
        field37: BigUint::from(33u32),
        field38: Some(BigUint::from(34u32)),
        field39: vec![35],
        field40: Some(vec![36]),
        field41: vec![vec!["37".to_string(), "38".to_string()]],
        field42_with_underscore: Some(vec![vec!["39".to_string(), "40".to_string()]]),
    };
    let (columns, column_values): (Vec<Column>, Vec<Option<ColumnValue>>) =
        document.column_values().into_iter().unzip();
    assert_eq!(columns, TestDocument::columns());
    assert_eq!(
        column_values,
        vec![
            Some(ColumnValue::Bool(true)),
            Some(ColumnValue::Bool(false)),
            Some(ColumnValue::Char('f')),
            Some(ColumnValue::Char('d')),
            Some(ColumnValue::U8(1)),
            Some(ColumnValue::U8(2)),
            Some(ColumnValue::U16(3)),
            Some(ColumnValue::U16(4)),
            Some(ColumnValue::U32(5)),
            Some(ColumnValue::U32(6)),
            Some(ColumnValue::U64(7)),
            Some(ColumnValue::U64(8)),
            Some(ColumnValue::U128(9)),
            Some(ColumnValue::U128(10)),
            Some(ColumnValue::USize(11)),
            Some(ColumnValue::USize(12)),
            Some(ColumnValue::I8(13)),
            Some(ColumnValue::I8(14)),
            Some(ColumnValue::I16(15)),
            Some(ColumnValue::I16(16)),
            Some(ColumnValue::I32(17)),
            Some(ColumnValue::I32(18)),
            Some(ColumnValue::I64(19)),
            Some(ColumnValue::I64(20)),
            Some(ColumnValue::I128(21)),
            Some(ColumnValue::I128(22)),
            Some(ColumnValue::ISize(23)),
            Some(ColumnValue::ISize(24)),
            Some(ColumnValue::F32(25.0)),
            Some(ColumnValue::F32(26.0)),
            Some(ColumnValue::F64(27.0)),
            Some(ColumnValue::F64(28.0)),
            Some(ColumnValue::String("29".to_string())),
            Some(ColumnValue::String("30".to_string())),
            Some(ColumnValue::BigInt(BigInt::from(31))),
            Some(ColumnValue::BigInt(BigInt::from(32))),
            Some(ColumnValue::BigUint(BigUint::from(33u32))),
            Some(ColumnValue::BigUint(BigUint::from(34u32))),
            Some(ColumnValue::Json(serde_json::to_value(vec![35]).unwrap())),
            Some(ColumnValue::Json(serde_json::to_value(vec![36]).unwrap())),
            Some(ColumnValue::Json(
                serde_json::to_value(vec![vec!["37".to_string(), "38".to_string()]]).unwrap()
            )),
            Some(ColumnValue::Json(
                serde_json::to_value(vec![vec!["39".to_string(), "40".to_string()]]).unwrap()
            )),
        ]
    );
}

#[test]
fn test_document_data_impl_column_values_with_none() {
    let document = TestDocument {
        field1: true,
        field2: None,
        field3: 'f',
        field4: None,
        field5: 1,
        field6: None,
        field7: 2,
        field8: None,
        field9: 3,
        field10: None,
        field11: 4,
        field12: None,
        field13: 5,
        field14: None,
        field15: 6,
        field16: None,
        field17: 7,
        field18: None,
        field19: 8,
        field20: None,
        field21: 9,
        field22: None,
        field23: 10,
        field24: None,
        field25: 11,
        field26: None,
        field27: 12,
        field28: None,
        field29: 13.0,
        field30: None,
        field31: 14.0,
        field32: None,
        field33: "15".to_string(),
        field34: None,
        field35: BigInt::from(16),
        field36: None,
        field37: BigUint::from(17u32),
        field38: None,
        field39: vec![18],
        field40: None,
        field41: vec![vec!["19".to_string(), "20".to_string()]],
        field42_with_underscore: None,
    };
    let (columns, column_values): (Vec<Column>, Vec<Option<ColumnValue>>) =
        document.column_values().into_iter().unzip();
    assert_eq!(columns, TestDocument::columns());
    assert_eq!(
        column_values,
        vec![
            Some(ColumnValue::Bool(true)),
            None,
            Some(ColumnValue::Char('f')),
            None,
            Some(ColumnValue::U8(1)),
            None,
            Some(ColumnValue::U16(2)),
            None,
            Some(ColumnValue::U32(3)),
            None,
            Some(ColumnValue::U64(4)),
            None,
            Some(ColumnValue::U128(5)),
            None,
            Some(ColumnValue::USize(6)),
            None,
            Some(ColumnValue::I8(7)),
            None,
            Some(ColumnValue::I16(8)),
            None,
            Some(ColumnValue::I32(9)),
            None,
            Some(ColumnValue::I64(10)),
            None,
            Some(ColumnValue::I128(11)),
            None,
            Some(ColumnValue::ISize(12)),
            None,
            Some(ColumnValue::F32(13.0)),
            None,
            Some(ColumnValue::F64(14.0)),
            None,
            Some(ColumnValue::String("15".to_string())),
            None,
            Some(ColumnValue::BigInt(BigInt::from(16))),
            None,
            Some(ColumnValue::BigUint(BigUint::from(17u32))),
            None,
            Some(ColumnValue::Json(serde_json::to_value(vec![18]).unwrap())),
            None,
            Some(ColumnValue::Json(
                serde_json::to_value(vec![vec!["19".to_string(), "20".to_string()]]).unwrap()
            )),
            None,
        ]
    );
}

#[test]
fn test_default_collection_name() {
    #[derive(CollectionBuilder, Clone, Debug, PartialEq)]
    pub struct TestDocumentWithDefaultName {
        pub field1: bool,
    }
    assert_eq!(
        TestDocumentWithDefaultName::collection_name(),
        "test_document_with_default_names"
    );
    assert_eq!(TestDocumentWithDefaultNameColumn::Field1.to_string(), "field1");
}
