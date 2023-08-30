use crate::data::{FullData, LoadedData};
use anyhow::Result;
use ethers_core::types::Address;
use mystiko_protos::data::v1::ContractData as ProtoContractData;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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
        let address = ethers_core::utils::to_checksum(&Address::from_slice(&contract_data.contract_address), None);
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
        let address = Address::from_str(&self.address)?;
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
