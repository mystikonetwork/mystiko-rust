use crate::error::ProtosError;
use crate::gen::mystiko::storage;
use crate::storage::v1::column_value::Value;
use crate::storage::v1::{ColumnType, ColumnValue};
use mystiko_utils::convert::{
    biguint_to_bytes, bytes_to_biguint, bytes_to_i128, bytes_to_u128, i128_to_bytes, u128_to_bytes,
};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Signed;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Display, Formatter};

type Result<T> = anyhow::Result<T, ProtosError>;

impl From<bool> for ColumnValue {
    fn from(value: bool) -> Self {
        ColumnValue::builder().value(Value::BoolValue(value)).build()
    }
}

impl From<char> for ColumnValue {
    fn from(value: char) -> Self {
        ColumnValue::builder()
            .value(Value::CharValue(value.to_string()))
            .build()
    }
}

impl From<i8> for ColumnValue {
    fn from(value: i8) -> Self {
        ColumnValue::builder().value(Value::I8Value(value as i32)).build()
    }
}

impl From<i16> for ColumnValue {
    fn from(value: i16) -> Self {
        ColumnValue::builder().value(Value::I16Value(value as i32)).build()
    }
}

impl From<i32> for ColumnValue {
    fn from(value: i32) -> Self {
        ColumnValue::builder().value(Value::I32Value(value)).build()
    }
}

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::builder().value(Value::I64Value(value)).build()
    }
}

impl From<i128> for ColumnValue {
    fn from(value: i128) -> Self {
        ColumnValue::builder()
            .value(Value::I128Value(i128_to_bytes(value)))
            .build()
    }
}

impl From<isize> for ColumnValue {
    fn from(value: isize) -> Self {
        ColumnValue::builder().value(Value::IsizeValue(value as i64)).build()
    }
}

impl From<u8> for ColumnValue {
    fn from(value: u8) -> Self {
        ColumnValue::builder().value(Value::U8Value(value as u32)).build()
    }
}

impl From<u16> for ColumnValue {
    fn from(value: u16) -> Self {
        ColumnValue::builder().value(Value::U16Value(value as u32)).build()
    }
}

impl From<u32> for ColumnValue {
    fn from(value: u32) -> Self {
        ColumnValue::builder().value(Value::U32Value(value)).build()
    }
}

impl From<u64> for ColumnValue {
    fn from(value: u64) -> Self {
        ColumnValue::builder().value(Value::U64Value(value)).build()
    }
}

impl From<u128> for ColumnValue {
    fn from(value: u128) -> Self {
        ColumnValue::builder()
            .value(Value::U128Value(u128_to_bytes(value)))
            .build()
    }
}

impl From<usize> for ColumnValue {
    fn from(value: usize) -> Self {
        ColumnValue::builder().value(Value::UsizeValue(value as u64)).build()
    }
}

impl From<f32> for ColumnValue {
    fn from(value: f32) -> Self {
        ColumnValue::builder().value(Value::F32Value(value)).build()
    }
}

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        ColumnValue::builder().value(Value::F64Value(value)).build()
    }
}

impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        ColumnValue::builder().value(Value::StringValue(value)).build()
    }
}

impl From<&str> for ColumnValue {
    fn from(value: &str) -> Self {
        ColumnValue::builder().value(Value::StringValue(value.into())).build()
    }
}

impl From<BigInt> for ColumnValue {
    fn from(value: BigInt) -> Self {
        ColumnValue::builder().value(Value::BigIntValue(value.into())).build()
    }
}

impl From<BigUint> for ColumnValue {
    fn from(value: BigUint) -> Self {
        ColumnValue::builder()
            .value(Value::BigUintValue(biguint_to_bytes(&value)))
            .build()
    }
}

impl From<serde_json::Value> for ColumnValue {
    fn from(value: serde_json::Value) -> Self {
        ColumnValue::builder()
            .value(Value::JsonValue(value.to_string()))
            .build()
    }
}

impl From<BigInt> for storage::v1::BigInt {
    fn from(value: BigInt) -> Self {
        let is_positive = value.is_positive();
        let (_, value) = value.to_bytes_le();
        storage::v1::BigInt::builder()
            .is_positive(is_positive)
            .value(value)
            .build()
    }
}

