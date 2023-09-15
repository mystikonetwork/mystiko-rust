use crate::data::{FullData, LoadedData};
use anyhow::Result;
use mystiko_protos::data::v1::ContractData as ProtoContractData;
use mystiko_utils::address::{ethers_address_from_string, string_address_from_bytes};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(setter(into)))]
pub struct ContractData<R: LoadedData> {
    pub address: String,
    pub start_block: u64,
    pub end_block: u64,
    #[builder(default)]
    pub data: Option<R>,
}

impl<R> ContractData<R>
where
    R: LoadedData,
{
    pub fn from_proto(start_block: u64, end_block: u64, contract_data: ProtoContractData) -> Self {
        let address = string_address_from_bytes(contract_data.contract_address);
        let data = FullData::builder()
            .commitments(contract_data.commitments)
            .nullifiers(contract_data.nullifiers)
            .build();
        Self::builder()
            .address(address)
            .start_block(start_block)
            .end_block(end_block)
            .data(R::from_data(data.into_data()))
            .build()
    }

    pub fn into_proto(self) -> Result<ProtoContractData> {
        let address = ethers_address_from_string(&self.address)?;
        if let Some(data) = self.data {
            let data = FullData::from_data(data.into_data());
            Ok(ProtoContractData::builder()
                .contract_address(address.as_bytes().to_vec())
                .commitments(data.commitments)
                .nullifiers(data.nullifiers)
                .build())
        } else {
            Ok(ProtoContractData::builder()
                .contract_address(address.as_bytes().to_vec())
                .build())
        }
    }
}
