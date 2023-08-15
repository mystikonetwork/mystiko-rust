use crate::storage::v1::{column_value, ColumnValue};
use mystiko_utils::convert::biguint_to_bytes;
use num_bigint::{BigInt, BigUint};
use serde_json::Value;

impl From<bool> for ColumnValue {
    fn from(value: bool) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::BoolValue(value))
            .build()
    }
}

impl From<char> for ColumnValue {
    fn from(value: char) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::CharValue(value.to_string()))
            .build()
    }
}

impl From<i8> for ColumnValue {
    fn from(value: i8) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::I8Value(value as i32))
            .build()
    }
}

impl From<i16> for ColumnValue {
    fn from(value: i16) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::I16Value(value as i32))
            .build()
    }
}

impl From<i32> for ColumnValue {
    fn from(value: i32) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::I32Value(value))
            .build()
    }
}

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::I64Value(value))
            .build()
    }
}

impl From<i128> for ColumnValue {
    fn from(value: i128) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::I128Value(value.to_be_bytes().to_vec()))
            .build()
    }
}

impl From<isize> for ColumnValue {
    fn from(value: isize) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::IsizeValue(value as i64))
            .build()
    }
}

impl From<u8> for ColumnValue {
    fn from(value: u8) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::U8Value(value as u32))
            .build()
    }
}

impl From<u16> for ColumnValue {
    fn from(value: u16) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::U16Value(value as u32))
            .build()
    }
}

impl From<u32> for ColumnValue {
    fn from(value: u32) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::U32Value(value))
            .build()
    }
}

impl From<u64> for ColumnValue {
    fn from(value: u64) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::U64Value(value))
            .build()
    }
}

impl From<u128> for ColumnValue {
    fn from(value: u128) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::U128Value(value.to_be_bytes().to_vec()))
            .build()
    }
}

impl From<usize> for ColumnValue {
    fn from(value: usize) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::UsizeValue(value as u64))
            .build()
    }
}

impl From<f32> for ColumnValue {
    fn from(value: f32) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::F32Value(value))
            .build()
    }
}

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::F64Value(value))
            .build()
    }
}

impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::StringValue(value))
            .build()
    }
}

impl From<&str> for ColumnValue {
    fn from(value: &str) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::StringValue(value.into()))
            .build()
    }
}

impl From<BigInt> for ColumnValue {
    fn from(value: BigInt) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::BigIntValue(value.into()))
            .build()
    }
}

impl From<BigUint> for ColumnValue {
    fn from(value: BigUint) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::BigUintValue(biguint_to_bytes(&value)))
            .build()
    }
}

impl From<Value> for ColumnValue {
    fn from(value: Value) -> Self {
        ColumnValue::builder()
            .value(column_value::Value::JsonValue(value.to_string()))
            .build()
    }
}
