use crate::error::StorageError;
use num_bigint::BigInt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};
use typed_builder::TypedBuilder;

type Result<T> = anyhow::Result<T, StorageError>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnType {
    Bool,
    Char,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    String,
    BigInt,
    Json,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnValue {
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    F32(f32),
    F64(f64),
    String(String),
    BigInt(BigInt),
    Json(Value),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct Column {
    pub column_type: ColumnType,
    #[builder(setter(into))]
    pub column_name: String,
    #[builder(default = false)]
    pub nullable: bool,
    #[builder(default = false)]
    pub is_primary_key: bool,
    #[builder(default)]
    pub length_limit: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct UniqueColumns {
    #[builder(default, setter(into, strip_option))]
    pub unique_name: Option<String>,
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypedBuilder)]
pub struct IndexColumns {
    #[builder(default, setter(into, strip_option))]
    pub index_name: Option<String>,
    pub column_names: Vec<String>,
}

impl From<bool> for ColumnValue {
    fn from(value: bool) -> Self {
        ColumnValue::Bool(value)
    }
}

impl From<char> for ColumnValue {
    fn from(value: char) -> Self {
        ColumnValue::Char(value)
    }
}

impl From<i8> for ColumnValue {
    fn from(value: i8) -> Self {
        ColumnValue::I8(value)
    }
}

impl From<i16> for ColumnValue {
    fn from(value: i16) -> Self {
        ColumnValue::I16(value)
    }
}

impl From<i32> for ColumnValue {
    fn from(value: i32) -> Self {
        ColumnValue::I32(value)
    }
}

impl From<i64> for ColumnValue {
    fn from(value: i64) -> Self {
        ColumnValue::I64(value)
    }
}

impl From<i128> for ColumnValue {
    fn from(value: i128) -> Self {
        ColumnValue::I128(value)
    }
}

impl From<isize> for ColumnValue {
    fn from(value: isize) -> Self {
        ColumnValue::ISize(value)
    }
}

impl From<u8> for ColumnValue {
    fn from(value: u8) -> Self {
        ColumnValue::U8(value)
    }
}

impl From<u16> for ColumnValue {
    fn from(value: u16) -> Self {
        ColumnValue::U16(value)
    }
}

impl From<u32> for ColumnValue {
    fn from(value: u32) -> Self {
        ColumnValue::U32(value)
    }
}

impl From<u64> for ColumnValue {
    fn from(value: u64) -> Self {
        ColumnValue::U64(value)
    }
}

impl From<u128> for ColumnValue {
    fn from(value: u128) -> Self {
        ColumnValue::U128(value)
    }
}

impl From<usize> for ColumnValue {
    fn from(value: usize) -> Self {
        ColumnValue::USize(value)
    }
}

impl From<f32> for ColumnValue {
    fn from(value: f32) -> Self {
        ColumnValue::F32(value)
    }
}

impl From<f64> for ColumnValue {
    fn from(value: f64) -> Self {
        ColumnValue::F64(value)
    }
}

impl From<String> for ColumnValue {
    fn from(value: String) -> Self {
        ColumnValue::String(value)
    }
}
impl From<&str> for ColumnValue {
    fn from(value: &str) -> Self {
        ColumnValue::String(value.into())
    }
}

impl From<BigInt> for ColumnValue {
    fn from(value: BigInt) -> Self {
        ColumnValue::BigInt(value)
    }
}

impl From<Value> for ColumnValue {
    fn from(value: Value) -> Self {
        ColumnValue::Json(value)
    }
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnType::Bool => Display::fmt("bool", f),
            ColumnType::Char => Display::fmt("char", f),
            ColumnType::I8 => Display::fmt("i8", f),
            ColumnType::I16 => Display::fmt("i16", f),
            ColumnType::I32 => Display::fmt("i32", f),
            ColumnType::I64 => Display::fmt("i64", f),
            ColumnType::I128 => Display::fmt("i128", f),
            ColumnType::ISize => Display::fmt("isize", f),
            ColumnType::U8 => Display::fmt("u8", f),
            ColumnType::U16 => Display::fmt("u16", f),
            ColumnType::U32 => Display::fmt("u32", f),
            ColumnType::U64 => Display::fmt("u64", f),
            ColumnType::U128 => Display::fmt("u128", f),
            ColumnType::USize => Display::fmt("usize", f),
            ColumnType::F32 => Display::fmt("f32", f),
            ColumnType::F64 => Display::fmt("f64", f),
            ColumnType::String => Display::fmt("string", f),
            ColumnType::BigInt => Display::fmt("bigint", f),
            ColumnType::Json => Display::fmt("json", f),
        }
    }
}