impl From<storage::v1::BigInt> for BigInt {
    fn from(value: crate::storage::v1::BigInt) -> Self {
        let sign = if value.is_positive { Sign::Plus } else { Sign::Minus };
        BigInt::from_biguint(sign, bytes_to_biguint(&value.value))
    }
}

impl Display for storage::v1::BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad_integral(self.is_positive, "", &bytes_to_biguint(&self.value).to_str_radix(10))
    }
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ColumnType::Unspecified => Display::fmt("bool", f),
            ColumnType::Bool => Display::fmt("bool", f),
            ColumnType::Char => Display::fmt("char", f),
            ColumnType::I8 => Display::fmt("i8", f),
            ColumnType::I16 => Display::fmt("i16", f),
            ColumnType::I32 => Display::fmt("i32", f),
            ColumnType::I64 => Display::fmt("i64", f),
            ColumnType::I128 => Display::fmt("i128", f),
            ColumnType::Isize => Display::fmt("isize", f),
            ColumnType::U8 => Display::fmt("u8", f),
            ColumnType::U16 => Display::fmt("u16", f),
            ColumnType::U32 => Display::fmt("u32", f),
            ColumnType::U64 => Display::fmt("u64", f),
            ColumnType::U128 => Display::fmt("u128", f),
            ColumnType::Usize => Display::fmt("usize", f),
            ColumnType::F32 => Display::fmt("f32", f),
            ColumnType::F64 => Display::fmt("f64", f),
            ColumnType::String => Display::fmt("string", f),
            ColumnType::BigInt => Display::fmt("bigint", f),
            ColumnType::BigUint => Display::fmt("biguint", f),
            ColumnType::Json => Display::fmt("json", f),
        }
    }
}

impl Display for ColumnValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = if let Some(value) = &self.value {
            value
        } else {
            return Err(fmt::Error);
        };
        match value {
            Value::BoolValue(value) => Display::fmt(&value, f),
            Value::CharValue(value) => Display::fmt(&value, f),
            Value::I8Value(value) => Display::fmt(&value, f),
            Value::I16Value(value) => Display::fmt(&value, f),
            Value::I32Value(value) => Display::fmt(&value, f),
            Value::I64Value(value) => Display::fmt(&value, f),
            Value::I128Value(value) => Display::fmt(&bytes_to_i128(value), f),
            Value::IsizeValue(value) => Display::fmt(&value, f),
            Value::U8Value(value) => Display::fmt(&value, f),
            Value::U16Value(value) => Display::fmt(&value, f),
            Value::U32Value(value) => Display::fmt(&value, f),
            Value::U64Value(value) => Display::fmt(&value, f),
            Value::U128Value(value) => Display::fmt(&bytes_to_u128(value), f),
            Value::UsizeValue(value) => Display::fmt(&value, f),
            Value::F32Value(value) => Display::fmt(&value, f),
            Value::F64Value(value) => Display::fmt(&value, f),
            Value::StringValue(value) => Display::fmt(&value, f),
            Value::BigIntValue(value) => Display::fmt(&value, f),
            Value::BigUintValue(value) => Display::fmt(&bytes_to_biguint(value), f),
            Value::JsonValue(value) => Display::fmt(&value, f),
        }
    }
}

