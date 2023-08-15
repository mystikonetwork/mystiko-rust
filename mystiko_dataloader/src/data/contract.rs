use crate::data::LoadedData;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractData<R: LoadedData> {
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
    #[builder(default, setter(strip_option))]
    pub data: Option<R>,
}
