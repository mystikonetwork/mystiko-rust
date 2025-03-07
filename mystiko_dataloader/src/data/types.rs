use mystiko_protos::data::v1::{Commitment, Nullifier};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DataType {
    Full = 0,
    Lite = 1,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct FullData {
    #[builder(default)]
    pub commitments: Vec<Commitment>,
    #[builder(default)]
    pub nullifiers: Vec<Nullifier>,
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct LiteData {
    #[builder(default)]
    pub commitments: Vec<Commitment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Full(FullData),
    Lite(LiteData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataRef<'a> {
    Full(&'a FullData),
    Lite(&'a LiteData),
}

pub trait LoadedData: Send + Sync {
    fn data_type() -> DataType;
    fn data_ref(&self) -> DataRef;
    fn from_data(data: Data) -> Self;
    fn into_data(self) -> Data;
}

impl LoadedData for FullData {
    fn data_type() -> DataType {
        DataType::Full
    }

    fn data_ref(&self) -> DataRef {
        DataRef::Full(self)
    }

    fn from_data(data: Data) -> Self {
        match data {
            Data::Full(full_data) => full_data,
            Data::Lite(lite_data) => FullData {
                commitments: lite_data.commitments,
                nullifiers: Vec::new(),
            },
        }
    }

    fn into_data(self) -> Data {
        Data::Full(self)
    }
}

impl LoadedData for LiteData {
    fn data_type() -> DataType {
        DataType::Lite
    }

    fn data_ref(&self) -> DataRef {
        DataRef::Lite(self)
    }

    fn from_data(data: Data) -> Self {
        match data {
            Data::Full(full_data) => LiteData {
                commitments: full_data.commitments,
            },
            Data::Lite(lite_data) => lite_data,
        }
    }

    fn into_data(self) -> Data {
        Data::Lite(self)
    }
}