impl ColumnValue {
    pub fn column_type(&self) -> Result<ColumnType> {
        let value = if let Some(value) = &self.value {
            value
        } else {
            return Err(ProtosError::ColumnValueNoneError());
        };
        match value {
            Value::BoolValue(_) => Ok(ColumnType::Bool),
            Value::CharValue(_) => Ok(ColumnType::Char),
            Value::I8Value(_) => Ok(ColumnType::I8),
            Value::I16Value(_) => Ok(ColumnType::I16),
            Value::I32Value(_) => Ok(ColumnType::I32),
            Value::I64Value(_) => Ok(ColumnType::I64),
            Value::I128Value(_) => Ok(ColumnType::I128),
            Value::IsizeValue(_) => Ok(ColumnType::Isize),
            Value::U8Value(_) => Ok(ColumnType::U8),
            Value::U16Value(_) => Ok(ColumnType::U16),
            Value::U32Value(_) => Ok(ColumnType::U32),
            Value::U64Value(_) => Ok(ColumnType::U64),
            Value::U128Value(_) => Ok(ColumnType::U128),
            Value::UsizeValue(_) => Ok(ColumnType::Usize),
            Value::F32Value(_) => Ok(ColumnType::F32),
            Value::F64Value(_) => Ok(ColumnType::F64),
            Value::StringValue(_) => Ok(ColumnType::String),
            Value::BigIntValue(_) => Ok(ColumnType::BigInt),
            Value::BigUintValue(_) => Ok(ColumnType::BigUint),
            Value::JsonValue(_) => Ok(ColumnType::Json),
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::BoolValue(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Bool.to_string(),
            )),
        }
    }

    pub fn as_char(&self) -> Result<char> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::CharValue(value) => Ok(value.chars().next().ok_or(ProtosError::ColumnValueNoneError())?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Char.to_string(),
            )),
        }
    }

    pub fn as_i8(&self) -> Result<i8> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::I8Value(value) => Ok(i8::try_from(*value).map_err(ProtosError::TryFromIntError)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::I8.to_string(),
            )),
        }
    }

    pub fn as_i16(&self) -> Result<i16> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::I16Value(value) => Ok(i16::try_from(*value).map_err(ProtosError::TryFromIntError)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::I16.to_string(),
            )),
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::I32Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::I32.to_string(),
            )),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::I64Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::I64.to_string(),
            )),
        }
    }

    pub fn as_i128(&self) -> Result<i128> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::I128Value(value) => Ok(bytes_to_i128(value)),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::I128.to_string(),
            )),
        }
    }

    pub fn as_isize(&self) -> Result<isize> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::IsizeValue(value) => Ok(*value as isize),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Isize.to_string(),
            )),
        }
    }

    pub fn as_u8(&self) -> Result<u8> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::U8Value(value) => Ok(u8::try_from(*value).map_err(ProtosError::TryFromIntError)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::U8.to_string(),
            )),
        }
    }

    pub fn as_u16(&self) -> Result<u16> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::U16Value(value) => Ok(u16::try_from(*value).map_err(ProtosError::TryFromIntError)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::U16.to_string(),
            )),
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::U32Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::U32.to_string(),
            )),
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::U64Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::U64.to_string(),
            )),
        }
    }

    pub fn as_u128(&self) -> Result<u128> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::U128Value(value) => Ok(bytes_to_u128(value)),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::U128.to_string(),
            )),
        }
    }

    pub fn as_usize(&self) -> Result<usize> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::UsizeValue(value) => Ok(*value as usize),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Usize.to_string(),
            )),
        }
    }

    pub fn as_f32(&self) -> Result<f32> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::F32Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::F32.to_string(),
            )),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::F64Value(value) => Ok(*value),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::F64.to_string(),
            )),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::StringValue(value) => Ok(value.clone()),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::String.to_string(),
            )),
        }
    }

    pub fn as_bigint(&self) -> Result<BigInt> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::BigIntValue(value) => Ok(value.clone().into()),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::BigInt.to_string(),
            )),
        }
    }

    pub fn as_biguint(&self) -> Result<BigUint> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::BigUintValue(value) => Ok(bytes_to_biguint(value)),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::BigUint.to_string(),
            )),
        }
    }

    pub fn as_json(&self) -> Result<serde_json::Value> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::JsonValue(value) => Ok(serde_json::from_str(value)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Json.to_string(),
            )),
        }
    }

    pub fn as_json_object<T: DeserializeOwned>(&self) -> Result<T> {
        let value = self.value.as_ref().ok_or(ProtosError::ColumnValueNoneError())?;
        match value {
            Value::JsonValue(value) => Ok(serde_json::from_str(value)?),
            _ => Err(ProtosError::WrongColumnTypeError(
                self.column_type()?.to_string(),
                ColumnType::Json.to_string(),
            )),
        }
    }
}