impl Display for ColumnValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnValue::Bool(value) => Display::fmt(&value, f),
            ColumnValue::Char(value) => Display::fmt(&value, f),
            ColumnValue::I8(value) => Display::fmt(&value, f),
            ColumnValue::I16(value) => Display::fmt(&value, f),
            ColumnValue::I32(value) => Display::fmt(&value, f),
            ColumnValue::I64(value) => Display::fmt(&value, f),
            ColumnValue::I128(value) => Display::fmt(&value, f),
            ColumnValue::ISize(value) => Display::fmt(&value, f),
            ColumnValue::U8(value) => Display::fmt(&value, f),
            ColumnValue::U16(value) => Display::fmt(&value, f),
            ColumnValue::U32(value) => Display::fmt(&value, f),
            ColumnValue::U64(value) => Display::fmt(&value, f),
            ColumnValue::U128(value) => Display::fmt(&value, f),
            ColumnValue::USize(value) => Display::fmt(&value, f),
            ColumnValue::F32(value) => Display::fmt(&value, f),
            ColumnValue::F64(value) => Display::fmt(&value, f),
            ColumnValue::String(value) => Display::fmt(&value, f),
            ColumnValue::BigInt(value) => Display::fmt(&value, f),
            ColumnValue::Json(value) => Display::fmt(&value, f),
        }
    }
}

impl ColumnValue {
    pub fn column_type(&self) -> ColumnType {
        match self {
            ColumnValue::Bool(_) => ColumnType::Bool,
            ColumnValue::Char(_) => ColumnType::Char,
            ColumnValue::I8(_) => ColumnType::I8,
            ColumnValue::I16(_) => ColumnType::I16,
            ColumnValue::I32(_) => ColumnType::I32,
            ColumnValue::I64(_) => ColumnType::I64,
            ColumnValue::I128(_) => ColumnType::I128,
            ColumnValue::ISize(_) => ColumnType::ISize,
            ColumnValue::U8(_) => ColumnType::U8,
            ColumnValue::U16(_) => ColumnType::U16,
            ColumnValue::U32(_) => ColumnType::U32,
            ColumnValue::U64(_) => ColumnType::U64,
            ColumnValue::U128(_) => ColumnType::U128,
            ColumnValue::USize(_) => ColumnType::USize,
            ColumnValue::F32(_) => ColumnType::F32,
            ColumnValue::F64(_) => ColumnType::F64,
            ColumnValue::String(_) => ColumnType::String,
            ColumnValue::BigInt(_) => ColumnType::BigInt,
            ColumnValue::Json(_) => ColumnType::Json,
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        match self {
            ColumnValue::Bool(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::Bool.to_string(),
            )),
        }
    }

    pub fn as_char(&self) -> Result<char> {
        match self {
            ColumnValue::Char(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::Char.to_string(),
            )),
        }
    }

    pub fn as_i8(&self) -> Result<i8> {
        match self {
            ColumnValue::I8(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::I8.to_string(),
            )),
        }
    }

    pub fn as_i16(&self) -> Result<i16> {
        match self {
            ColumnValue::I16(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::I16.to_string(),
            )),
        }
    }

    pub fn as_i32(&self) -> Result<i32> {
        match self {
            ColumnValue::I32(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::I32.to_string(),
            )),
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        match self {
            ColumnValue::I64(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::I64.to_string(),
            )),
        }
    }

    pub fn as_i128(&self) -> Result<i128> {
        match self {
            ColumnValue::I128(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::I128.to_string(),
            )),
        }
    }

    pub fn as_isize(&self) -> Result<isize> {
        match self {
            ColumnValue::ISize(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::ISize.to_string(),
            )),
        }
    }

    pub fn as_u8(&self) -> Result<u8> {
        match self {
            ColumnValue::U8(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::U8.to_string(),
            )),
        }
    }

    pub fn as_u16(&self) -> Result<u16> {
        match self {
            ColumnValue::U16(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::U16.to_string(),
            )),
        }
    }

    pub fn as_u32(&self) -> Result<u32> {
        match self {
            ColumnValue::U32(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::U32.to_string(),
            )),
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        match self {
            ColumnValue::U64(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::U64.to_string(),
            )),
        }
    }

    pub fn as_u128(&self) -> Result<u128> {
        match self {
            ColumnValue::U128(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::U128.to_string(),
            )),
        }
    }

    pub fn as_usize(&self) -> Result<usize> {
        match self {
            ColumnValue::USize(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::USize.to_string(),
            )),
        }
    }

    pub fn as_f32(&self) -> Result<f32> {
        match self {
            ColumnValue::F32(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::F32.to_string(),
            )),
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        match self {
            ColumnValue::F64(value) => Ok(*value),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::F64.to_string(),
            )),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        match self {
            ColumnValue::String(value) => Ok(value.clone()),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::String.to_string(),
            )),
        }
    }

    pub fn as_bigint(&self) -> Result<BigInt> {
        match self {
            ColumnValue::BigInt(value) => Ok(value.clone()),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::BigInt.to_string(),
            )),
        }
    }

    pub fn as_json(&self) -> Result<Value> {
        match self {
            ColumnValue::Json(value) => Ok(value.clone()),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::Json.to_string(),
            )),
        }
    }

    pub fn as_json_object<T: DeserializeOwned>(&self) -> Result<T> {
        match self {
            ColumnValue::Json(value) => Ok(serde_json::from_value(value.clone())?),
            _ => Err(StorageError::WrongColumnTypeError(
                self.column_type().to_string(),
                ColumnType::Json.to_string(),
            )),
        }
    }
}
