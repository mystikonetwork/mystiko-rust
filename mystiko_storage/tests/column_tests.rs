use mystiko_storage::column::{ColumnType, ColumnValue};
use num_bigint::{BigInt, BigUint};

#[test]
fn test_column_type() {
    assert_eq!(ColumnType::Bool.to_string(), "bool");
    assert_eq!(ColumnType::Char.to_string(), "char");
    assert_eq!(ColumnType::I8.to_string(), "i8");
    assert_eq!(ColumnType::I16.to_string(), "i16");
    assert_eq!(ColumnType::I32.to_string(), "i32");
    assert_eq!(ColumnType::I64.to_string(), "i64");
    assert_eq!(ColumnType::I128.to_string(), "i128");
    assert_eq!(ColumnType::ISize.to_string(), "isize");
    assert_eq!(ColumnType::U8.to_string(), "u8");
    assert_eq!(ColumnType::U16.to_string(), "u16");
    assert_eq!(ColumnType::U32.to_string(), "u32");
    assert_eq!(ColumnType::U64.to_string(), "u64");
    assert_eq!(ColumnType::U128.to_string(), "u128");
    assert_eq!(ColumnType::USize.to_string(), "usize");
    assert_eq!(ColumnType::F32.to_string(), "f32");
    assert_eq!(ColumnType::F64.to_string(), "f64");
    assert_eq!(ColumnType::String.to_string(), "string");
    assert_eq!(ColumnType::BigInt.to_string(), "bigint");
    assert_eq!(ColumnType::BigUint.to_string(), "biguint");
    assert_eq!(ColumnType::Json.to_string(), "json");
}

#[test]
fn test_bool_column_value() {
    let value: ColumnValue = true.into();
    assert!(value.as_bool().unwrap());
    assert!(value.as_char().is_err());
    assert_eq!(value.column_type(), ColumnType::Bool);
    assert_eq!(value.to_string(), "true");
}

#[test]
fn test_char_column_value() {
    let value: ColumnValue = 'a'.into();
    assert_eq!(value.as_char().unwrap(), 'a');
    assert!(value.as_i8().is_err());
    assert_eq!(value.column_type(), ColumnType::Char);
    assert_eq!(value.to_string(), "a");
}

#[test]
fn test_i8_column_value() {
    let value: ColumnValue = 1i8.into();
    assert_eq!(value.as_i8().unwrap(), 1i8);
    assert!(value.as_i16().is_err());
    assert_eq!(value.column_type(), ColumnType::I8);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_i16_column_value() {
    let value: ColumnValue = 1i16.into();
    assert_eq!(value.as_i16().unwrap(), 1i16);
    assert!(value.as_i32().is_err());
    assert_eq!(value.column_type(), ColumnType::I16);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_i32_column_value() {
    let value: ColumnValue = 1i32.into();
    assert_eq!(value.as_i32().unwrap(), 1i32);
    assert!(value.as_i64().is_err());
    assert_eq!(value.column_type(), ColumnType::I32);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_i64_column_value() {
    let value: ColumnValue = 1i64.into();
    assert_eq!(value.as_i64().unwrap(), 1i64);
    assert!(value.as_i128().is_err());
    assert_eq!(value.column_type(), ColumnType::I64);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_i128_column_value() {
    let value: ColumnValue = 1i128.into();
    assert_eq!(value.as_i128().unwrap(), 1i128);
    assert!(value.as_isize().is_err());
    assert_eq!(value.column_type(), ColumnType::I128);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_isize_column_value() {
    let value: ColumnValue = 1isize.into();
    assert_eq!(value.as_isize().unwrap(), 1isize);
    assert!(value.as_u8().is_err());
    assert_eq!(value.column_type(), ColumnType::ISize);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_u8_column_value() {
    let value: ColumnValue = 1u8.into();
    assert_eq!(value.as_u8().unwrap(), 1u8);
    assert!(value.as_u16().is_err());
    assert_eq!(value.column_type(), ColumnType::U8);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_u16_column_value() {
    let value: ColumnValue = 1u16.into();
    assert_eq!(value.as_u16().unwrap(), 1u16);
    assert!(value.as_u32().is_err());
    assert_eq!(value.column_type(), ColumnType::U16);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_u32_column_value() {
    let value: ColumnValue = 1u32.into();
    assert_eq!(value.as_u32().unwrap(), 1u32);
    assert!(value.as_u64().is_err());
    assert_eq!(value.column_type(), ColumnType::U32);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_u64_column_value() {
    let value: ColumnValue = 1u64.into();
    assert_eq!(value.as_u64().unwrap(), 1u64);
    assert!(value.as_u128().is_err());
    assert_eq!(value.column_type(), ColumnType::U64);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_u128_column_value() {
    let value: ColumnValue = 1u128.into();
    assert_eq!(value.as_u128().unwrap(), 1u128);
    assert!(value.as_usize().is_err());
    assert_eq!(value.column_type(), ColumnType::U128);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_usize_column_value() {
    let value: ColumnValue = 1usize.into();
    assert_eq!(value.as_usize().unwrap(), 1usize);
    assert!(value.as_f32().is_err());
    assert_eq!(value.column_type(), ColumnType::USize);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_f32_column_value() {
    let value: ColumnValue = 1.0f32.into();
    assert_eq!(value.as_f32().unwrap(), 1.0f32);
    assert!(value.as_f64().is_err());
    assert_eq!(value.column_type(), ColumnType::F32);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_f64_column_value() {
    let value: ColumnValue = 1.0f64.into();
    assert_eq!(value.as_f64().unwrap(), 1.0f64);
    assert!(value.as_string().is_err());
    assert_eq!(value.column_type(), ColumnType::F64);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_string_column_value() {
    let value: ColumnValue = "hello".into();
    assert_eq!(value.as_string().unwrap(), "hello");
    assert!(value.as_bigint().is_err());
    assert_eq!(value.column_type(), ColumnType::String);
    assert_eq!(value.to_string(), "hello");
}

#[test]
fn test_bigint_column_value() {
    let value: ColumnValue = BigInt::from(1).into();
    assert_eq!(value.as_bigint().unwrap(), BigInt::from(1));
    assert!(value.as_biguint().is_err());
    assert_eq!(value.column_type(), ColumnType::BigInt);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_biguint_column_value() {
    let value: ColumnValue = BigUint::from(1u32).into();
    assert_eq!(value.as_biguint().unwrap(), BigUint::from(1u32));
    assert!(value.as_json().is_err());
    assert_eq!(value.column_type(), ColumnType::BigUint);
    assert_eq!(value.to_string(), "1");
}

#[test]
fn test_json_column_value() {
    let value: ColumnValue = serde_json::to_value(vec!["a", "b"]).unwrap().into();
    assert_eq!(value.as_json().unwrap(), serde_json::to_value(vec!["a", "b"]).unwrap());
    assert!(value.as_bool().is_err());
    assert_eq!(value.column_type(), ColumnType::Json);
    assert_eq!(value.to_string(), "[\"a\",\"b\"]");
}
