use crate::data::raw::RawData;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractData<R: RawData> {
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
    #[builder(setter(strip_option))]
    pub data: Option<R>,
}
