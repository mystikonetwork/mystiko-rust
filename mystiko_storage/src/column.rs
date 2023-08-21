use mystiko_protos::storage::v1::ColumnType;
use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;
use typed_builder::TypedBuilder;

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
    BigUint(BigUint),
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

impl<T: ToString, I: IntoIterator<Item = T>> From<I> for UniqueColumns {
    fn from(value: I) -> Self {
        UniqueColumns::builder()
            .column_names(value.into_iter().map(|v| v.to_string()).collect())
            .build()
    }
}

impl<T: ToString, I: IntoIterator<Item = T>> From<I> for IndexColumns {
    fn from(value: I) -> Self {
        IndexColumns::builder()
            .column_names(value.into_iter().map(|v| v.to_string()).collect())
            .build()
    }
}
