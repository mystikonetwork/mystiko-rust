use mystiko_protos::data::v1::{Commitment, Nullifier};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DataType {
    Full = 0,
    Lite = 1,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FullData {
    pub commitments: Vec<Commitment>,
    pub nullifiers: Vec<Nullifier>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LiteData {
    pub commitments: Vec<Commitment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data<'a> {
    Full(&'a FullData),
    Lite(&'a LiteData),
}

pub trait LoadedData: Send + Sync {
    fn data_type() -> DataType;
    fn data(&self) -> Data;
}

impl LoadedData for FullData {
    fn data_type() -> DataType {
        DataType::Full
    }

    fn data(&self) -> Data {
        Data::Full(self)
    }
}

impl LoadedData for LiteData {
    fn data_type() -> DataType {
        DataType::Lite
    }

    fn data(&self) -> Data {
        Data::Lite(self)
    }
}
