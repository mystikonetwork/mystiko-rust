use crate::data::contract::ContractData;
use crate::data::raw::RawData;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainData<R: RawData> {
    pub chain_id: u64,
    #[builder(default)]
    pub contracts_data: Vec<ContractData<R>>,
}
