use mystiko_protos::data::v1::{Commitment, Nullifier};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RawDataType {
    Full = 0,
    Lite = 1,
}

pub trait RawData: Send + Sync {
    fn data_type() -> RawDataType;
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FullRawData {
    pub commitments: Vec<Commitment>,
    pub nullifiers: Vec<Nullifier>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LiteRawData {
    pub commitments: Vec<Commitment>,
}

impl RawData for FullRawData {
    fn data_type() -> RawDataType {
        RawDataType::Full
    }
}

impl RawData for LiteRawData {
    fn data_type() -> RawDataType {
        RawDataType::Lite
    }
}
