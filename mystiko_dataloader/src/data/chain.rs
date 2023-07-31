use crate::data::contract::{ContractData, ContractResult};
use crate::data::types::LoadedData;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainData<R: LoadedData> {
    pub chain_id: u64,
    #[builder(default)]
    pub contracts_data: Vec<ContractData<R>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ChainResult {
    pub chain_id: u64,
    #[builder(default)]
    pub contracts_result: Vec<ContractResult>,
}
